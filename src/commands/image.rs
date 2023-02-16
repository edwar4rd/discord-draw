use image::EncodableLayout;

use crate::prelude::*;

/// Displays information about the current image
#[poise::command(slash_command, rename = "image-info")]
pub async fn img_info(ctx: Context<'_>) -> Result<(), Error> {
    use image::GenericImageView;

    let dimension = { ctx.data().image.lock().unwrap().dimensions() };
    let color_type = { ctx.data().image.lock().unwrap().color() };

    ctx.send(|msg| {
        msg.ephemeral(true).content(format!(
            "```dimension={}x{}\ncolor_type={:?}```",
            dimension.0, dimension.1, color_type
        ))
    })
    .await?;
    Ok(())
}

// /// Reset the whole image and create a new one
// #[poise::command(slash_command, rename = "image-reset")]
// pub async fn img_reset(ctx: Context<'_>) -> Result<(), Error> {
//     ctx.send(|msg| {
//         msg.ephemeral(true).content(format!(
//             "```dimension={}x{}\ncolor_type={:?}```",
//             dimension.0, dimension.1, color_type
//         ))
//     })
//     .await?;
//     Ok(())
// }

/// Displays information about the current image
#[poise::command(slash_command, rename = "image-view")]
pub async fn img_view(ctx: Context<'_>, public: bool) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let image_vec_cursor = {
        let mut vec_writer = std::io::Cursor::new(Vec::new());
        image::imageops::resize(
            &(*ctx.data().image.lock().unwrap()),
            512,
            512,
            image::imageops::Nearest,
        )
        .write_to(&mut vec_writer, image::ImageFormat::Png)
        .unwrap();
        vec_writer
    };

    let interaction = slash_ctx_as_applicationcommand_interaction(&ctx);
    if public {
        ctx.channel_id()
            .send_message(ctx, |m| {
                m.add_file((image_vec_cursor.get_ref().as_bytes(), "image.png"))
            })
            .await?;
        interaction
            .create_followup_message(ctx, |m| m.content("Image sent!"))
            .await?;
    } else {
        interaction
            .create_followup_message(ctx, |m| {
                m.add_file((image_vec_cursor.get_ref().as_bytes(), "image.png"))
            })
            .await?;
    }

    Ok(())
}
