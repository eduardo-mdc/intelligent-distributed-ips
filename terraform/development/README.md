# Terraform Infrastructure - Development Environment

Terraform configuration for deploying Proxmox VMs using the [bpg/proxmox](https://registry.terraform.io/providers/bpg/proxmox/latest) provider.

## Quick Start

1. **Configure variables**:
   ```bash
   cp terraform.tfvars.example terraform.tfvars
   # Edit terraform.tfvars with your settings
   ```

2. **Initialize and apply**:
   ```bash
   terraform init
   terraform plan
   terraform apply
   ```

## Authentication

Set in `terraform.tfvars`:
```hcl
proxmox_endpoint  = "https://proxmox.example.com:8006"
proxmox_api_token = "terraform@pam!mytoken=secret-value"
```

Or use environment variables:
```bash
export PROXMOX_VE_ENDPOINT="https://proxmox.example.com:8006"
export PROXMOX_VE_API_TOKEN="terraform@pam!mytoken=secret"
```

## Example Configuration

```hcl
default_proxmox_node   = "pve"
default_template_vm_id = 9000
default_ssh_keys       = ["ssh-rsa AAAAB3... your-key"]

vms = {
  web = {
    vm_name    = "web-server"
    cores      = 4
    memory     = 4096
    ip_address = "192.168.1.100"
    use_dhcp   = false
  }
}
```

## Module Documentation

- [Proxmox Module](./modules/proxmox/README.md)
- [VM Sub-module](./modules/proxmox/vm/README.md)
