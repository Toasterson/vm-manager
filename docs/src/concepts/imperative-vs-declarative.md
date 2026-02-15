# Imperative vs Declarative

vmctl supports two workflows for managing VMs.

## Imperative

Use individual commands to create, configure, and manage VMs step by step:

```bash
vmctl create --name myvm --image-url https://example.com/image.img --vcpus 2 --memory 2048 --start
vmctl ssh myvm
vmctl stop myvm
vmctl destroy myvm
```

This is useful for:
- Quick one-off VMs
- Experimenting with different images
- Scripting custom workflows

## Declarative

Define your VMs in a `VMFile.kdl` and let vmctl converge to the desired state:

```kdl
vm "myvm" {
    image-url "https://example.com/image.img"
    vcpus 2
    memory 2048

    cloud-init {
        hostname "myvm"
    }

    ssh {
        user "ubuntu"
    }

    provision "shell" {
        inline "echo hello"
    }
}
```

```bash
vmctl up          # create + start + provision
vmctl down        # stop
vmctl reload      # destroy + recreate + provision
vmctl provision   # re-run provisioners only
```

This is useful for:
- Reproducible development environments
- Multi-VM setups
- Checked-in VM definitions alongside your project
- Complex provisioning workflows

## When to Use Which

| Scenario | Approach |
|---|---|
| "I need a quick VM to test something" | Imperative |
| "My project needs a build VM with specific packages" | Declarative |
| "I want to script VM lifecycle in CI" | Either, depending on complexity |
| "Multiple VMs that work together" | Declarative |
