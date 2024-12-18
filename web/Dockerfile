FROM node:22-alpine AS build
WORKDIR /usr/src/web/dev

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./package.json ./

RUN npm install

COPY ./ ./

RUN npm run build
RUN npm prune --omit=dev

FROM node:22-alpine
WORKDIR /usr/src/web/prod

COPY --from=build /usr/src/web/dev/build /usr/src/web/prod/build
COPY --from=build /usr/src/web/dev/node_modules /usr/src/web/prod/node_modules

ENV BODY_SIZE_LIMIT="Infinity"
# run the app
USER root
EXPOSE 3000
CMD [ "node", "build" ]