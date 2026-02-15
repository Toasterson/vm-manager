# Hypervisor Trait

The `Hypervisor` trait is the core abstraction for VM lifecycle management. All backends implement it.

## Definition

```rust
pub trait Hypervisor: Send + Sync {
    fn prepare(&self, spec: &VmSpec) -> impl Future<Output = Result<VmHandle>>;
    fn start(&self, vm: &VmHandle) -> impl Future<Output = Result<VmHandle>>;
    fn stop(&self, vm: &VmHandle, timeout: Duration) -> impl Future<Output = Result<VmHandle>>;
    fn suspend(&self, vm: &VmHandle) -> impl Future<Output = Result<VmHandle>>;
    fn resume(&self, vm: &VmHandle) -> impl Future<Output = Result<VmHandle>>;
    fn destroy(&self, vm: VmHandle) -> impl Future<Output = Result<()>>;
    fn state(&self, vm: &VmHandle) -> impl Future<Output = Result<VmState>>;
    fn guest_ip(&self, vm: &VmHandle) -> impl Future<Output = Result<String>>;
    fn console_endpoint(&self, vm: &VmHandle) -> Result<ConsoleEndpoint>;
}
```

## Methods

### prepare

Allocates resources for a VM based on the provided `VmSpec`. Creates the work directory, QCOW2 overlay, cloud-init ISO, and networking configuration. Returns a `VmHandle` in the `Prepared` state.

### start

Boots the VM. Returns an updated `VmHandle` with runtime information (PID, VNC address, etc.).

### stop

Gracefully shuts down the VM. Tries ACPI power-down first, then force-kills after the timeout. Returns the handle in `Stopped` state.

### suspend / resume

Pauses and unpauses VM vCPUs without shutting down.

### destroy

Stops the VM (if running) and removes all associated resources. Takes ownership of the handle.

### state

Queries the current VM state by checking the process and QMP status.

### guest_ip

Discovers the guest's IP address. Method varies by network mode and backend.

### console_endpoint

Returns the console connection details. Synchronous (not async).

## ConsoleEndpoint

```rust
pub enum ConsoleEndpoint {
    UnixSocket(PathBuf),  // QEMU serial console
    WebSocket(String),    // Propolis console
    None,                 // Noop backend
}
```

## Implementing a Custom Backend

To add a new hypervisor backend:

1. Create a struct implementing `Hypervisor`.
2. Add it to `RouterHypervisor` with appropriate `#[cfg]` gates.
3. Add a new variant to `BackendTag`.
4. Implement dispatch in `RouterHypervisor`'s `Hypervisor` impl.
