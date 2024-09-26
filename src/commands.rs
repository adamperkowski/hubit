use std::{future::Future, pin::Pin};

use reqwest::{Client, StatusCode};
use secrecy::SecretBox;

use crate::api::*;

trait CommandFunc {
    fn process_arg(&self);
}

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
}

pub const COMMAND_GROUPS: &[CommandGroup] = &[CommandGroup {
    name: "issue",
    alias: &["is", "i"],
    docs: "WIP",
}];

pub const COMMANDS: &[Command] = &[Command {
    name: "create",
    args: "[user]/[repo]",
    alias: &["cr", "c"],
    group: &COMMAND_GROUPS[0],
    docs: "WIP",
}];
