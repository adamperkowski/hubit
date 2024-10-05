use secrecy::{ExposeSecret, SecretBox};
use std::{env, fs, io, io::Write, path::Path, path::PathBuf, process};

pub mod api;
mod commands;
mod text;

use commands::{init_commands, CommandFunction, COMMAND_GROUPS};
use text::SHELL_CHARS;

#[tokio::main]
async fn main() {
    text::welcome();

    let token_path = get_token_path();
    if !token_path.exists() {
        println!(
            "{} Token not found.\n{} Requesting...",
            SHELL_CHARS.error, SHELL_CHARS.info
        );
        store_pat(&token_path, request_pat()).expect("Failed to store PAT");
        println!("{} Token stored.", SHELL_CHARS.success);

        process::exit(0);
    }

    let request_client = api::init();
    let commands = init_commands();

    loop {
        print!("{} ", SHELL_CHARS.prompt);

        let input = read_input_line();
        if input.is_empty() {
            print!("\x1B[1A");
            continue;
        }

        print!("\x1B[1A");
        text::command_processed(input.clone());

        let token = get_pat(&token_path);

        let input_args: Vec<&str> = input.split_whitespace().collect();

        match handle_input(input_args, &commands) {
            Ok(command_func) => command_func(
                request_client.clone(),
                token,
                vec!["adamperkowski", "hubit_test_repo"],
            )
            .await
            .expect("Failed to execute"),
            Err(err) => eprintln!("{} {}", SHELL_CHARS.error, err),
        }
    }
}

fn get_token_path() -> PathBuf {
    let config_dir = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.config", home)
    });

    Path::new(&config_dir).join("hubit/github_token")
}

fn request_pat() -> SecretBox<String> {
    print!("\nPaste your GitHub PAT (Personal Access Token) below.\nYou can generate a PAT here: https://github.com/settings/tokens/new\nThe needed scopes are: repo. It's recommended to set expiration to 'No expiration'.\n\n{} ", SHELL_CHARS.prompt);

    let input = read_input_line();

    print!("\x1B[1A");
    text::command_processed("[TOKEN]".to_string());
    println!();

    SecretBox::new(input.into())
}

fn store_pat(path: &Path, token: SecretBox<String>) -> Result<(), std::io::Error> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = fs::File::create(path)?;
    file.write_all(token.expose_secret().as_bytes())?;

    Ok(())
}

fn get_pat(path: &Path) -> SecretBox<String> {
    SecretBox::new(
        fs::read_to_string(path)
            .expect("Failed to read token")
            .into(),
    )
}

fn read_input_line() -> String {
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn handle_input<'a>(
    input_args: Vec<&'a str>,
    commands: &'a [commands::Command],
) -> Result<&'a CommandFunction, String> {
    let input_group = input_args.first().ok_or("No command provided")?;

    if *input_group == "exit" || *input_group == "quit" {
        process::exit(0);
    }

    let input_command = input_args
        .get(1)
        .ok_or(format!("Command not found: {}", input_group))?;

    let command_group = COMMAND_GROUPS
        .iter()
        .find(|com_group| com_group.name == *input_group || com_group.alias.contains(input_group));

    if let Some(command_group) = command_group {
        let command = commands.iter().find(|com| {
            com.group == command_group && com.name == *input_command
                || com.alias.contains(input_command)
        });

        if let Some(command) = command {
            Ok(&command.func)
            /* if input_args.len() < 3 {
                eprintln!("{} {}: {}", command.name, command.args, command.docs);
                ()
            }

            for arg in input_args.iter().skip(2) {
                // process_arg
            } */
        } else {
            Err(format!("Command not found: {}", input_command))
        }
    } else {
        Err(format!("Command not found: {}", input_group))
    }
}
