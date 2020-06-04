extern crate font_loader;
extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate rand;
extern crate crypto;

use self::font_loader::system_fonts;
use self::rusttype::{Font, Scale};
use self::image::{Rgb, RgbImage, Pixel};
use self::imageproc::drawing::draw_text_mut;
use self::imageproc::noise::salt_and_pepper_noise_mut;
use self::imageproc::affine::*;

use self::rand::{thread_rng, Rng};
use self::rand::distributions::Alphanumeric;

use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn make_font() -> Font<'static> {
    let family = std::env::var("KOCAPTCHA_FONT_FAMILY").expect("Expected KOCAPTCHA_FONT_FAMILY to be set");

    let prop = system_fonts::FontPropertyBuilder::new().family(&family).build();
    let (font_data, _) = system_fonts::get(&prop).unwrap();

    Font::from_bytes(font_data).unwrap()
}

pub fn generate_text(font: &Font) -> (String, String) {
    let mut image = RgbImage::new(400, 150);
    let (w, h) = (image.width(), image.height());

    let font_h = h / 3;

    let mut rng = thread_rng();

    let count = rng.gen_range(6, 8);
    let text: String = rng.sample_iter(&Alphanumeric).take(count).collect();

    // dividing by 5 seems to make it look ok-ish and gives some space
    let scale = Scale { x: (w / 5) as f32, y: font_h as f32 };
    draw_text_mut(
        &mut image,
        Rgb([255, 255, 255]), // white
        w / 8,
        0, // Seems to be roughly in the middle
        scale,
        &font,
        &text
    );

    image = affine(
        &image,
        Affine2::from_matrix_unchecked([
            1.0, rng.gen_range(-1.0, 1.0), 0.0,
            rng.gen_range(0.1, 0.2), 1.0, 0.0,
            0.0, 1.0, 1.0
        ]),
        Interpolation::Bilinear
    ).unwrap();

    salt_and_pepper_noise_mut(
        &mut image,
        0.5,
        rng.gen()
    );
    image.enumerate_pixels_mut().for_each(|(_, _, pix)| pix.invert()); // Invert colors to make text black;

    let mut md5 = Md5::new();
    md5.input_str(&text);
    let md5_s = md5.result_str();
    md5.reset();

    md5.input(&image.clone().into_raw());
    let md5_i_s = md5.result_str();

    let _ = image.save(format!("./captchas/{}.png", md5_i_s)).unwrap();

    return (md5_s, md5_i_s);
}
