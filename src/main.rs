use inquire::Select;
use self_update::{backends::github, update::Release};

/// Type-struct to implement `Display` for `Release` for the selector.
struct MyRelease(Release);

impl std::fmt::Display for MyRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

pub fn main() -> eyre::Result<()> {
    // https://github.com/jaemk/self_update/issues/44

    let mut rel_builder = github::ReleaseList::configure();

    let releases = rel_builder
        .repo_owner("erhant")
        .repo_name("self-update-tester")
        .build()
        .unwrap() // TODO:!!!
        .fetch()
        .unwrap() // TODO:!!!
        .into_iter()
        .map(|r| MyRelease(r))
        .collect::<Vec<_>>();

    // .iter().filter(|r| r.version.starts_with);

    let Some(chosen_release) = Select::new("Select a version:", releases)
        .with_help_message("↑↓ to move, enter to select, type to filter, ESC to go back")
        .prompt_skippable()?
    else {
        return Ok(());
    };

    println!("Chosen version: {}", chosen_release);

    Ok(())
}
