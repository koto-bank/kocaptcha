FROM alpine:latest AS build

RUN apk add --no-cache rust cargo fontconfig-dev freetype-dev cmake make pkgconf gcc g++

COPY . /kocaptcha
WORKDIR /kocaptcha

RUN cargo build --release

FROM alpine:latest

COPY --from=build /kocaptcha/target/release/kocaptcha /kocaptcha/kocaptcha

RUN apk add --no-cache fontconfig freetype libgcc ttf-ubuntu-font-family

WORKDIR /kocaptcha

EXPOSE 9093

ENV KOCAPTCHA_FONT_FAMILY=Ubuntu

ENV RUST_BACKTRACE=1

CMD ./kocaptcha
