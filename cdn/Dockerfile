FROM rust:alpine AS build
WORKDIR /usr/src/cdn/dev

#* Dependencies

RUN apk add build-base 
RUN apk add pkgconfig 
RUN apk add openssl-dev 
RUN apk add openssl-libs-static

#* Build
COPY ./cdn .
COPY ./shared ../shared

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/cdn/prod

COPY --from=build /usr/src/cdn/dev/target/release/saes-cdn /usr/src/cdn/prod/

HEALTHCHECK CMD curl --fail http://localhost:3100 || exit 1
RUN touch .env
EXPOSE 3100
ENTRYPOINT ["./saes-cdn"]
