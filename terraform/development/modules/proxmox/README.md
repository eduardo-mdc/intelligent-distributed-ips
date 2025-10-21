# Proxmox Terraform Modules

Collection of Terraform modules for managing Proxmox Virtual Environment resources using the [bpg/proxmox](https://registry.terraform.io/providers/bpg/proxmox/latest) provider.

## Features

- **Flexible VM Creation**: Clone from templates OR create from ISO/cloud images
- **Map-based Configuration**: Define multiple VMs in a single configuration map
- **Inheritance with Overrides**: Set defaults for all VMs, override per-VM as needed
- **Cloud-init Support**: Automated VM initialization with SSH keys and network config
- **Full Resource Control**: CPU, memory, disk, network, and more

## Parent Module Usage

The parent proxmox module accepts a map of VM configurations and creates multiple VMs by calling the vm sub-module. Each VM can use different settings, or inherit from module-level defaults.

### Basic Usage

```hcl
module "proxmox" {
  source = "./modules/proxmox"

  default_target_node    = "pve"
  default_template_vm_id = 9000  # VM ID of your template

  vms = {
    web = {
      vm_name = "web-server"
      cores   = 4
      memory  = 4096
    }
    db = {
      vm_name   = "db-server"
      cores     = 4
      memory    = 8192
      disk_size = 100  # GB
    }
    worker = {
      vm_name = "worker-node"
    }
  }
}
```

### With Static IPs and SSH Keys

```hcl
module "proxmox" {
  source = "./modules/proxmox"

  default_target_node    = "pve"
  default_template_vm_id = 9000
  default_use_dhcp       = false
  default_gateway        = "192.168.1.1"
  default_ssh_keys       = [file("~/.ssh/id_rsa.pub")]

  vms = {
    web = {
      vm_name    = "web-server"
      ip_address = "192.168.1.100"
    }
    db = {
      vm_name    = "db-server"
      ip_address = "192.168.1.101"
    }
  }
}
```

### Creating VMs from ISO/Cloud Images

Instead of cloning from a template, you can create VMs by importing from ISO or cloud images:

```hcl
# First, define the image download resource
resource "proxmox_virtual_environment_download_file" "ubuntu_cloud_image" {
  content_type = "iso"
  datastore_id = "local"
  node_name    = "pve"
  url          = "https://cloud-images.ubuntu.com/jammy/current/jammy-server-cloudimg-amd64.img"
  file_name    = "jammy-server-cloudimg-amd64.img"
}

module "proxmox" {
  source = "./modules/proxmox"

  default_target_node = "pve"
  default_template_vm_id = null  # No template, create from image
  default_ssh_keys = [file("~/.ssh/id_rsa.pub")]
  default_tags = ["terraform", "ubuntu"]

  vms = {
    ubuntu_vm = {
      vm_name          = "ubuntu-from-image"
      disk_import_from = proxmox_virtual_environment_download_file.ubuntu_cloud_image.id
      os_type          = "l26"  # Linux 2.6+
      cores            = 2
      memory           = 2048
    }
  }
}
```

### Accessing Outputs

```hcl
output "all_vms" {
  value = module.proxmox.vms
}

output "web_server_ip" {
  value = module.proxmox.vm_ipv4_addresses["web"]
}
```

## Available Modules

### VM Module (`vm/`)

Sub-module for creating individual VMs on Proxmox. Can be used directly or through the parent module.

**Features:**
- Clone from VM templates OR create from ISO/cloud images
- Full hardware configuration (CPU, memory, disk)
- Network configuration with DHCP or static IP
- Cloud-init support with SSH keys and passwords
- Metadata support (description, tags)

**Direct Usage:**
```hcl
module "my_vm" {
  source = "./modules/proxmox/vm"

  vm_name        = "test-vm"
  target_node    = "pve"
  template_vm_id = 9000  # VM ID of template

  ssh_keys = [file("~/.ssh/id_rsa.pub")]
}
```

See [vm/README.md](vm/README.md) for detailed documentation.

## Requirements

### Proxmox Provider

This module requires the **bpg/proxmox** provider (version ~> 0.45+):

```hcl
terraform {
  required_providers {
    proxmox = {
      source  = "bpg/proxmox"
      version = "~> 0.45"
    }
  }
}
```

### Provider Authentication

Configure the provider in your root module using ONE of these authentication methods:

**Option 1: API Token (Recommended for production)**
```hcl
provider "proxmox" {
  endpoint  = "https://proxmox.example.com:8006"
  api_token = "user@realm!tokenid=secret-value"
  insecure  = true  # Only for self-signed certificates

  ssh {
    agent = true  # Use SSH agent for file operations
  }
}
```

**Option 2: Username/Password (Development)**
```hcl
provider "proxmox" {
  endpoint = "https://proxmox.example.com:8006"
  username = "terraform@pam"
  password = var.proxmox_password
  insecure = true

  ssh {
    agent = true
  }
}
```

**Option 3: Environment Variables (Recommended)**
```bash
export PROXMOX_VE_ENDPOINT="https://proxmox.example.com:8006"
export PROXMOX_VE_API_TOKEN="user@realm!tokenid=secret-value"
# Or use username/password
export PROXMOX_VE_USERNAME="terraform@pam"
export PROXMOX_VE_PASSWORD="your-password"
```

### Other Requirements

- Access to a Proxmox VE cluster (version 7.0+)
- VM templates (for cloning) OR cloud images (for ISO install)
- Appropriate permissions for the Terraform user/token

## Module Structure

```
modules/proxmox/
├── provider.tf       # Provider version requirements
├── main.tf           # Parent module logic
├── variables.tf      # Input variables
├── outputs.tf        # Output values
├── README.md         # This file
└── vm/               # VM sub-module
    ├── provider.tf   # Sub-module provider requirements
    ├── main.tf       # VM resource definition
    ├── variables.tf  # VM-specific variables
    ├── outputs.tf    # VM outputs
    └── README.md     # VM module documentation
```

## Future Enhancements

Additional sub-modules can be added:
- **Container module**: Manage LXC containers
- **Storage module**: Manage storage pools and volumes
- **Network module**: Manage virtual networks and bridges
- **Download module**: Manage ISO/image downloads
