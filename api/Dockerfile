FROM rust:alpine
WORKDIR /usr/src/api

#* ALPINE Cuccok

RUN apk add build-base 
RUN apk add pkgconfig 
RUN apk add openssl-dev 
RUN apk add openssl-libs-static

#* Projekt

COPY . .

RUN cargo install --path .
RUN touch .env
EXPOSE 3000
CMD ["saes-api"]
