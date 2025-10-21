# Proxmox VM Module

Simple Terraform module for creating a single VM on Proxmox.

## Features

- Create a single VM from a template
- Simple hardware configuration (CPU, memory, disk)
- Basic network configuration
- Cloud-init support with DHCP or static IP
- QEMU guest agent support

## Usage

### Basic VM with DHCP

```hcl
module "vm" {
  source = "./modules/proxmox/vm"

  vm_name        = "my-vm"
  target_node    = "pve"
  template_vm_id = 9000  # VM ID of your template
}
```

### VM with Custom Resources

```hcl
module "web_server" {
  source = "./modules/proxmox/vm"

  vm_name        = "web-server"
  target_node    = "pve"
  template_vm_id = 9000

  # Hardware
  cores  = 4
  memory = 4096

  # Disk
  disk_storage = "local-lvm"
  disk_size    = 50  # GB
}
```

### VM with Static IP and SSH Keys

```hcl
module "db_server" {
  source = "./modules/proxmox/vm"

  vm_name        = "db-server"
  target_node    = "pve"
  template_vm_id = 9000

  # Network with static IP
  use_dhcp   = false
  ip_address = "192.168.1.100"
  ip_cidr    = "24"
  gateway    = "192.168.1.1"

  # Cloud-init
  ssh_keys = [file("~/.ssh/id_rsa.pub")]
}
```

### VM from ISO/Cloud Image

Create a VM from a cloud image or ISO instead of cloning:

```hcl
resource "proxmox_virtual_environment_download_file" "ubuntu_image" {
  content_type = "iso"
  datastore_id = "local"
  node_name    = "pve"
  url          = "https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img"
  file_name    = "jammy-server-cloudimg-amd64.img"
}

module "ubuntu_vm" {
  source = "./modules/proxmox/vm"

  vm_name          = "ubuntu-from-image"
  target_node      = "pve"
  template_vm_id   = null  # Don't clone, create from image

  # Import from downloaded image
  disk_import_from = proxmox_virtual_environment_download_file.ubuntu_image.id

  # OS type for non-template installs
  os_type = "l26"  # Linux 2.6+

  # Hardware
  cores  = 2
  memory = 2048

  # Cloud-init
  ssh_keys = [file("~/.ssh/id_rsa.pub")]

  tags = ["terraform", "ubuntu"]
}
```

### Multiple VMs

```hcl
module "vm1" {
  source = "./modules/proxmox/vm"

  vm_name        = "node-1"
  target_node    = "pve"
  template_vm_id = 9000
}

module "vm2" {
  source = "./modules/proxmox/vm"

  vm_name        = "node-2"
  target_node    = "pve"
  template_vm_id = 9000
}
```

## Requirements

- Proxmox provider (bpg/proxmox) configured in your main Terraform configuration
- A VM template available in Proxmox with a known VM ID
- Appropriate permissions on the Proxmox node
- Cloud-init enabled on the template (recommended)

## Inputs

| Name | Description | Type | Default | Required |
|------|-------------|------|---------|:--------:|
| vm_name | Name of the VM | string | - | yes |
| target_node | Proxmox node to create the VM on | string | - | yes |
| template_vm_id | VM ID of the template to clone from | number | - | yes |
| vm_id | VM ID (null for auto-assignment) | number | null | no |
| full_clone | Whether to create a full clone or linked clone | bool | true | no |
| cores | Number of CPU cores | number | 2 | no |
| sockets | Number of CPU sockets | number | 1 | no |
| cpu_type | CPU type (e.g., 'x86-64-v2-AES') | string | "x86-64-v2-AES" | no |
| memory | Amount of memory in MB | number | 2048 | no |
| disk_storage | Storage pool for the disk | string | "local-lvm" | no |
| disk_size | Disk size in GB | number | 20 | no |
| disk_interface | Disk interface (e.g., 'scsi0') | string | "scsi0" | no |
| network_model | Network card model | string | "virtio" | no |
| network_bridge | Network bridge to use | string | "vmbr0" | no |
| use_dhcp | Use DHCP for IP configuration | bool | true | no |
| ip_address | Static IP address (if not using DHCP) | string | "" | no |
| ip_cidr | CIDR notation for IP (e.g., 24) | string | "24" | no |
| gateway | Gateway IP address | string | "" | no |
| cloud_init_user | Cloud-init default user | string | "ubuntu" | no |
| ssh_keys | List of SSH public keys | list(string) | [] | no |
| started | Whether to start the VM after creation | bool | true | no |
| onboot | Start VM on boot | bool | false | no |
| qemu_agent_enabled | Enable QEMU guest agent | bool | true | no |

## Outputs

| Name | Description |
|------|-------------|
| vm_id | VM ID |
| vm_name | VM name |
| vm_ipv4_address | VM IPv4 address |
| vm_ssh_host | SSH host address |
| vm_ssh_port | SSH port |
