[![Docker Pulls](https://img.shields.io/docker/pulls/vaartis/kocaptcha.svg)](https://hub.docker.com/r/vaartis/kocaptcha)

A simple CAPTCHA service with a single API endpoint at `/new` that will give you a JSON output of `md5` (the md5 of the answer)
and `url` (the captcha image). Captchas expire after five minutes and delete themselves.

The files are stored in the captchas directory, which is deleted on exit.

## Installation

If you don't want to build kocaptcha, you can use the [docker image](https://hub.docker.com/r/vaartis/kocaptcha).

Otherwise, the only external dependency besides [rust](https://www.rust-lang.org/tools/install) itself seems to be `fontconfig` to find fonts.
The font used for captchas can be set with the `KOCAPTCHA_FONT_FAMILY` environmental variable, or the first one provided by fontconfig will be used.

`cd` into the kocaptcha directory and run `cargo build --release`. This will produce a binary, which you'll want to copy wherever you want to
run the service from. You can then run the service bu simply launching the binary
(optionally adding the `KOCAPTCHA_FONT_FAMILY` environmental variable). For example:
`KOCAPTCHA_FONT_FAMILY=FontFamily ./kocaptcha`. This will start the service at `0.0.0.0:9093`.

## Running with a supervisor

You can also use run the service with systemd, openrc, or any other supervising system you'd like, it's fairly straightforward to do.

### OpenRC service example

``` shell
#!/sbin/openrc-run

# Requires OpenRC >= 0.35
directory=~kocaptcha

command=~kocaptcha/kocaptcha
command_user=kocaptcha:kocaptcha
command_background=1

# Ask process to terminate within 30 seconds, otherwise kill it
retry="SIGTERM/30/SIGKILL/5"

pidfile="/var/run/kocaptcha.pid"

depend() {
    need net
}
```

Note that this configuration implies that you have a kocaptcha user in your system and that
user has a home directory, which in turn has the kocaptcha binary. If you don't, change the appropriate
lines in the script to match your system configuration.

### Systemd service example

```
[Unit]
Description=Kocaptcha captcha service
After=network.target

[Service]
ExecStart=/home/kocaptcha/kocaptcha

[Install]
WantedBy = multi-user.target
```

This systemd service implies that the home directory for kocaptcha exists. You can also provide a custom user for it, if needed.


### Nginx configuration

While you can run the service on the `9093` port, it's better to run it on an actual domain. It's
easier to remeber and as an additional benefit, you can configure nginx to redirect users from the http
to the https version.

``` nginx
server {
    listen 80;
    server_name <domain name>;

    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name <domain name>;
    ssl_certificate /etc/letsencrypt/live/<domain name>/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/<domain name>/privkey.pem; # managed by Certbot
    include /etc/letsencrypt/options-ssl-nginx.conf; # managed by Certbot
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem; # managed by Certbot

    location / {
        proxy_pass http://localhost:9093;
    }
}
```

This configuration uses Let's encrypt certbot generated certificates and routes all traffic from your domain to the service's port.
Note that you'll have to install and configure [certbot](https://certbot.eff.org/instructions) for your domain.
