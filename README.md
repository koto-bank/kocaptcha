A simple CAPTCHA service with a single API endpoint at `/new` that will give you a JSON output of `md5` (the md5 of the answer)
and `url` (the captcha image). Captchas expire after five minutes and delete themselves.

The files are stored in the captchas directory, which is deleted on exist.

The only dependency seems to be fontconfig to find fonts,
which can be set with the `KOCAPTCHA_FONT_FAMILY` environmental variable.

To run, simply do `KOCAPTCHA_FONT_FAMILY=FontFamily cargo run --release` (if not font family is provided, the first available is used).
This will start the service at `0.0.0.0:8424`.
