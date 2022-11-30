FROM node:18 as builder

WORKDIR /usr/src/app

COPY package.json yarn.lock ./

RUN yarn install --frozen-lockfile

COPY . .

RUN yarn build

FROM node:18-alpine

WORKDIR /usr/src/app

COPY package.json yarn.lock ./

RUN yarn install --production --frozen-lockfile

COPY --from=builder /usr/src/app/dist ./dist
COPY ./ dest

EXPOSE 8080
CMD [ "node", "dist/index.js" ]