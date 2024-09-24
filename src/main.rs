use secrecy::{ExposeSecret, SecretBox};
use std::{env, fs, io, io::Write, path::Path};

mod api;

#[tokio::main]
async fn main() {
    println!("Welcome to Hubit!");

    let token_path = Path::new(&env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.config", home)
    }))
    .join("hubit/github_token");

    if !token_path.exists() {
        store_pat(&token_path, request_pat()).expect("Failed to store PAT");
    }

    let token = get_pat(&token_path);

    let request_client = api::init();

    api::list_assigned_issues(request_client, token).await;
}

fn request_pat() -> SecretBox<String> {
    print!("Paste your GitHub PAT (Personal Access Token) below.\nYou can generate a PAT here: https://github.com/settings/tokens/new\nThe needed scopes are: repo. It's recommended to set expiration to 'No expiration'.\n> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    SecretBox::new(input.replace("\n", "").into())
}

fn store_pat(path: &Path, token: SecretBox<String>) -> Result<(), std::io::Error> {
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
