use std::{
    fmt::{Debug, Display},
    fs,
    path::Path,
    process::exit,
};

use serenity::all::{Colour, CreateEmbed, CreateInteractionResponseMessage};

use super::ErrorResult;

pub fn read_token(path_ref: impl AsRef<Path>) -> ErrorResult<Option<String>> {
    let path = path_ref.as_ref();
    Ok(if !path.is_file() {
        fs::File::create(path)?;
        None
    } else {
        Some(fs::read_to_string(path)?)
    })
}

pub fn throw(message: impl Display, error: impl Debug) -> ! {
    log::error!("{message}: {error:?}");
    exit(1)
}

pub fn error_msg(
    cirm: CreateInteractionResponseMessage,
    msg: impl Into<String>,
) -> CreateInteractionResponseMessage {
    cirm.embed(
        CreateEmbed::new()
            .color(Colour::ROSEWATER)
            .title("Error")
            .description(msg),
    )
}

pub fn completed_msg(
    cirm: CreateInteractionResponseMessage,
    msg: impl Into<String>,
) -> CreateInteractionResponseMessage {
    cirm.embed(
        CreateEmbed::new()
            .color(Colour::BLURPLE)
            .title("Completed")
            .description(msg),
    )
}
