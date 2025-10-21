variable "vm_name" {
  description = "Name of the VM"
  type        = string
}

variable "description" {
  description = "VM description"
  type        = string
  default     = "Managed by Terraform"
}

variable "tags" {
  description = "List of tags for the VM"
  type        = list(string)
  default     = []
}

variable "target_node" {
  description = "Proxmox node to create the VM on"
  type        = string
}

variable "vm_id" {
  description = "VM ID (leave null for auto-assignment)"
  type        = number
  default     = null
}

variable "template_vm_id" {
  description = "VM ID of the template to clone from (null to create from ISO/image)"
  type        = number
  default     = null
}

variable "full_clone" {
  description = "Whether to create a full clone or linked clone"
  type        = bool
  default     = true
}

variable "stop_on_destroy" {
  description = "Stop VM on destroy (useful if agent is not enabled)"
  type        = bool
  default     = true
}

# Hardware Configuration
variable "cores" {
  description = "Number of CPU cores"
  type        = number
  default     = 2
}

variable "sockets" {
  description = "Number of CPU sockets"
  type        = number
  default     = 1
}

variable "cpu_type" {
  description = "CPU type (e.g., 'x86-64-v2-AES', 'host')"
  type        = string
  default     = "x86-64-v2-AES"
}

variable "memory" {
  description = "Amount of memory in MB"
  type        = number
  default     = 2048
}

variable "memory_floating" {
  description = "Floating memory in MB (set equal to dedicated to enable ballooning)"
  type        = number
  default     = null
}

# Disk Configuration
variable "disk_storage" {
  description = "Storage pool for the disk"
  type        = string
  default     = "local-lvm"
}

variable "disk_size" {
  description = "Disk size in GB (numeric value)"
  type        = number
  default     = 20
}

variable "disk_interface" {
  description = "Disk interface (e.g., 'scsi0', 'virtio0')"
  type        = string
  default     = "scsi0"
}

variable "disk_file_format" {
  description = "Disk file format (e.g., 'qcow2', 'raw')"
  type        = string
  default     = "qcow2"
}

variable "disk_import_from" {
  description = "Import disk from image file ID (e.g., 'local:import/debian-13-genericcloud-amd64.img') - only used when not cloning from template"
  type        = string
  default     = null
}

# Network Configuration
variable "network_model" {
  description = "Network card model (e.g., 'virtio', 'e1000')"
  type        = string
  default     = "virtio"
}

variable "network_bridge" {
  description = "Network bridge to use"
  type        = string
  default     = "vmbr0"
}

# IP Configuration
variable "use_dhcp" {
  description = "Use DHCP for IP configuration"
  type        = bool
  default     = true
}

variable "ip_address" {
  description = "Static IP address (if not using DHCP)"
  type        = string
  default     = ""
}

variable "ip_cidr" {
  description = "CIDR notation for IP (e.g., 24)"
  type        = string
  default     = "24"
}

variable "gateway" {
  description = "Gateway IP address"
  type        = string
  default     = ""
}

# Cloud-init Configuration
variable "cloud_init_datastore_id" {
  description = "Datastore for cloud-init disk"
  type        = string
  default     = null
}

variable "cloud_init_user" {
  description = "Cloud-init default user"
  type        = string
  default     = "root"
}

variable "cloud_init_password" {
  description = "Cloud-init user password"
  type        = string
  default     = null
  sensitive   = true
}

variable "ssh_keys" {
  description = "List of SSH public keys"
  type        = list(string)
  default     = []
}

variable "cloud_init_user_data_file_id" {
  description = "Cloud-init user data file ID"
  type        = string
  default     = null
}

# Operating System
variable "os_type" {
  description = "Operating system type (e.g., 'l26' for Linux 2.6+, 'win11' for Windows)"
  type        = string
  default     = null
}

# Additional Settings
variable "started" {
  description = "Whether to start the VM after creation"
  type        = bool
  default     = true
}

variable "onboot" {
  description = "Start VM on boot"
  type        = bool
  default     = true
}

variable "qemu_agent_enabled" {
  description = "Enable QEMU guest agent"
  type        = bool
  default     = true
}
