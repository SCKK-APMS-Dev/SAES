FROM node:18
WORKDIR /usr/src/app

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./package.json ./

RUN npm install -g pnpm
RUN pnpm install

COPY ./ ./

RUN pnpm build

# run the app
USER root
EXPOSE 3000
CMD [ "node", "." ]