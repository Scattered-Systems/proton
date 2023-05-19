FROM jo3mccain/trunkrs:latest as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN trunk build --release

FROM builder as development

ENV TRUNK_SERVE_PORT=80

EXPOSE ${TRUNK_SERVE_PORT}

ENTRYPOINT [ "trunk" ]
CMD [ "serve" ]

FROM nginx:alpine-slim as production

COPY --from=builder /workspace/dist /usr/share/nginx/html