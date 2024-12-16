FROM node:22-alpine AS build
RUN npm install pnpm -g
WORKDIR /usr/src/web/dev

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./package.json ./


RUN pnpm ci

COPY ./ ./

RUN pnpm build
RUN npm prune --production

FROM node:22-alpine
WORKDIR /usr/src/web/prod
COPY --from=build /usr/src/web/dev/build /usr/src/web/prod/build
WORKDIR /usr/src/web/prod/build
ENV BODY_SIZE_LIMIT="Infinity"
# run the app
USER root
EXPOSE 3000
CMD [ "node", "." ]