# BUILD
FROM node:alpine
RUN npm i -g pnpm

WORKDIR /app

COPY ./ .
RUN [ -f .env.local ] && rm .env.local || true

RUN touch .env.production
RUN echo "VITE_MULTIPLAYER_HOST_GAME=wss://mm.codeinthedark.dev" >> .env.production
# RUN echo "VITE_MULTIPLAYER_HOST_GAME=ws://localhost:8081" >> .env.production


RUN pnpm i
RUN npm run build

COPY serve.json /app/dist/serve.json

WORKDIR /app/dist

CMD ["npx", "serve", "-p", "3000"]

