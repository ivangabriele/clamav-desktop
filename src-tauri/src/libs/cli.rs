use tauri::utils::platform::current_exe;

pub async fn run_sidecar_with_elevated_permissions(
    sidecar: &str,
    args: Vec<String>,
) -> Result<(String, String), String> {
    let sidecar_path = current_exe().unwrap().parent().unwrap().join(sidecar);
    println!("sidecar_path: {:?}", sidecar_path);
    let sidecar_path_as_str = sidecar_path
        .to_str()
        .ok_or("Failed to convert sidecar path to string")?;

    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        // Run sidecar with elevated permissions via `pkexec`
        let output = Command::new("pkexec")
            .arg(sidecar_path_as_str)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute sidecar: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Sidecar `{}` exited with status {}",
                sidecar,
                output.status.code().unwrap_or(-1)
            ));
        }

        let stdout = String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 in stdout: {}", e))?;
        let stderr = String::from_utf8(output.stderr).map_err(|e| format!("Invalid UTF-8 in stderr: {}", e))?;

        return Ok((stdout, stderr));
    }

    #[cfg(target_os = "macos")]
    {
        use shell_quote::quote;
        use std::process::Command;

        // Build the command with properly quoted arguments
        let mut command_parts = vec![sidecar_path_str];
        for arg in &args {
            command_parts.push(arg);
        }
        let quoted_command = quote(command_parts.iter().map(|s| s.as_ref()));

        // Build the AppleScript command
        let apple_script = format!("do shell script {} with administrator privileges", quoted_command);

        // Run sidecar with elevated permissions via `osascript`
        let output = Command::new("osascript")
            .arg("-e")
            .arg(&apple_script)
            .output()
            .map_err(|e| format!("Failed to execute sidecar: {}", e))?;

        let stdout = String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 in stdout: {}", e))?;
        let stderr = String::from_utf8(output.stderr).map_err(|e| format!("Invalid UTF-8 in stderr: {}", e))?;

        return Ok((stdout, stderr));
    }

    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use widestring::U16CString;
        use windows::core::PCWSTR;
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

        let args_as_str = args.join(" ");
        // Convert strings to wide strings
        let operation = U16CString::from_str("runas").unwrap();
        let file = U16CString::from_str(sidecar_path_str).unwrap();
        let parameters_wide = U16CString::from_str(&args_as_str).unwrap();

        // Run sidecar with elevated permissions
        let result = unsafe {
            ShellExecuteW(
                HWND(0),
                PCWSTR(operation.as_ptr()),
                PCWSTR(file.as_ptr()),
                PCWSTR(parameters_wide.as_ptr()),
                PCWSTR(std::ptr::null()),
                SW_SHOWNORMAL,
            )
        };

        if result.0 as usize <= 32 {
            return Err(format!("Failed to execute sidecar, error code: {}", result.0 as usize));
        }

        return ("".to_string(), "".to_string());
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    Err("Unsupported platform".to_string())
}
