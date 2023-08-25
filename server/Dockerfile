FROM node:alpine

COPY . /server
WORKDIR /server

RUN rm .env

RUN npm install

CMD ["npm", "start"]
