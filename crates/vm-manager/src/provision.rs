use std::path::Path;

use ssh2::Session;
use tracing::info;

use crate::error::{Result, VmError};
use crate::ssh;
use crate::vmfile::{FileProvision, ProvisionDef, ShellProvision, resolve_path};

/// Run all provision steps on an established SSH session.
pub fn run_provisions(
    sess: &Session,
    provisions: &[ProvisionDef],
    base_dir: &Path,
    vm_name: &str,
) -> Result<()> {
    for (i, prov) in provisions.iter().enumerate() {
        let step = i + 1;
        match prov {
            ProvisionDef::Shell(shell) => {
                run_shell(sess, shell, base_dir, vm_name, step)?;
            }
            ProvisionDef::File(file) => {
                run_file(sess, file, base_dir, vm_name, step)?;
            }
        }
    }
    Ok(())
}

fn run_shell(
    sess: &Session,
    shell: &ShellProvision,
    base_dir: &Path,
    vm_name: &str,
    step: usize,
) -> Result<()> {
    if let Some(ref cmd) = shell.inline {
        info!(vm = %vm_name, step, cmd = %cmd, "running inline shell provision");
        let (stdout, stderr, exit_code) =
            ssh::exec(sess, cmd).map_err(|e| VmError::ProvisionFailed {
                vm: vm_name.into(),
                step,
                detail: format!("shell exec: {e}"),
            })?;

        if exit_code != 0 {
            return Err(VmError::ProvisionFailed {
                vm: vm_name.into(),
                step,
                detail: format!(
                    "inline command exited with code {exit_code}\nstdout: {stdout}\nstderr: {stderr}"
                ),
            });
        }
        info!(vm = %vm_name, step, "inline shell provision completed");
    } else if let Some(ref script_raw) = shell.script {
        let local_path = resolve_path(script_raw, base_dir);
        info!(vm = %vm_name, step, script = %local_path.display(), "running script provision");

        let remote_path_str = format!("/tmp/vmctl-provision-{step}.sh");
        let remote_path = Path::new(&remote_path_str);

        // Upload the script
        ssh::upload(sess, &local_path, remote_path).map_err(|e| VmError::ProvisionFailed {
            vm: vm_name.into(),
            step,
            detail: format!("upload script: {e}"),
        })?;

        // Make executable and run
        let run_cmd = format!("chmod +x {remote_path_str} && {remote_path_str}");
        let (stdout, stderr, exit_code) =
            ssh::exec(sess, &run_cmd).map_err(|e| VmError::ProvisionFailed {
                vm: vm_name.into(),
                step,
                detail: format!("script exec: {e}"),
            })?;

        if exit_code != 0 {
            return Err(VmError::ProvisionFailed {
                vm: vm_name.into(),
                step,
                detail: format!(
                    "script exited with code {exit_code}\nstdout: {stdout}\nstderr: {stderr}"
                ),
            });
        }
        info!(vm = %vm_name, step, "script provision completed");
    }
    Ok(())
}

fn run_file(
    sess: &Session,
    file: &FileProvision,
    base_dir: &Path,
    vm_name: &str,
    step: usize,
) -> Result<()> {
    let local_path = resolve_path(&file.source, base_dir);
    let remote_path = Path::new(&file.destination);

    info!(
        vm = %vm_name,
        step,
        source = %local_path.display(),
        destination = %file.destination,
        "running file provision"
    );

    ssh::upload(sess, &local_path, remote_path).map_err(|e| VmError::ProvisionFailed {
        vm: vm_name.into(),
        step,
        detail: format!("file upload: {e}"),
    })?;

    info!(vm = %vm_name, step, "file provision completed");
    Ok(())
}
