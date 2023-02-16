use crate::prelude::*;

/// Show a help menu
#[poise::command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}

/// Displays information about the bot
#[poise::command(slash_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    ctx.send(|msg| {
        msg.ephemeral(true).content(format!(
            "```version = {}\n```",
            VERSION.unwrap_or("UNKNOWN"),
        ))
    })
    .await?;
    Ok(())
}

pub mod draw;
pub mod image;
