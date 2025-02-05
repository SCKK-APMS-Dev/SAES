FROM rust:alpine AS build
WORKDIR /usr/src/api/dev

#* Dependencies

RUN apk add build-base 
RUN apk add pkgconfig 
RUN apk add openssl-dev 
RUN apk add openssl-libs-static

#* Build
COPY ./api/ .
COPY ./shared ../shared

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/api/prod

ARG ENV_MODE
ENV ENV_MODE=${ENV_MODE}

COPY --from=build /usr/src/api/dev/target/release/saes-api /usr/src/api/prod/

HEALTHCHECK CMD curl --fail http://localhost:3000 || exit 1
RUN touch .env
EXPOSE 3000
ENTRYPOINT ["./saes-api"]
