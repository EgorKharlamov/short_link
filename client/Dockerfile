FROM node:19-alpine as builder

#RUN apt-get update -qq && apt-get install -y build-essential
#RUN apt-get install -y libpq-dev postgresql-client

WORKDIR /app
ENV PATH /app/node_modules/.bin:$PATH
COPY pnpm-lock.yaml ./

RUN npm i -g pnpm
RUN pnpm i
COPY . ./
RUN pnpm run build

FROM nginx:1.16.0-alpine
COPY --from=builder /app/dist /usr/share/nginx/html
RUN rm /etc/nginx/conf.d/default.conf
COPY ./nginx.conf /etc/nginx/conf.d
EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]
