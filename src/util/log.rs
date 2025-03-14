use std::{
    fmt::Display,
    io::{self, BufRead as _},
};

use eyre::Result;
use owo_colors::OwoColorize as _;

pub fn info(item: impl Display) {
    eprintln!(
        "{}{}{}  {}",
        "[".blue().dimmed(),
        "morlana".blue().bold(),
        "]".blue().dimmed(),
        item
    );
}

pub fn error(item: impl Display) {
    eprintln!(
        "{}{}{}  {}",
        "[".red().dimmed(),
        "morlana".red().bold(),
        "]".red().dimmed(),
        item.red()
    );
}

pub fn warn(item: impl Display) {
    eprintln!(
        "{}{}{}  {}",
        "[".yellow().dimmed(),
        "morlana".yellow().bold(),
        "]".yellow().dimmed(),
        item
    );
}

pub fn success(item: impl Display) {
    eprintln!(
        "{}{}{}  {}",
        "[".green().dimmed(),
        "morlana".green().bold(),
        "]".green().dimmed(),
        item
    );
}

pub fn confirm(description: &str, default: bool) -> Result<bool> {
    eprint!(
        "{}{}{}  {} {} ",
        "[".cyan().dimmed(),
        "morlana".cyan().bold(),
        "]".cyan().dimmed(),
        description,
        (if default { "[Y/n]" } else { "[y/N]" }).dimmed()
    );

    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    input = input.trim().to_owned();

    if input.is_empty() {
        Ok(default)
    } else if input.eq_ignore_ascii_case("y") {
        Ok(true)
    } else if input.eq_ignore_ascii_case("n") {
        Ok(false)
    } else {
        error("invalid input!");
        confirm(description, default)
    }
}
