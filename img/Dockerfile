FROM rust:alpine AS build
WORKDIR /usr/src/img/dev

#* Előkészítés

RUN apk add build-base 
RUN apk add pkgconfig 
RUN apk add openssl-dev 
RUN apk add openssl-libs-static

#* Build
COPY . .

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/img/prod

COPY --from=build /usr/src/img/dev/target/release/saes-img /usr/src/img/prod/

RUN touch .env
EXPOSE 3100
CMD ["./saes-img"]
