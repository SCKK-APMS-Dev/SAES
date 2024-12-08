FROM node:22-alpine AS build
WORKDIR /usr/src/web/dev

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./package.json ./


RUN npm install -g pnpm
RUN pnpm install

COPY ./ ./

RUN pnpm build

FROM node:22-alpine

RUN mkdir -p /usr/src/web/prod
COPY --from=build /usr/src/web/dev/build /usr/src/web/prod/build

ENV BODY_SIZE_LIMIT="Infinity"
# run the app
USER root
EXPOSE 3000
CMD [ "node", "/usr/src/web/prod/build" ]