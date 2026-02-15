# Custom Cloud-Init User Data

For advanced guest configuration, you can provide a complete cloud-config YAML file instead of using vmctl's built-in cloud-init generation.

## Raw User-Data

```kdl
vm "custom" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"

    cloud-init {
        user-data "cloud-config.yaml"
    }
}
```

## Example cloud-config.yaml

```yaml
#cloud-config
users:
  - name: deploy
    groups: sudo
    shell: /bin/bash
    sudo: ALL=(ALL) NOPASSWD:ALL
    ssh_authorized_keys:
      - ssh-ed25519 AAAA... your-key

package_update: true
packages:
  - nginx
  - certbot
  - python3-certbot-nginx

write_files:
  - path: /etc/nginx/sites-available/default
    content: |
      server {
          listen 80;
          server_name _;
          root /var/www/html;
      }

runcmd:
  - systemctl enable nginx
  - systemctl start nginx

growpart:
  mode: auto
  devices: ["/"]
```

## The pure-iso Feature

By default, vmctl generates the NoCloud seed ISO by shelling out to `genisoimage` or `mkisofs`. If neither is available, you can build with the `pure-iso` feature:

```bash
cargo build --release -p vmctl --features vm-manager/pure-iso
```

This uses the `isobemak` crate to generate ISO 9660 images entirely in Rust.

## What vmctl Generates

When you don't provide raw user-data, vmctl generates a cloud-config that:

1. Creates a user with the specified name.
2. Grants passwordless sudo.
3. Sets bash as the default shell.
4. Injects the SSH public key into `authorized_keys`.
5. Disables root login.
6. Sets the hostname (from `hostname` field or VM name).
7. Sets a unique `instance-id` in the metadata.

If you need more control than this, use raw user-data.
