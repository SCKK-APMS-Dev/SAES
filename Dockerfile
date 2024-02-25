FROM node:18
WORKDIR /api

# install dependencies into temp directory
# this will cache them and speed up future builds

COPY ./api/package.json ./

RUN npm install

COPY ./api .

# run the app
USER root
EXPOSE 3000
ENTRYPOINT [ "pnpm", "start"]