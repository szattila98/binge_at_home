FROM node:20.2.0-alpine as builder

WORKDIR /app
COPY . ./

RUN npm ci --no-audit
RUN npm run build

FROM node:20.2.0-alpine

WORKDIR /app

COPY --from=builder /app/package*.json ./
COPY --from=builder /app/.output ./

# clean install dependencies, no devDependencies, no prepare script
RUN npm ci --omit=dev --ignore-scripts --no-audit --loglevel verbose

EXPOSE 3000
CMD ["node", "./server/index.mjs"]
