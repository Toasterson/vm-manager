# Networking Modes

vmctl supports several networking modes depending on your needs and permissions.

## User Mode (SLIRP) - Default

```kdl
network "user"
```

QEMU's built-in user-mode networking. No root or special permissions required.

**How it works:**
- QEMU emulates a full TCP/IP stack in userspace.
- The guest gets a private IP (typically `10.0.2.x`).
- Outbound connections from the guest are NAT'd through the host.
- SSH access is provided via host port forwarding (ports 10022-10122, deterministically assigned per VM name).

**Pros:** Zero setup, no root needed.
**Cons:** No inbound connections (except forwarded ports), lower performance than TAP.

## TAP Mode

```kdl
network "tap" {
    bridge "br0"
}
```

Creates a TAP device and attaches it to a host bridge. The guest appears as a real machine on the bridge's network.

**How it works:**
- vmctl creates a TAP interface and bridges it.
- The guest gets an IP via DHCP from whatever serves the bridge network.
- Full Layer 2 connectivity.

**Pros:** Real network presence, full inbound/outbound, better performance.
**Cons:** Requires bridge setup, may need root or appropriate capabilities.

If no bridge name is specified, it defaults to `br0`.

## VNIC Mode (illumos only)

```kdl
network "vnic" {
    name "vnic0"
}
```

Uses an illumos VNIC for exclusive-IP zone networking. Only available on the Propolis backend.

## None

```kdl
network "none"
```

No networking at all. Useful for isolated compute tasks or testing.

## IP Discovery

vmctl discovers the guest IP differently depending on the network mode:

| Mode | IP Discovery Method |
|---|---|
| User | Returns `127.0.0.1` (SSH via forwarded port) |
| TAP | Parses ARP table (`ip neigh show`), falls back to dnsmasq lease files by MAC address |
| VNIC | Zone-based discovery |
| None | Not available |
