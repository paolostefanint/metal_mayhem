# BUILD
FROM node:alpine
RUN npm i -g pnpm

WORKDIR /app

COPY ./ .
RUN rm .env.local

RUN touch .env.production
RUN echo "VITE_MULTIPLAYER_HOST_GAME=ws://localhost:7000" >> .env.production


RUN pnpm i
RUN npm run build

COPY serve.json /app/dist/serve.json
COPY src/assets/logo_square.png /app/dist/assets/logo_square.png
COPY src/assets/drone.png /app/dist/assets/drone.png

WORKDIR /app/dist

CMD ["npx", "serve", "-p", "3000"]

