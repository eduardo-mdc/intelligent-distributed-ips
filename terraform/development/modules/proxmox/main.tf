module "vms" {
  source = "./vm"

  for_each = var.vms

  # Required variables
  vm_name     = each.value.vm_name
  target_node = coalesce(each.value.target_node, var.default_target_node)

  # Optional metadata
  description = coalesce(each.value.description, var.default_description)
  tags        = coalesce(each.value.tags, var.default_tags)

  # Optional VM ID
  vm_id = each.value.vm_id

  # Clone or ISO/image install
  template_vm_id = each.value.template_vm_id != null ? each.value.template_vm_id : var.default_template_vm_id
  full_clone     = coalesce(each.value.full_clone, var.default_full_clone)

  # Stop behavior
  stop_on_destroy = coalesce(each.value.stop_on_destroy, var.default_stop_on_destroy)

  # Hardware configuration
  cores           = coalesce(each.value.cores, var.default_cores)
  sockets         = coalesce(each.value.sockets, var.default_sockets)
  cpu_type        = coalesce(each.value.cpu_type, var.default_cpu_type)
  memory          = coalesce(each.value.memory, var.default_memory)
  memory_floating = each.value.memory_floating != null ? each.value.memory_floating : var.default_memory_floating

  # Disk configuration
  disk_storage     = coalesce(each.value.disk_storage, var.default_disk_storage)
  disk_size        = coalesce(each.value.disk_size, var.default_disk_size)
  disk_interface   = coalesce(each.value.disk_interface, var.default_disk_interface)
  disk_file_format = coalesce(each.value.disk_file_format, var.default_disk_file_format)
  disk_import_from = each.value.disk_import_from

  # Network configuration
  network_model  = coalesce(each.value.network_model, var.default_network_model)
  network_bridge = coalesce(each.value.network_bridge, var.default_network_bridge)

  # IP configuration
  use_dhcp   = coalesce(each.value.use_dhcp, var.default_use_dhcp)
  ip_address = each.value.ip_address != null ? each.value.ip_address : ""
  ip_cidr    = coalesce(each.value.ip_cidr, var.default_ip_cidr)
  gateway    = each.value.gateway != null ? each.value.gateway : var.default_gateway

  # Cloud-init configuration
  cloud_init_datastore_id      = each.value.cloud_init_datastore_id != null ? each.value.cloud_init_datastore_id : var.default_cloud_init_datastore_id
  cloud_init_user              = coalesce(each.value.cloud_init_user, var.default_cloud_init_user)
  cloud_init_password          = each.value.cloud_init_password != null ? each.value.cloud_init_password : var.default_cloud_init_password
  ssh_keys                     = coalesce(each.value.ssh_keys, var.default_ssh_keys)
  cloud_init_user_data_file_id = each.value.cloud_init_user_data_file_id

  # Operating system
  os_type = each.value.os_type != null ? each.value.os_type : var.default_os_type

  # Additional settings
  started            = coalesce(each.value.started, var.default_started)
  onboot             = coalesce(each.value.onboot, var.default_onboot)
  qemu_agent_enabled = coalesce(each.value.qemu_agent_enabled, var.default_qemu_agent_enabled)
}
