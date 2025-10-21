variable "vms" {
  description = "Map of VMs to create. Key is a unique identifier, value is VM configuration."
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
}

# Default values for all VMs
variable "default_target_node" {
  description = "Default Proxmox node to create VMs on"
  type        = string
}

variable "default_description" {
  description = "Default VM description"
  type        = string
  default     = "Managed by Terraform"
}

variable "default_tags" {
  description = "Default tags for VMs"
  type        = list(string)
  default     = []
}

variable "default_template_vm_id" {
  description = "Default template VM ID to clone from (null to create from ISO)"
  type        = number
  default     = null
}

variable "default_stop_on_destroy" {
  description = "Default stop on destroy setting"
  type        = bool
  default     = true
}

variable "default_full_clone" {
  description = "Default full clone setting"
  type        = bool
  default     = true
}

variable "default_cores" {
  description = "Default number of CPU cores"
  type        = number
  default     = 2
}

variable "default_sockets" {
  description = "Default number of CPU sockets"
  type        = number
  default     = 1
}

variable "default_cpu_type" {
  description = "Default CPU type"
  type        = string
  default     = "x86-64-v2-AES"
}

variable "default_memory" {
  description = "Default amount of memory in MB"
  type        = number
  default     = 2048
}

variable "default_memory_floating" {
  description = "Default floating memory in MB (set equal to dedicated to enable ballooning)"
  type        = number
  default     = null
}

variable "default_disk_storage" {
  description = "Default storage pool for disks"
  type        = string
  default     = "local-lvm"
}

variable "default_disk_size" {
  description = "Default disk size in GB"
  type        = number
  default     = 20
}

variable "default_disk_interface" {
  description = "Default disk interface"
  type        = string
  default     = "scsi0"
}

variable "default_disk_file_format" {
  description = "Default disk file format"
  type        = string
  default     = "qcow2"
}

variable "default_network_model" {
  description = "Default network card model"
  type        = string
  default     = "virtio"
}

variable "default_network_bridge" {
  description = "Default network bridge"
  type        = string
  default     = "vmbr0"
}

variable "default_use_dhcp" {
  description = "Default DHCP setting"
  type        = bool
  default     = true
}

variable "default_ip_cidr" {
  description = "Default IP CIDR notation"
  type        = string
  default     = "24"
}

variable "default_gateway" {
  description = "Default gateway IP"
  type        = string
  default     = ""
}

variable "default_cloud_init_datastore_id" {
  description = "Default datastore for cloud-init disk"
  type        = string
  default     = null
}

variable "default_cloud_init_user" {
  description = "Default cloud-init user"
  type        = string
  default     = "root"
}

variable "default_cloud_init_password" {
  description = "Default cloud-init user password"
  type        = string
  default     = null
  sensitive   = true
}

variable "default_ssh_keys" {
  description = "Default SSH keys"
  type        = list(string)
  default     = []
}

variable "default_os_type" {
  description = "Default OS type (e.g., 'l26' for Linux)"
  type        = string
  default     = null
}

variable "default_started" {
  description = "Default started setting"
  type        = bool
  default     = true
}

variable "default_onboot" {
  description = "Default onboot setting"
  type        = bool
  default     = true
}

variable "default_qemu_agent_enabled" {
  description = "Default QEMU agent enabled setting"
  type        = bool
  default     = true
}
