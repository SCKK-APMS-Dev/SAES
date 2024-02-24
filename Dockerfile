FROM node:18
WORKDIR /app
COPY package.json ./
COPY pnpm-lock.yaml ./
RUN npm install -g pnpm
RUN pnpm install -P
COPY . .
RUN pnpm build
EXPOSE 3000
CMD ["node","build"]