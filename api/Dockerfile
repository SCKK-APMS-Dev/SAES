FROM rust

WORKDIR /usr/src/api

COPY . .

RUN cargo install --path .
EXPOSE 3000
CMD ["api"]
