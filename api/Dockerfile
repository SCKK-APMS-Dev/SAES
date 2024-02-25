FROM oven/bun:1 as base
WORKDIR /app

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY package.json ./
COPY bun.lockb ./

RUN bun install

COPY . .

# run the app
USER root
EXPOSE 8080
ENTRYPOINT [ "bun", "start"]