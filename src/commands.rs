use std::{future::Future, pin::Pin};

use reqwest::{Client, StatusCode};
use secrecy::SecretBox;

use crate::api::*;

#[derive(PartialEq)]
pub struct CommandGroup {
    pub name: &'static str,
    pub alias: &'static [&'static str],
    pub docs: &'static str,
}

pub struct Command {
    pub name: &'static str,
    pub args: &'static str,
    pub alias: &'static [&'static str],
    pub group: &'static CommandGroup,
    pub docs: &'static str,
    pub func: Box<
        dyn Fn(
            Client,
            SecretBox<String>,
            Vec<&'static str>,
        ) -> Pin<Box<dyn Future<Output = Result<(), StatusCode>>>>,
    >,
}

pub const COMMAND_GROUPS: &[CommandGroup] = &[CommandGroup {
    name: "issue",
    alias: &["is", "i"],
    docs: "WIP",
}];

pub fn init_commands() -> Vec<Command> {
    vec![Command {
        name: "create",
        args: "[user]/[repo]",
        alias: &["cr", "c"],
        group: &COMMAND_GROUPS[0],
        docs: "WIP",
        func: Box::new(|client, token, args| {
            Box::pin(async move { create_issue(client, token, args).await })
        }),
    }]
}
