# VM IDs
output "vm_ids" {
  description = "Map of VM IDs by VM key"
  value       = module.proxmox_vms.vm_ids
}

# VM Names
output "vm_names" {
  description = "Map of VM names by VM key"
  value       = module.proxmox_vms.vm_names
}

# VM IP Addresses
output "vm_ipv4_addresses" {
  description = "Map of VM IPv4 addresses by VM key"
  value       = module.proxmox_vms.vm_ipv4_addresses
}

# VM SSH Hosts
output "vm_ssh_hosts" {
  description = "Map of VM SSH host addresses by VM key"
  value       = module.proxmox_vms.vm_ssh_hosts
}

# All VM Information
output "vms" {
  description = "Complete map of all VM information"
  value       = module.proxmox_vms.vms
}
