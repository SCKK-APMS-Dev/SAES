FROM rust AS build
WORKDIR /usr/src/converter/dev

#* Build

COPY ./converter .
COPY ./shared ../shared

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/converter/prod

COPY --from=build /usr/src/converter/dev/target/release/saes-converter /usr/src/converter/prod/

RUN touch .env
ENTRYPOINT ["./saes-converter"]
