FROM rust:alpine AS build
WORKDIR /usr/src/cdn/dev

#* Dependencies

RUN apk add --no-cache build-base 
RUN apk add --no-cache pkgconfig 
RUN apk add --no-cache openssl-dev 
RUN apk add --no-cache openssl-libs-static

#* Build
COPY ./cdn .
COPY ./shared ../shared

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/cdn/prod

COPY --from=build /usr/src/cdn/dev/target/release/saes-cdn /usr/src/cdn/prod/

RUN apk add --no-cache curl
HEALTHCHECK CMD curl --fail http://localhost:3100 || exit 1

RUN touch .env
EXPOSE 3100
ENTRYPOINT ["./saes-cdn"]
