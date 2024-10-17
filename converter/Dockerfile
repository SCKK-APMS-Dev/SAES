FROM rust

WORKDIR /usr/src/converter

COPY . .
COPY ../api ../api

RUN cargo install --path .
RUN touch .env
CMD ["saes-converter"]
