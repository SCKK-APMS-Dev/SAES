FROM rust:alpine AS build
WORKDIR /usr/src/static/dev

#* Előkészítés

RUN apk add build-base 
RUN apk add pkgconfig 
RUN apk add openssl-dev 
RUN apk add openssl-libs-static

#* Build
COPY . .
COPY ../shared /usr/src/static/dev/shared

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/static/prod

COPY --from=build /usr/src/static/dev/target/release/saes-static /usr/src/static/prod/

RUN touch .env
EXPOSE 3100
CMD ["./saes-static"]
