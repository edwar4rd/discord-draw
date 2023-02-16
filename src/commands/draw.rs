use image::GenericImage;

use crate::prelude::*;

/// Change a single pixel in the image
#[poise::command(slash_command, rename = "change-pixel")]
pub async fn change_pixel(ctx: Context<'_>, x: u32, y: u32, black: bool) -> Result<(), Error> {
    use image::GenericImageView;
    use image::Rgba;

    let dimension = { ctx.data().image.lock().unwrap().dimensions() };

    if x >= dimension.0 || y >= dimension.1 {
        ctx.send(|msg| {
            msg.ephemeral(true)
                .content("Specified pixel is out of the image boundary.")
        })
        .await?;
        return Ok(());
    }

    {
        ctx.data().image.lock().unwrap().put_pixel(
            x,
            y,
            if black {
                Rgba::<u8>([0, 0, 0, 255])
            } else {
                Rgba::<u8>([255, 255, 255, 255])
            },
        );
    }

    ctx.send(|msg| msg.ephemeral(true).content("Pixel drawn!"))
        .await?;
    Ok(())
}

/// Displays information about the current image
#[poise::command(slash_command, rename = "erase-pixel")]
pub async fn erase_pixel(ctx: Context<'_>, x: u32, y: u32) -> Result<(), Error> {
    use image::GenericImageView;
    use image::Rgba;

    let dimension = { ctx.data().image.lock().unwrap().dimensions() };

    if x >= dimension.0 || y >= dimension.1 {
        ctx.send(|msg| {
            msg.ephemeral(true)
                .content("Specified pixel is out of the image boundary.")
        })
        .await?;
        return Ok(());
    }

    {
        ctx.data()
            .image
            .lock()
            .unwrap()
            .put_pixel(x, y, Rgba::<u8>([255, 255, 255, 0]));
    }

    ctx.send(|msg| msg.ephemeral(true).content("Pixel erased!"))
        .await?;
    Ok(())
}
