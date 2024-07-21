//! Run script module.

/// Spawn a script in the foreground, using the appropriate shell
///
/// This must not block. Return the child and the caller may block if they like
pub fn spawn_script(script: &str) -> anyhow::Result<std::process::Child> {
    #[cfg(target_os = "linux")]
    let runner = Some("bash".to_string());
    #[cfg(not(target_os = "linux"))]
    let runner = None;

    let options = run_script::ScriptOptions {
        runner,
        runner_args: None,
        working_directory: None,
        input_redirection: run_script::types::IoOptions::Inherit,
        output_redirection: run_script::types::IoOptions::Inherit,
        exit_on_error: true,
        print_commands: true,
        env_vars: None,
    };

    Ok(run_script::spawn_script!(script, &options)?)
}

/// Run a given script.
pub fn run_script(script: &str, verbose: bool) -> anyhow::Result<ProcessOutput> {
    #[cfg(unix)]
    {
        let options = run_script::ScriptOptions::new();
        if verbose {
            println!("Executing `{script}` using {options:?}.");
        }
        Ok(
            run_script::run(script, &vec![], &options).map(|(status, out, err)| ProcessOutput {
                code: status,
                stderr: err.trim_end().to_string(),
                stdout: out.trim_end().to_string(),
            })?,
        )
    }
    #[cfg(windows)]
    run_powershell(script, verbose)
}

/// Run powershell script in silent mode.
#[cfg(windows)]
pub fn run_powershell_failsafe(command: &str, debug: bool) -> anyhow::Result<String> {
    let ps = powershell_script::PsScriptBuilder::new()
        .hidden(true)
        .no_profile(true)
        .non_interactive(true)
        .print_commands(debug)
        .build();
    let output = ps.run(command)?;
    Ok(output.stdout().unwrap_or_default().trim_end().to_string())
}

/// Execute a powershell script in silent mode.
#[cfg(windows)]
fn run_powershell(command: &str, debug: bool) -> anyhow::Result<ProcessOutput> {
    let ps = powershell_script::PsScriptBuilder::new()
        .hidden(true)
        .no_profile(true)
        .non_interactive(true)
        .print_commands(debug)
        .build();
    let output = ps.run(command)?;
    let stdout = output
        .stdout()
        .map(|x| x.trim_end().to_string())
        .unwrap_or("".to_string());
    let stderr = output
        .stderr()
        .map(|x| x.trim_end().to_string())
        .unwrap_or("".to_string());
    Ok(ProcessOutput::new(
        if output.success() { 0 } else { 1 },
        stdout,
        stderr,
    ))
}

/// Execution status of an Process/Child.
///
/// It is a triple (`i32`, `String`, `String`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct ProcessOutput {
    /// return code.
    pub code: i32,
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
}

impl ProcessOutput {
    /// Create new [`Self`]
    pub fn new(code: i32, stdout: String, stderr: String) -> Self {
        Self {
            code,
            stdout,
            stderr,
        }
    }

    /// was the execution successful.
    pub fn success(&self) -> bool {
        #[cfg(windows)]
        {
            self.code == 0 && self.stderr.is_empty()
        }

        #[cfg(unix)]
        {
            self.code == 0
        }
    }
}

impl std::fmt::Display for ProcessOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.code, self.stdout, self.stderr)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn test_script_multiline() {
        let script = r#"ls $HOME
ls $TEMP
"#;
        let x = run_script(script, true).unwrap();
        println!("{x:?}");
        assert_eq!(x.code, 0);
        assert!(x.stdout.len() > 10);
        assert!(x.stderr.is_empty())
    }
}
