FROM rust AS build
WORKDIR /usr/src/api/dev

#* Build
COPY ./api/ .
COPY ./shared ../shared

RUN cargo build --release

#* Release

FROM alpine
WORKDIR /usr/src/api/prod

COPY --from=build /usr/src/api/dev/target/release/saes-api /usr/src/api/prod/

RUN touch .env
EXPOSE 3000
ENTRYPOINT ["./saes-api"]
