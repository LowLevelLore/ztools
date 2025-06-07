use std::{
    env,
    fs::{self},
    path::Path,
    process::{self, ExitStatus},
};

use crate::ZToolsError;

pub fn run_script(
    name: &str,
    scripts_directory: &str,
    args: &Vec<String>,
) -> Result<(), ZToolsError> {
    let base_directory: &Path = Path::new(scripts_directory);
    if !base_directory.exists() {
        return Err(ZToolsError::PathError(format!(
            "Scripts directory [{}] does not exists, \neither make one or configure the /home/user/.config/ztools/ to use some other default.",
            base_directory.display()
        )));
    }

    let mut script_path = base_directory.join(Path::new(name));

    match script_path.extension() {
        Some(extension) => {
            if extension.to_str().unwrap_or("shitty") != "sh" {
                return Err(ZToolsError::SpawnError(format!(
                    "Cannot run scripts with [{}] as extension.",
                    extension.to_str().unwrap_or_default()
                )));
            }
        }
        None => {
            script_path.set_extension("sh");
        }
    }

    if !script_path.exists() {
        return Err(ZToolsError::PathError(format!(
            "Script with name {} does not exist at location {}",
            name,
            base_directory.display()
        )));
    }

    if env::consts::OS == "linux" {
        use std::os::unix::fs::PermissionsExt;
        let _ = if let Ok(metadata) = fs::metadata(&script_path) {
            let mut perms = metadata.permissions();
            let mode = perms.mode();
            perms.set_mode(mode | 0o111);

            fs::set_permissions(&script_path, perms)
        } else {
            return Err(ZToolsError::PermissionError(format!(
                "Cannot view or change permissions of [{}]",
                script_path.display()
            )));
        };
    }

    let mut process = process::Command::new(&script_path);

    process.args(args);

    let mut child = process.spawn().map_err(|e| {
        ZToolsError::SpawnError(format!(
            "Cannot spawn process for '{}': {}",
            script_path.display(),
            e
        ))
    })?;

    let status: ExitStatus = child.wait().map_err(|e| ZToolsError::Io(e))?;
    if !status.success() {
        return Err(ZToolsError::InvalidInput(format!(
            "Script '{}' exited with status {}",
            name, status
        )));
    }

    return Ok(());
}
