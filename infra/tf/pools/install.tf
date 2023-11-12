resource "null_resource" "install" {
	for_each = nonsensitive(var.server_install_scripts)

	triggers = {
		install_script = md5(each.value)
	}

	connection {
		type = "ssh"
		host = module.servers[each.key].host
		user = module.servers[each.key].user
		private_key = module.servers[each.key].private_key_openssh
	}

	provisioner "remote-exec" {
		inline = [
			each.value
		]
	}
}
