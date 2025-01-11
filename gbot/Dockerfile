FROM rust AS build
WORKDIR /usr/src/gbot/dev

#* Build

COPY ./gbot .

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/gbot/prod

COPY --from=build /usr/src/gbot/dev/target/release/saes-gbot /usr/src/gbot/prod/

RUN touch .env
EXPOSE 3000
ENTRYPOINT ["./saes-gbot"]