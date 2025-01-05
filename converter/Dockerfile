FROM rust:alpine AS build
WORKDIR /usr/src/converter/dev

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
WORKDIR /usr/src/converter/prod

COPY --from=build /usr/src/converter/dev/target/release/saes-converter /usr/src/converter/prod/

RUN touch .env
CMD ["./saes-converter"]
