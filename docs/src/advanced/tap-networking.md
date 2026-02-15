# TAP Networking and Bridges

TAP networking gives VMs a real presence on a host network, with full Layer 2 connectivity.

## Creating a Bridge

```bash
# Create bridge
sudo ip link add br0 type bridge
sudo ip link set br0 up

# Assign an IP to the bridge (optional, for host-to-guest communication)
sudo ip addr add 10.0.0.1/24 dev br0
```

## DHCP with dnsmasq

Provide IP addresses to guests:

```bash
sudo dnsmasq \
  --interface=br0 \
  --bind-interfaces \
  --dhcp-range=10.0.0.100,10.0.0.200,12h \
  --no-daemon
```

## IP Forwarding and NAT

If you want guests to reach the internet:

```bash
# Enable forwarding
sudo sysctl -w net.ipv4.ip_forward=1

# NAT outbound traffic
sudo iptables -t nat -A POSTROUTING -s 10.0.0.0/24 ! -o br0 -j MASQUERADE
```

## Using with vmctl

### Imperative

```bash
vmctl create --name myvm --image ./image.qcow2 --bridge br0
```

### Declarative

```kdl
vm "myvm" {
    image "image.qcow2"

    network "tap" {
        bridge "br0"
    }

    cloud-init {
        hostname "myvm"
    }

    ssh {
        user "ubuntu"
    }
}
```

## IP Discovery

vmctl discovers TAP-networked guest IPs by:
1. Checking the ARP table (`ip neigh show`) for the guest's MAC address on the bridge.
2. Falling back to dnsmasq lease files.

This happens automatically when you run `vmctl ssh` or provisioners.

## Security Considerations

- TAP interfaces may bypass host firewall rules.
- Guests on the bridge can see other devices on the network.
- Use iptables rules on the bridge to restrict traffic if needed.
