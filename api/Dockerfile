FROM rust:alpine AS build
WORKDIR /usr/src/api/dev

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

RUN mkdir -p /usr/src/api/prod
COPY --from=build /usr/src/api/dev/target/release/saes-api /usr/src/api/prod/

RUN touch .env
EXPOSE 3000
CMD ["/usr/src/api/prod/saes-api"]
