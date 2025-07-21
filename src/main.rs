use clap::Parser;
use commands::{Cli, Commands, SyncAction};
use nix::NixPath;
use owo_colors::OwoColorize;
use profiles::{Config, Profile};
use std::{
    env,
    error::Error,
    fs,
    process::{self, Command, Stdio},
    thread::sleep,
    time::Duration,
};

mod commands;
mod profiles;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let config = Config::load()?;

    match args.command {
        Commands::Sync { action } => {
            match action {
                SyncAction::All => {
                    println!("{}", "Syncing all profiles...".bold().underline().blue());

                    config.sync.iter().try_for_each(|entry| run_sync(entry))?;
                }
                SyncAction::Profile { name } => {
                    if let Some(entry) = config.sync.iter().find(|e| e.name == name) {
                        run_sync(entry)?;
                    } else {
                        eprintln!(
                            "{}{}{}",
                            "Profile '".red(),
                            name.red().italic(),
                            "' not found.".red()
                        );
                    }
                }
            };
        }
        Commands::Init => init_config()?,

        Commands::List => {
            println!("{}", "Profiles List".bold().underline().blue());

            for profile in config.sync.iter() {
                println!();
                println!("{}", profile.name.bold());
                println!("{} {}", "from:   ", profile.from.dimmed());
                println!("{} {}", "to:     ", profile.to.dimmed());
                println!("{} {}", "mode:   ", profile.mode.dimmed());

                if let Some(flags) = &profile.flags {
                    println!("{} {}", "flags:  ", flags.dimmed());
                } else {
                    println!("{} {}", "flags:  ", "None".dimmed());
                }
            }
        }

        Commands::Watch {
            action,
            interval,
            __run,
        } => {
            if __run {
                match action {
                    SyncAction::Profile { name } => {
                        if let Some(profile) = config.sync.iter().find(|e| e.name == name) {
                            loop {
                                println!(
                                    "{} {}",
                                    "Watching profile".cyan().bold(),
                                    name.italic().cyan()
                                );
                                if let Err(err) = run_sync(profile) {
                                    eprintln!("{} {}", "Error:".red(), err);
                                }

                                sleep(Duration::from_secs(interval));
                            }
                        } else {
                            eprintln!(
                                "{}{}{}",
                                "Profile '".red(),
                                name.red().italic(),
                                "' not found.".red()
                            );
                        }
                    }
                    SyncAction::All => loop {
                        println!("{}", "Watching all profiles".cyan().bold());
                        for profile in config.sync.iter() {
                            if let Err(err) = run_sync(profile) {
                                eprintln!("{} {}", "Error:".red(), err);
                            }
                        }

                        sleep(Duration::from_secs(interval));
                    },
                }
            } else {
                match action {
                    SyncAction::Profile { name } => {
                        if let Some(profile) = config.sync.iter().find(|e| e.name == name) {
                            spawn_watcher(profile, interval)?;
                        } else {
                            eprintln!(
                                "{}{}{}",
                                "Profile '".red(),
                                name.red().italic(),
                                "' not found.".red()
                            );
                        }
                    }
                    SyncAction::All => {
                        for profile in config.sync.iter() {
                            spawn_watcher(profile, interval)?;
                        }
                    }
                }
            }
        }

        Commands::Stop { action } => match action {
            SyncAction::Profile { name } => {
                stop_profile(&name)?;
            }
            SyncAction::All => {
                for profile in config.sync.iter() {
                    if let Err(err) = stop_profile(&profile.name) {
                        eprintln!(
                            "{} '{}' {} {}",
                            "Failed to stop".red(),
                            profile.name.red().italic(),
                            ":".red(),
                            err
                        );
                    }
                }
            }
        },

        Commands::Status => {
            let pid_dir = Config::path().join(".pids");

            if pid_dir.exists() {
                for entry in fs::read_dir(pid_dir)? {
                    let entry = entry?;
                    let name = entry.file_name().to_string_lossy().replace(".pid", "");

                    println!("{} {}", "Running:".green().bold(), name.italic());
                }
            } else if pid_dir.is_empty() {
                println!("{}", "No watchers are running.".dimmed());
            }
        }
    }

    Ok(())
}

