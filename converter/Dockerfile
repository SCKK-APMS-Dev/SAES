FROM rust

WORKDIR /usr/src/converter

COPY . .

RUN cargo install --path .
RUN touch .env
CMD ["saes-converter"]
