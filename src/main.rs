mod generate_text;
mod stats;
mod cleanup;

use rusttype::Font;

use serde_derive::Serialize;

use actix::prelude::*;
use actix_web::{
    web,
    web::{Json, Data},
    HttpServer, App
};
use actix_files as fs;

use stats::{IncGeneratedCaptchas, GetStats, StatsActor};
use cleanup::{CleanupActor, AddEntry};

#[derive(Serialize)]
struct Response {
    md5: String,
    token: String,
    url: String
}

async fn new(font: Data<Font<'static>>, stats_addr: Data<Addr<StatsActor>>, cleanup_addr: Data<Addr<CleanupActor>>) -> Json<Response> {
    // File name is md5
    let (captcha_md5, image_md5) = generate_text::generate_text(&font);

    let url = format!("/captchas/{}.png", &image_md5);

    let response = Json(
        Response {
            md5: captcha_md5,
            token: image_md5.clone(),
            url: url
        }
    );

    // Schedule a cleanup job
    cleanup_addr.do_send(AddEntry(image_md5));

    // Increment the amount of generated captchas
    stats_addr.do_send(IncGeneratedCaptchas);

    return response;
}

async fn index(stats_addr: Data<Addr<StatsActor>>) -> String {
    let mut result: String = include_str!("index.txt").to_string();

    let stats = stats_addr.send(GetStats).await.unwrap();

    result += &format!("\nCaptchas generated since last restart: {}", stats.generated_captchas);

    return result;
}

#[actix_rt::main]
async fn main() {
    let _ = std::fs::create_dir("captchas");

    let stats_actor_addr = StatsActor::default().start();
    let cleanup_actor_addr = CleanupActor::default().start();

    HttpServer::new(move || {

        let font = generate_text::make_font();

        App::new()
            .data(stats_actor_addr.clone())
            .data(cleanup_actor_addr.clone())
            .data(font)
            .service(
                fs::Files::new("/captchas", "./captchas")
            )
            .route("/new", web::get().to(new))
            .route("/", web::get().to(index))
    }).bind("0.0.0.0:9093").unwrap().run().await.unwrap();
}
