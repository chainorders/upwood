FROM node:latest as builder
COPY ./ /frontend
WORKDIR /frontend
RUN yarn install
RUN yarn build

FROM nginx:alpine
COPY --from=builder /frontend/dist /usr/share/nginx/html
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]