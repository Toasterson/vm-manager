# vmctl

The main entry point for the vmctl CLI.

## Synopsis

```
vmctl <COMMAND>
```

## Commands

| Command | Description |
|---|---|
| `create` | Create a new VM |
| `start` | Start an existing VM |
| `stop` | Stop a running VM |
| `destroy` | Destroy a VM and clean up resources |
| `list` | List all VMs |
| `status` | Show detailed VM status |
| `console` | Attach to serial console |
| `ssh` | SSH into a VM |
| `suspend` | Suspend (pause) a running VM |
| `resume` | Resume a suspended VM |
| `image` | Manage VM images |
| `up` | Bring up VMs from VMFile.kdl |
| `down` | Bring down VMs from VMFile.kdl |
| `reload` | Destroy and recreate VMs from VMFile.kdl |
| `provision` | Re-run provisioners from VMFile.kdl |
| `log` | Show VM logs |

## Environment Variables

| Variable | Description |
|---|---|
| `RUST_LOG` | Control log verbosity (e.g., `RUST_LOG=debug vmctl up`) |
| `XDG_DATA_HOME` | Override data directory (default: `~/.local/share`) |
