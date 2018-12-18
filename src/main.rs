mod generate_text;
#[cfg(feature = "generated-captchas-stat")]
mod generated_captchas_stat;

use std::time::Duration;
use std::thread;

use serde_derive::Serialize;

use actix_web::{server, fs, Json,  App, HttpRequest};

#[derive(Serialize)]
struct Response {
    md5: String,
    token: String,
    url: String
}

fn new(_req: &HttpRequest) -> Json<Response> {
    let font_family = std::env::var("KOCAPTCHA_FONT_FAMILY").ok();

    // File name is md5
    let (captcha_md5, image_md5) = generate_text::generate_text(font_family);

    let response = Json(
        Response {
            md5: captcha_md5.clone(),
            token: image_md5.clone(),
            url: format!("/captchas/{}.png", image_md5.clone())
        }
    );

    let _ = thread::spawn(move || {
        thread::sleep(Duration::new(60 * 5, 0)); // sleep ~5 minutes
        let _ = std::fs::remove_file(format!("./captchas/{}.png", image_md5.clone()));
    });

    // Optionally add 1 to the stat thing
    #[cfg(feature = "generated-captchas-stat")]
    generated_captchas_stat::inc();

    return response;
}

fn index(_req: &HttpRequest) -> String {
    #[allow(unused_mut)]
    let mut result: String = include_str!("index.txt").to_string();

    #[cfg(feature = "generated-captchas-stat")]
    {
        let gend = generated_captchas_stat::get();

        result += &format!("\nCaptchas generated since last restart: {}", gend)
    }

    return result;
}

fn main() {
    let _ = std::fs::create_dir("captchas");

    server::new(||
                App::new()
                .handler(
                    "/captchas",
                    fs::StaticFiles::new("./captchas")
                        .unwrap()
                )
                .resource("/new", |r| r.f(new))
                .resource("/", |r| r.f(index))
                .finish()
    ).bind("0.0.0.0:9093").unwrap().run();

    // Remove captcha directory if there are still any there
    let _ = std::fs::remove_dir_all("./captchas");
}
