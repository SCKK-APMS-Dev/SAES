FROM node:22-alpine
WORKDIR /usr/src/app

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./package.json ./

ENV BODY_SIZE_LIMIT="Infinity"

RUN npm install -g pnpm
RUN pnpm install

COPY ./ ./

RUN pnpm build

# run the app
USER root
EXPOSE 3000
CMD [ "node", "build" ]