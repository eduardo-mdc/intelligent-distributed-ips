resource "proxmox_virtual_environment_vm" "vm" {
  name        = var.vm_name
  description = var.description
  tags        = var.tags
  node_name   = var.target_node
  vm_id       = var.vm_id

  # Clone from template (only if template_vm_id is provided)
  dynamic "clone" {
    for_each = var.template_vm_id != null ? [1] : []
    content {
      vm_id        = var.template_vm_id
      full         = var.full_clone
      datastore_id = var.disk_storage
    }
  }

  # Stop behavior
  stop_on_destroy = var.stop_on_destroy

  # CPU Configuration
  cpu {
    cores   = var.cores
    sockets = var.sockets
    type    = var.cpu_type
  }

  # Memory Configuration
  memory {
    dedicated = var.memory
    floating  = var.memory_floating
  }

  # Disk Configuration
  disk {
    datastore_id = var.disk_storage
    size         = var.disk_size
    interface    = var.disk_interface
    file_format  = var.disk_file_format
    # Import from ISO/image (only if import_from is provided and not cloning)
    import_from = var.template_vm_id == null ? var.disk_import_from : null
  }

  # Network Configuration
  network_device {
    bridge = var.network_bridge
    model  = var.network_model
  }

  # Operating System type (useful for ISO installs)
  dynamic "operating_system" {
    for_each = var.os_type != null ? [1] : []
    content {
      type = var.os_type
    }
  }

  # Cloud-init Configuration
  initialization {
    datastore_id = var.cloud_init_datastore_id

    ip_config {
      ipv4 {
        address = var.use_dhcp ? "dhcp" : "${var.ip_address}/${var.ip_cidr}"
        gateway = var.use_dhcp ? null : var.gateway
      }
    }

    user_account {
      username = var.cloud_init_user
      password = var.cloud_init_password
      keys     = var.ssh_keys
    }

    user_data_file_id = var.cloud_init_user_data_file_id
  }

  # Additional settings
  started = var.started
  on_boot = var.onboot

  agent {
    enabled = var.qemu_agent_enabled
  }

  lifecycle {
    ignore_changes = [
      network_device,
    ]
  }
}
