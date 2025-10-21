terraform {
  required_version = ">= 1.0"

  required_providers {
    proxmox = {
      source  = "bpg/proxmox"
      version = "~> 0.45"
    }
  }
}

# Provider configuration is typically defined in the root module
# This file only declares the required provider version
#
# Example provider configuration for your root main.tf:
#
# provider "proxmox" {
#   endpoint = var.proxmox_endpoint  # e.g., "https://proxmox.example.com:8006"
#   insecure = var.proxmox_insecure  # Skip TLS verification (dev only)
#
#   # Authentication: Use ONE of these methods
#
#   # Option 1: API Token (Recommended for production)
#   api_token = var.proxmox_api_token  # Format: "user@realm!tokenid=secret"
#
#   # Option 2: Username/Password (Development)
#   # username = var.proxmox_username  # Format: "user@realm"
#   # password = var.proxmox_password
#
#   # Option 3: Auth Ticket + CSRF Token (Automated scripts)
#   # auth_ticket = var.proxmox_auth_ticket
#   # csrf_prevention_token = var.proxmox_csrf_token
#
#   # Optional: SSH configuration for file uploads and downloads
#   ssh {
#     agent = true  # Use SSH agent
#     # Alternatively, specify username and password or private key
#     # username = var.proxmox_ssh_username
#     # private_key = file("~/.ssh/id_rsa")
#   }
# }
#
# Environment variables can be used instead:
# - PROXMOX_VE_ENDPOINT
# - PROXMOX_VE_API_TOKEN
# - PROXMOX_VE_USERNAME
# - PROXMOX_VE_PASSWORD
# - PROXMOX_VE_INSECURE
