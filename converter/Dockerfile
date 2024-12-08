FROM rust:alpine
WORKDIR /usr/src/converter

#* ALPINE Cuccok

RUN apk add build-base 
RUN apk add pkgconfig 
RUN apk add openssl-dev 
RUN apk add openssl-libs-static

#* Projekt

COPY . .

RUN cargo install --path .
RUN touch .env
CMD ["saes-converter"]
