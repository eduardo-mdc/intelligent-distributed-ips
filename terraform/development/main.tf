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
  ssh {
    agent = true
    # Alternatively, specify credentials:
    # username = var.proxmox_ssh_username
    # password = var.proxmox_ssh_password
    # private_key = file("~/.ssh/id_rsa")
  }
}

# Proxmox VMs Module
module "proxmox_vms" {
  source = "./modules/proxmox"

  # Default settings for all VMs
  default_target_node    = var.default_proxmox_node
  default_template_vm_id = var.default_template_vm_id
  default_ssh_keys       = var.default_ssh_keys
  default_tags           = ["terraform","proxmox","vms"]

  # VM configurations
  vms = var.vms
}
