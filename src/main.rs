use secrecy::{ExposeSecret, SecretBox};
use std::{env, fs, io, io::Write, path::Path, process};

pub mod api;
mod commands;

use commands::{COMMANDS, COMMAND_GROUPS};

#[tokio::main]
async fn main() {
    println!("Welcome to Hubit!");

    let token_path = Path::new(&env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.config", home)
    }))
    .join("hubit/github_token");

    if !token_path.exists() {
        println!("Token not found. Requesting...");
        store_pat(&token_path, request_pat()).expect("Failed to store PAT");
        println!("Token stored.");
        process::exit(0);
    }

    let token = get_pat(&token_path);

    let request_client = api::init();

    loop {
        print!("\n> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let input = input.trim();
        let mut input_args: Vec<&str> = input.split_whitespace().collect();

        if input_args.is_empty() {
            continue;
        }

        let input_group = input_args.remove(0);

        if let Some(group) = COMMAND_GROUPS.iter().find(|com_group| {
            com_group.name == input_group || com_group.alias.contains(&input_group)
        }) {
            if input_args.is_empty() {
                eprintln!("{}: {}", group.name, group.docs);
                continue;
            }
            let input_command = input_args.remove(0);
            if let Some(command) = COMMANDS
                .iter()
                .find(|com| com.name == input_command || com.alias.contains(&input_command))
            {
                if input_args.is_empty() {
                    eprintln!("{} {}: {}", command.name, command.args, command.docs);
                    continue;
                }

                for arg in input_args {
                    println!("{}", arg);    // process_arg
                }
            }
        } else if input_group == "exit" || input_group == "quit" {
            break;
        } else {
            eprintln!("Command not found: {}", input);
        }
    }
}

fn request_pat() -> SecretBox<String> {
    print!("Paste your GitHub PAT (Personal Access Token) below.\nYou can generate a PAT here: https://github.com/settings/tokens/new\nThe needed scopes are: repo. It's recommended to set expiration to 'No expiration'.\n\n> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.pop();

    print!("\x1B[1A");
    print!("\x1B[2K");
    print!("\x1B[2K> [TOKEN]\n\n");

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
