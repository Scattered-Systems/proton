FROM node:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN npm install -g npm@9.1.1

FROM builder-base as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN npm install && npm run build

FROM builder as development

ENV PORT=5173

EXPOSE ${PORT}

CMD ["npm", "run", "dev"]

FROM builder as preview

ENV PORT=4173

EXPOSE 4173:${PORT}

CMD ["npm", "run", "preview"]