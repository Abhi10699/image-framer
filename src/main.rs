mod overlay;
mod utils;

use std::io::Write;

use dotenv::{dotenv, var};

use reqwest;
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use overlay::Overlay;

struct Handler;

struct FileResponse {
    bytes_written: usize,
    saved_file_path: String,
}

async fn download_discord_attachment(
    proxy_url: &str,
    file_mimetype: &str,
) -> Result<FileResponse, std::io::Error> {
    let file_response = reqwest::get(proxy_url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let file_type = file_mimetype.split("/").last().unwrap();

    let file_path = format!("temp.{}", file_type);
    let mut temp_file = std::fs::File::create(&file_path).unwrap();
    let bytes_written = temp_file.write(&file_response);
    match bytes_written {
        Ok(bytes) => Ok(FileResponse {
            bytes_written: bytes,
            saved_file_path: file_path,
        }),
        Err(err) => Err(err),
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let message_attachments = msg.attachments;

        if message_attachments.len() == 0 {
            return;
        }

        let first_attachment = message_attachments.first().unwrap();
        let first_attachment_content_type = "image/jpeg";
        let file = download_discord_attachment(
            &first_attachment.proxy_url,
            &first_attachment_content_type,
        )
        .await
        .unwrap();

        // do reel here

        let overlayed_image = Overlay::overlay_white_backdrop(&file.saved_file_path);
        overlayed_image.save("./overlayed.png").unwrap();

        let reel_video = utils::merge_mp4_image(
            String::from("./overlayed.png"),
            &String::from("./video.mp4"),
            String::from("./output.mp4"),
        );

        if reel_video {
            // send back files
            let files = [CreateAttachment::path("./output.mp4").await.unwrap()];

            let builder = CreateMessage::new().content("reel video");
            if let Err(_) = msg.channel_id.send_files(&ctx.http, files, builder).await {
                println!("Error sending attachment");
            };
            return;
        }

        if let Err(_) = msg.channel_id.say(&ctx.http, "Pong!").await {
            println!("Error sending message");
        }
        // msg.channel_id.send_files(cache_http, files, builder);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = var("DISCORD_TOKEN").unwrap();
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
