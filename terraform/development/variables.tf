# Proxmox Provider Configuration
variable "proxmox_endpoint" {
  description = "Proxmox API endpoint (e.g., https://proxmox.example.com:8006)"
  type        = string
}

variable "proxmox_insecure" {
  description = "Skip TLS verification (use for self-signed certificates)"
  type        = bool
  default     = true
}

# Authentication - API Token (recommended)
variable "proxmox_api_token" {
  description = "Proxmox API token (format: user@realm!tokenid=secret)"
  type        = string
  sensitive   = true
  default     = null
}

# Authentication - Username/Password (alternative)
variable "proxmox_username" {
  description = "Proxmox username (format: user@realm)"
  type        = string
  default     = null
}

variable "proxmox_password" {
  description = "Proxmox password"
  type        = string
  sensitive   = true
  default     = null
}

# SSH Configuratio

# VM Module Defaults
variable "default_proxmox_node" {
  description = "Default Proxmox node name"
  type        = string
}

variable "default_template_vm_id" {
  description = "Default template VM ID to clone from (null to create from image file)"
  type        = number
  default     = null
}

variable "default_ssh_keys" {
  description = "Default SSH public keys for VM access"
  type        = list(string)
  default     = []
}

# VM Configurations
variable "vms" {
  description = "Map of VM configurations"
  type = map(object({
    vm_name                      = string
    vm_id                        = optional(number)
    description                  = optional(string)
    tags                         = optional(list(string))
    target_node                  = optional(string)
    template_vm_id               = optional(number)
    full_clone                   = optional(bool)
    stop_on_destroy              = optional(bool)
    cores                        = optional(number)
    sockets                      = optional(number)
    cpu_type                     = optional(string)
    memory                       = optional(number)
    memory_floating              = optional(number)
    disk_storage                 = optional(string)
    disk_size                    = optional(number)
    disk_interface               = optional(string)
    disk_file_format             = optional(string)
    disk_import_from             = optional(string)
    network_model                = optional(string)
    network_bridge               = optional(string)
    use_dhcp                     = optional(bool)
    ip_address                   = optional(string)
    ip_cidr                      = optional(string)
    gateway                      = optional(string)
    cloud_init_datastore_id      = optional(string)
    cloud_init_user              = optional(string)
    cloud_init_password          = optional(string)
    ssh_keys                     = optional(list(string))
    cloud_init_user_data_file_id = optional(string)
    os_type                      = optional(string)
    started                      = optional(bool)
    onboot                       = optional(bool)
    qemu_agent_enabled           = optional(bool)
  }))
  default = {}
}
