locals {
	cockroachdb_k8s = var.cockroachdb_provider == "kubernetes"
	service_cockroachdb = lookup(var.services, "cockroachdb", {
		count = 1
		resources = {
			cpu = 1000
			memory = 2048
		}
	})
}

module "crdb_secrets" {
	count = local.cockroachdb_k8s ? 1 : 0

	source = "../modules/secrets"

	keys = [ "crdb/username", "crdb/password" ]
}

resource "kubernetes_namespace" "cockroachdb" {
	count = local.cockroachdb_k8s ? 1 : 0

	metadata {
		name = "cockroachdb"
	}
}

resource "kubernetes_priority_class" "cockroachdb_priority" {
	metadata {
		name = "cockroachdb-priority"
	}

	value = 40
}

# NOTE: Helm chart is no longer supported by CockroachDB. However, it's intended to be used only for development and it's the easiest to set up.
resource "helm_release" "cockroachdb" {
	depends_on = [helm_release.prometheus]
	count = local.cockroachdb_k8s ? 1 : 0

	name = "cockroachdb"
	namespace = kubernetes_namespace.cockroachdb[0].metadata.0.name
	repository = "https://charts.cockroachdb.com/"
	chart = "cockroachdb"
	version = "11.1.5"  # v23.1.9
	values = [yamlencode({
		statefulset = {
			replicas = local.service_cockroachdb.count

			priorityClassName = kubernetes_priority_class.cockroachdb_priority.metadata.0.name

			resources = var.limit_resources ? {
				limits = {
					memory = "${local.service_cockroachdb.resources.memory}Mi"
					cpu = "${local.service_cockroachdb.resources.cpu}m"
				}
			} : null
		}
		conf = {
			single-node = true
		}
		tls = {
			enabled = true
		}
		storage = {
			persistentVolume = {
				storageClass = var.k8s_storage_class
			}
		}
		init = {
			provisioning = {
				enabled = true
				users = [
					{
						name = module.crdb_secrets[0].values["crdb/username"]
						password = module.crdb_secrets[0].values["crdb/password"]
						options = ["CREATEDB"]
					}
				]
			}
		}

		serviceMonitor = {
			# TODO: Doesn't work without insecure TLS
			enabled = false
			namespaced = true

			# tlsConfig = {
			# 	insecureSkipVerify = true
			# }
		}
	})]
}

data "kubernetes_secret" "crdb_ca" {
	count = local.cockroachdb_k8s ? 1 : 0

	depends_on = [helm_release.cockroachdb]

	metadata {
		name = "cockroachdb-ca-secret"
		namespace = kubernetes_namespace.cockroachdb[0].metadata.0.name
	}
}

resource "kubernetes_config_map" "crdb_ca" {
	for_each = local.cockroachdb_k8s ? toset(["rivet-service", "bolt"]) : toset([])

	metadata {
		name = "crdb-ca"
		namespace = each.value
	}

	data = {
		"ca.crt" = data.kubernetes_secret.crdb_ca[0].data["ca.crt"]
	}
}
