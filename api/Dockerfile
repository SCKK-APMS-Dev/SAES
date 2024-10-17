FROM rust

WORKDIR /usr/src/api

COPY . .

RUN cargo install --path .
RUN touch .env
EXPOSE 3000
CMD ["saes-api"]
