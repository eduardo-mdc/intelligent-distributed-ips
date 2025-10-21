# Proxmox Connection Settings
proxmox_endpoint = "https://proxmox.example.com:8006"
proxmox_insecure = true  # Set to false if using valid TLS certificates

# Authentication - Option 1: API Token (Recommended)
proxmox_api_token = "terraform@pam!mytoken=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"

# Authentication - Option 2: Username/Password (Alternative)
# proxmox_username = "terraform@pam"
# proxmox_password = "your-password-here"
