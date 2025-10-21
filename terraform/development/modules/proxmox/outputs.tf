output "vm_ids" {
  description = "Map of VM IDs"
  value       = { for k, v in module.vms : k => v.vm_id }
}

output "vm_names" {
  description = "Map of VM names"
  value       = { for k, v in module.vms : k => v.vm_name }
}

output "vm_ipv4_addresses" {
  description = "Map of VM IPv4 addresses"
  value       = { for k, v in module.vms : k => v.vm_ipv4_address }
}

output "vm_ssh_hosts" {
  description = "Map of VM SSH hosts"
  value       = { for k, v in module.vms : k => v.vm_ssh_host }
}

output "vm_ssh_ports" {
  description = "Map of VM SSH ports"
  value       = { for k, v in module.vms : k => v.vm_ssh_port }
}

output "vms" {
  description = "Complete map of all VM outputs"
  value = {
    for k, v in module.vms : k => {
      id              = v.vm_id
      name            = v.vm_name
      ipv4_address    = v.vm_ipv4_address
      ssh_host        = v.vm_ssh_host
      ssh_port        = v.vm_ssh_port
    }
  }
}
