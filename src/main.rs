use std::path::Path;

use clap::{Parser, Subcommand};
use self_update::self_replace;
use self_update::{backends::github, update::Release};

/// Type-struct to implement `Display` for `Release` for the selector.
#[derive(Debug, Clone)]
struct MyRelease(Release);

impl From<MyRelease> for Release {
    fn from(mr: MyRelease) -> Self {
        mr.0
    }
}

impl std::fmt::Display for MyRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// View releases
    Releases,
    /// Self-replace with the latest release
    Replace,
    /// Show the latest release
    Latest,
}

pub fn main() -> eyre::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Releases => releases()?,
        Commands::Latest => latest()?,
        Commands::Replace => replace()?,
    }
    Ok(())
}

pub fn latest() -> eyre::Result<()> {
    let release = github::Update::configure()
        .repo_owner("erhant")
        .repo_name("self-update-tester")
        .bin_name("self-update-tester-macOS-amd64")
        .current_version(Default::default()) // this is not used within `get_latest_release`
        .build()?
        .get_latest_release()?;

    println!("{}", MyRelease(release));

    Ok(())
}

pub fn releases() -> eyre::Result<()> {
    let releases = github::ReleaseList::configure()
        .repo_owner("erhant")
        .repo_name("self-update-tester")
        .build()?
        .fetch()?
        .into_iter()
        .map(MyRelease)
        .collect::<Vec<_>>();

    // println!("{}", MyRelease(release));

    for release in &releases {
        println!("{:#?}", release);
    }

    Ok(())
}

pub fn replace() -> eyre::Result<()> {
    println!("Replacing myself after 2 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    let new_exe = Path::new("./bin/self-update-tester-macOS-arm64_0-1-1");
    assert!(new_exe.exists(), "path must exist");

    self_replace::self_replace(new_exe)?;
    if std::env::var("FORCE_EXIT").ok().as_deref() == Some("1") {
        std::process::exit(0);
    }

    Ok(())
}
