# TODO: Wait until fargate is up
module "karpenter" {
	source = "terraform-aws-modules/eks/aws//modules/karpenter"
	version = "19.16.0"

	cluster_name = module.eks.cluster_name
	irsa_oidc_provider_arn = module.eks.oidc_provider_arn

	policies = {
		AmazonSSMManagedInstanceCore = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
	}

	tags = local.tags
}

resource "helm_release" "karpenter" {
	namespace = "karpenter"
	create_namespace = true

	name = "karpenter"
	repository = "oci://public.ecr.aws/karpenter"
	chart = "karpenter"
	version = "v0.31.0"

	values = [yamlencode({
		controller = {
			resources = {
				# Must be compatible with Fargate pod sizes:
				# 
				# https://docs.aws.amazon.com/eks/latest/userguide/fargate-pod-configuration.html
				limits = {
					cpu = "0.5"
					memory = "512M"
				}
			}
		}

		serviceAccount = {
			annotations = {
				"eks.amazonaws.com/role-arn" = module.karpenter.irsa_arn
			}
		}

		settings = {
			aws = {
				clusterName = module.eks.cluster_name
				clusterEndpoint = module.eks.cluster_endpoint
				defaultInstanceProfile = module.karpenter.instance_profile_name
				interruptionQueueName = module.karpenter.queue_name
			}
		}
	})]
}

resource "kubectl_manifest" "karpenter_provisioner" {
	depends_on = [helm_release.karpenter]

	yaml_body = yamlencode({
		apiVersion = "karpenter.sh/v1alpha5"
		kind = "Provisioner"
		metadata = {
			name = "default"
		}
		spec = {
			requirements = [
				# See how Karpenter selects instance types:
				# https://karpenter.sh/v0.31/faq/#how-does-karpenter-dynamically-select-instance-types

				{
					key = "kubernetes.io/os"
					operator = "In"
					values = ["linux"]
				},
				{
					key = "topology.kubernetes.io/zone"
					operator = "In"
					values = local.azs
				},
				{
					key = "karpenter.sh/capacity-type"
					operator = "In"
					values = ["on-demand"]
				},
			]
			limits = {
				resources = {
					cpu = 1000
					memory = "1000Gi"
				}
			}
			providerRef = {
				name = "default"
			}
			consolidation = {
				enabled = true
			}
		}
	})
}

resource "kubectl_manifest" "karpenter_node_template" {
	depends_on = [helm_release.karpenter]

	yaml_body = yamlencode({
		apiVersion = "karpenter.k8s.aws/v1alpha1"
		kind = "AWSNodeTemplate"
		metadata = {
			name = "default"
		}
		spec = {
			subnetSelector = {
				"karpenter.sh/discovery" = module.eks.cluster_name
			}
			securityGroupSelector = {
				"karpenter.sh/discovery" = module.eks.cluster_name
			}
			tags = {
				"karpenter.sh/discovery" = module.eks.cluster_name
			}
		}
	})
}
