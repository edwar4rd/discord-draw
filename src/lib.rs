pub mod commands;

pub mod prelude {
    pub use crate::commands;

    pub use poise::ApplicationCommandOrAutocompleteInteraction;
    pub use poise::{
        serenity_prelude as serenity,
        Context::{Application, Prefix},
    };
    use serenity::ApplicationCommandInteraction;
    pub use std::net::IpAddr;
    pub use std::process::Command;
    pub use std::time::Duration;

    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Context<'a> = poise::Context<'a, Data, Error>;
    pub struct Data {
        pub image: std::sync::Arc<std::sync::Mutex<image::DynamicImage>>,
    }
    pub struct ShardManagerContainer;

    pub fn slash_ctx_as_applicationcommand_interaction<'a>(
        ctx: &'a Context<'_>,
    ) -> &'a ApplicationCommandInteraction {
        match &ctx {
            Application(application_context) => match application_context.interaction {
                ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
                    interaction
                }
                _ => {
                    unreachable!()
                }
            },
            _ => {
                unreachable!();
            }
        }
    }

    impl serenity::TypeMapKey for ShardManagerContainer {
        type Value = std::sync::Arc<tokio::sync::Mutex<serenity::ShardManager>>;
    }
}
