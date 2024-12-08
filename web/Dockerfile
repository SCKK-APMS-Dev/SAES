FROM node:22-alpine
WORKDIR /usr/src/web

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./package.json ./

RUN npm install -g pnpm
RUN pnpm install

COPY ./ ./

RUN pnpm build

ENV BODY_SIZE_LIMIT="Infinity"
# run the app
USER root
EXPOSE 3000
CMD [ "node", "build" ]