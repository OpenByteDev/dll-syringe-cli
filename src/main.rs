use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dll_syringe::process::Process;
use dll_syringe::{Syringe, process::OwnedProcess};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Inject(Args),
    Eject(Args),
}

#[derive(Parser, Debug)]
struct Args {
    /// Path to the DLL file
    #[arg(long)]
    dll: PathBuf,

    /// Name of the process (e.g., notepad.exe)
    #[arg(long, conflicts_with = "pid")]
    process: Option<String>,

    /// PID of the target process
    #[arg(long, conflicts_with = "process")]
    pid: Option<u32>,
}

fn report_error(err: &anyhow::Error) {
    eprintln!("❌ {err}");
    // Print the chain of errors if any:
    for cause in err.chain().skip(1) {
        eprintln!("    caused by: {cause}");
    }
}

fn main() {
    if let Err(err) = run() {
        report_error(&err);
        exit(1);
    }
}

fn find_target_process(args: &Args) -> Result<OwnedProcess> {
    if let Some(pid) = args.pid {
        OwnedProcess::from_pid(pid)
            .with_context(|| format!("Could not find process with PID {pid}"))
    } else if let Some(name) = &args.process {
        OwnedProcess::find_first_by_name(name)
            .ok_or_else(|| anyhow::anyhow!("Could not find process containing '{name}'"))
    } else {
        Err(anyhow::anyhow!(
            "You must specify either a process name or a PID."
        ))
    }
}

fn is_file_name(path: &Path) -> Result<bool> {
    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .context("Failed to determine current path")?
            .join(path)
    };

    Ok(abs_path.exists())
}

fn get_file_name(path: &Path) -> Result<String> {
    path.file_name()
        .context("Failed to get file name from path")
        .map(|name| name.to_string_lossy().into_owned())
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Inject(args) => {
            let target_process = find_target_process(args)?;
            println!(
                "✅ Found target process '{}' (PID={})",
                target_process
                    .base_name()
                    .context("Failed to determine process name")?
                    .display(),
                target_process.pid().context("Failed to determine pid")?
            );

            let dll_file_name = get_file_name(&args.dll)?;
            let syringe = Syringe::for_process(target_process);
            let module = syringe
                .inject(&args.dll)
                .with_context(|| format!("Failed to inject DLL '{dll_file_name}'"))?;

            println!("✅ Successfully injected DLL at {:p}", module.handle());
        }
        Commands::Eject(args) => {
            let target_process = find_target_process(args)?;
            println!(
                "✅ Found target process '{}' (PID={})",
                target_process
                    .base_name()
                    .context("Failed to determine process name")?
                    .display(),
                target_process.pid().context("Failed to determine pid")?
            );

            let dll_file_name = get_file_name(&args.dll)?;
            let module = if is_file_name(&args.dll)? {
                // Has a parent directory, treat as full path
                print!(
                    "Searching for DLL '{}' by path...",
                    args.dll.parent().unwrap().display()
                );
                target_process.find_module_by_path(&args.dll)
            } else {
                // No parent = just a file name, search by name
                target_process.find_module_by_name(&args.dll)
            }
            .with_context(|| "Failed to check modules in target process".to_string())?
            .with_context(|| format!("DLL '{dll_file_name}' not found in target process",))?;

            println!("✅ Found DLL '{}' at {:p}", dll_file_name, module.handle());

            let syringe = Syringe::for_process(target_process);
            syringe
                .eject(module.borrowed())
                .context("Failed to eject DLL")?;

            println!("✅ Successfully ejected DLL");
        }
    }

    Ok(())
}
