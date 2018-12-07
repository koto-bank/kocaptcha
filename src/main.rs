extern crate actix_web;
#[macro_use] extern crate serde_derive;

mod generate_text;

use std::time::Duration;
use std::thread;

use actix_web::{server, fs, Json,  App, HttpRequest};

#[derive(Serialize)]
struct Response {
    md5: String,
    url: String
}

fn new(_req: &HttpRequest) -> Json<Response> {
    let font_family = std::env::var("KOCAPTCHA_FONT_FAMILY").ok();

    // File name is md5
    let captcha_md5 = generate_text::generate_text(font_family);

    let response = Json(Response { md5: captcha_md5.clone(), url: format!("/captchas/{}.png", captcha_md5.clone()) });

    let _ = thread::spawn(move || {
        thread::sleep(Duration::new(60 * 5, 0)); // sleep ~5 minutes
        let _ = std::fs::remove_file(format!("./captchas/{}.png", captcha_md5.clone()));
    });

    return response;
}

fn index(_req: &HttpRequest) -> &'static str {
    return "\
Head to /new for a captcha. The response will be in JSON format, with md5 field having the md5 of the answer and link field having a relative link to the image.
The captcha will expire and the image will be removed withing 5 minutes.
";
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
    ).bind("0.0.0.0:8424").unwrap().run();

    // Remove captcha directory if there are still any there
    let _ = std::fs::remove_dir_all("./captchas");
}