fn run_sync(entry: &Profile) -> Result<(), Box<dyn Error>> {
    if let Err(e) = check() {
        eprintln!("{} {}", "Error:".red().bold(), e.red());
        process::exit(1);
    }

    let mode = match entry.mode.as_str() {
        "replace" => "sync --delete-excluded",
        "mirror" => "sync",
        "copy" => "copy",
        "move" => "move",
        other => {
            eprintln!("{} {}", "Unknown mode:".red().bold(), other.dimmed());
            return Ok(());
        }
    };

    let mut cmd = Command::new("rclone");
    for part in mode.split_whitespace() {
        cmd.arg(part);
    }

    cmd.arg(&entry.from).arg(&entry.to);

    if let Some(flags) = &entry.flags {
        for flag in flags.split_whitespace() {
            cmd.arg(flag);
        }
    }

    println!("Running: {:?}", cmd);
    let status = cmd.status()?;

    if !status.success() {
        eprintln!("{}", "rclone exited with non-zero status".blue().bold());
    }

    Ok(())
}

fn check() -> Result<(), String> {
    let res = Command::new("rclone").arg("--version").output();

    match res {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err("rclone is not installed or not working".into()),
    }
}

fn init_config() -> Result<(), Box<dyn Error>> {
    let conf_dir = Config::path();
    let conf_path = conf_dir.join("profiles.toml");

    if conf_path.exists() {
        println!(
            "{} {}",
            "Config already exists at:".bold().red(),
            conf_path.display().dimmed()
        );
        return Ok(());
    }

    let template = r#"
# lazycloud config

# [[sync]]
# name = "" # name of the profile
# from = "" # has to be full path
# to = "" # example gdrive:projects
# mode = "replace" # or "mirror", "copy", "move"
# flags = "--progress" # any flags that are normally supported in rclone
"#;

    fs::write(&conf_path, template)?;

    println!(
        "{} {}",
        "Created blank profiles config at:".bold().blue(),
        conf_path.display().dimmed()
    );
    Ok(())
}

fn spawn_watcher(profile: &Profile, interval: u64) -> Result<(), Box<dyn Error>> {
    let conf_dir = Config::path().join(".pids");
    fs::create_dir_all(&conf_dir)?;

    let pid_path = conf_dir.join(format!("{}.pid", profile.name));

    if pid_path.exists() {
        println!(
            "{}{}{}",
            "Watcher for '".red().bold(),
            profile.name.red().italic(),
            "' is already running".red().bold()
        );
        return Ok(());
    }

    let child = Command::new(env::current_exe()?)
        .arg("watch")
        .arg(interval.to_string())
        .arg("--run")
        .arg("profile")
        .arg(&profile.name)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    fs::write(&pid_path, child.id().to_string())?;
    println!(
        "{} {} {}{}",
        "Started watcher for".bold().green(),
        format!("'{}' (PID", profile.name).green().italic(),
        child.id().green().italic(),
        ")".green().italic(),
    );

    Ok(())
}

fn stop_profile(profile: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pid_path = Config::path().join(".pids").join(format!("{profile}.pid"));

    if !pid_path.exists() {
        println!(
            "{} {}",
            "No watcher running for profile".red().bold(),
            format!("'{profile}'").red().italic()
        );
        return Ok(());
    }

    let pid = fs::read_to_string(&pid_path)?.trim().parse::<u32>()?;

    #[cfg(target_family = "unix")]
    {
        use nix::{
            sys::signal::{Signal, kill},
            unistd::Pid,
        };

        let result = kill(Pid::from_raw(pid as i32), Signal::SIGTERM);
        match result {
            Ok(_) => {}
            Err(nix::Error::ESRCH) => {
                println!(
                    "{} {}",
                    "No running process found for".yellow().bold(),
                    format!("'{profile}'").yellow().italic()
                );
                if pid_path.exists() {
                    fs::remove_file(&pid_path)?;
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }

    #[cfg(target_family = "windows")]
    {
        Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .status()?;
    }

    fs::remove_file(pid_path)?;
    println!(
        "{} {}",
        "Stopped watcher for".green().bold(),
        format!("'{profile}'").green().italic()
    );

    Ok(())
}
