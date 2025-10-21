output "vm_id" {
  description = "VM ID"
  value       = proxmox_vm_qemu.vm.id
}

output "vm_name" {
  description = "VM name"
  value       = proxmox_vm_qemu.vm.name
}

output "vm_ipv4_address" {
  description = "VM IPv4 address"
  value       = proxmox_vm_qemu.vm.default_ipv4_address
}

output "vm_ssh_host" {
  description = "SSH host address"
  value       = proxmox_vm_qemu.vm.ssh_host
}

output "vm_ssh_port" {
  description = "SSH port"
  value       = proxmox_vm_qemu.vm.ssh_port
}
