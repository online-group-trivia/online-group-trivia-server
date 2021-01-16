FROM rust as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/app /usr/local/bin/app
EXPOSE 9631
CMD ["app"]