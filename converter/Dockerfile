FROM rust:alpine AS build
WORKDIR /usr/src/converter/dev

#* Dependencies

RUN apk add build-base 
RUN apk add pkgconfig 
RUN apk add openssl-dev 
RUN apk add openssl-libs-static

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
