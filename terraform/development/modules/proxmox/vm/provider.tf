terraform {
  required_version = ">= 1.0"

  required_providers {
    proxmox = {
      source  = "bpg/proxmox"
      version = "~> 0.45"
    }
  }
}

# Provider configuration is inherited from the parent/root module
# This file only declares the required provider version for this sub-module
