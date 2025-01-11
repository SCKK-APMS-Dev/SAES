FROM rust AS build
WORKDIR /usr/src/static/dev

#* Build
COPY ./static .
COPY ./shared ../shared

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/static/prod

COPY --from=build /usr/src/static/dev/target/release/saes-static /usr/src/static/prod/

RUN touch .env
EXPOSE 3100
ENTRYPOINT ["./saes-static"]
