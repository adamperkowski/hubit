use colored::{ColoredString, Colorize};
use once_cell::sync::Lazy;

pub struct ShellChars {
    welcome: ColoredString,
    pub info: ColoredString,
    pub success: ColoredString,
    pub error: ColoredString,
    pub prompt: ColoredString,
    prompt_processed: ColoredString,
}

pub static SHELL_CHARS: Lazy<ShellChars> = Lazy::new(|| ShellChars {
    welcome: ":".magenta().bold(),
    info: ":".magenta(),
    success: ".".green(),
    error: "!".red(),
    prompt: "?".blue(),
    prompt_processed: ">".yellow(),
});

pub fn welcome() {
    println!("{} {}", SHELL_CHARS.welcome, "Welcome to Hubit!".bold());
}

pub fn command_processed(force_string: String) {
    println!(
        "\x1B[2K{} {}",
        SHELL_CHARS.prompt_processed,
        force_string.bright_black()
    )
}
