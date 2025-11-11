terraform {
  required_version = ">= 1.0"

  required_providers {
    proxmox = {
      source  = "bpg/proxmox"
      version = "~> 0.45"
    }
  }
}

# Proxmox Provider Configuration
provider "proxmox" {
  endpoint = var.proxmox_endpoint
  insecure = var.proxmox_insecure

  # Authentication - using API token (recommended)
  api_token = var.proxmox_api_token

  # Alternatively, use username/password:
  # username = var.proxmox_username
  # password = var.proxmox_password

  # SSH configuration for file operations
  # Do: eval $(ssh-agent) & ssh-add ~/.ssh/id_rsa
  # Before running Terraform to enable SSH agent forwarding
  ssh {
    agent = true
    username = "root"
  }
}

# Proxmox VMs Module
module "proxmox_vms" {
  source = "./modules/proxmox"

  # Default settings for all VMs
  default_target_node         = var.default_proxmox_node
  default_template_vm_id      = var.default_template_vm_id
  default_ssh_keys            = var.default_ssh_keys
  default_cloud_init_password = var.default_cloud_init_password
  default_tags                = ["terraform", "proxmox", "vms"]

  # VM configurations
  vms = var.vms
}
