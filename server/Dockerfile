FROM node:alpine

COPY . /server
WORKDIR /server

RUN [ -f .env ] && rm .env || true

RUN npm install

CMD ["npm", "start"]
