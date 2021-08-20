use serenity::framework::{StandardFramework, standard::macros::*};

use crate::commands;

use commands::{
    developer::info::*
};
use std::borrow::Borrow;

#[group("Developer")]
#[description = "Developer commands used in development"]
#[commands(info)]
struct Developer;




pub fn add_commands(framework: StandardFramework) -> StandardFramework {
    framework
        .group(&DEVELOPER_GROUP)
}