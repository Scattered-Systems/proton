FROM scsys/wasm:trunk as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .

RUN trunk build

FROM builder

EXPOSE 8080

CMD [ "trunk", "serve" ]
