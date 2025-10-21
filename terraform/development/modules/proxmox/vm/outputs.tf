output "vm_id" {
  description = "VM ID"
  value       = proxmox_virtual_environment_vm.vm.id
}

output "vm_name" {
  description = "VM name"
  value       = proxmox_virtual_environment_vm.vm.name
}

output "vm_ipv4_address" {
  description = "VM IPv4 address"
  value       = try(proxmox_virtual_environment_vm.vm.ipv4_addresses[1][0], null)
}

output "vm_ssh_host" {
  description = "SSH host address"
  value       = try(proxmox_virtual_environment_vm.vm.ipv4_addresses[1][0], null)
}

output "vm_ssh_port" {
  description = "SSH port"
  value       = 22
}
