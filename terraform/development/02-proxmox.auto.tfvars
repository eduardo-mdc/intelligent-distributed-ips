
# Default VM Settings
default_proxmox_node   = "pve"
default_template_vm_id = 9000 # VM ID of your template

# SSH Keys for VMs
default_ssh_keys = [
  "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQC8d/Bg52qrZqSRqdEeqy0rT1+5f/cP5BnsN5GHNQXP7PHe/0pGqADr6tMTboRwE9+q/6YdjzmnpE3qP1xqruh4gkH5GyvsGSuZofBCJuwumQZt91Oxul6Gu04eO2yj1qf2VXcPKbyvOgkbOqtudNMBwR3iZd18ce6Se03srzu1FVo1tPCItkxdRhNyMWbGaU7TS64P+KTSkqK1hkCxPJNz0gs0C+KabKE4glIO5AlXNoAxXAlNfvq0k27TyfJKRZFduN8blOC5owIlR7iyviWBK2FAKMu2/JW8/hFcjdtL8YN3wNcp0u/7KxgqGkPpW21mGtFdsQV4FfvcogLHjnhO5fMjw4iyBp71ZL73AbUj4+LFFfobPu19zpUOcOCUe+CDpwMnW6s5WfolRk8P6a0arQrA7lYfgV815EugPBlyEit89mBSpvaCASrVO8HhnZF3hzbk6DqFt9ePEybqMmFtTHM1M4yYuydDNqNEj5f1/DImRERJxBTi7M6QR7ICtoyksd6x8Sk1TtUp9dVUB4UaIycVG3DnAXILkbsfF+X95hU5vEJw1wuwU26R1JE6HtTXYnrH9Wi0f8RhoS2UXEgmt9V3h/FsGuH/J9U4K7A3XqNBGtIULeL2hi2rAncvD2K49cY95by4gayolLvHJ4bt0Ts8DuNnDcBJ7HzW5AruYw== eduardo.mmd.correia@gmail.com"
]

# VM Configurations
vms = {
  ips-protected-server-1 = {
    vm_name   = "ips-protected-server-1"
    cores     = 2
    memory    = 4096
    disk_size = 30

    # Static IP
    use_dhcp   = false
    ip_address = "192.168.1.100"
    ip_cidr    = "24"
    gateway    = "192.168.1.1"

    tags = ["server", "ips"]
  }

}
