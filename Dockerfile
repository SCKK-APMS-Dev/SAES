FROM oven/bun:1 as base
WORKDIR /app

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./api/package.json ./
COPY ./api/bun.lockb ./

RUN bun install

COPY ./api .

# run the app
USER root
EXPOSE 8080
ENTRYPOINT [ "bun", "start"]