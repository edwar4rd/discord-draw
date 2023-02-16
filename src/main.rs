use discord_draw::prelude::*;

#[cfg(feature = "handler")]
async fn handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut commands = vec![];

    commands.push(commands::help());
    commands.push(commands::botinfo());

    commands.push(commands::draw::change_pixel());
    commands.push(commands::draw::erase_pixel());

    commands.push(commands::image::img_info());
    commands.push(commands::image::img_view());

    #[allow(unused_mut)]
    let mut options = poise::FrameworkOptions {
        commands,
        ..Default::default()
    };

    #[cfg(feature = "handler")]
    {
        options.event_handler = |ctx, event, _framework, data| Box::pin(handler(ctx, event, data));
    }

    let image = image::DynamicImage::new_rgba8(64, 64);
    let image_arc = std::sync::Arc::new(std::sync::Mutex::new(image));

    let framework = poise::Framework::builder()
        .options(options)
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_presence(
                    Some(serenity::Activity::watching(
                        "大家一起成為電繪大師吧  ٩(´ᗜ`*)୨",
                    )),
                    serenity::OnlineStatus::Idle,
                )
                .await;
                ctx.data
                    .write()
                    .await
                    .insert::<ShardManagerContainer>(std::sync::Arc::clone(
                        &framework.shard_manager(),
                    ));
                Ok(Data {
                    image: image_arc.clone(),
                })
            })
        });

    framework.run().await.unwrap();
    eprintln!(
        "Bot stopped at {}...",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    println!("exit\n");
}
