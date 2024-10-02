FROM rust

WORKDIR /usr/src/gbot

COPY . .

RUN cargo install --path .
RUN touch .env
EXPOSE 3000
CMD ["saes-gbot"]