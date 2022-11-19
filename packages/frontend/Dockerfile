FROM node:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN npm install -g npm@9.1.1

FROM builder-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN npm install && npm run build

FROM builder as runner-base

FROM runner-base as development

EXPOSE 5173

CMD ["npm", "run", "dev"]

FROM runner-base as preview

EXPOSE 4173

CMD ["npm", "run", "preview"]