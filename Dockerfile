FROM node:14.15.4 as frontend
WORKDIR /usr/src/frontend
RUN mkdir -p public/
COPY frontend/package.json frontend/yarn.lock ./
RUN yarn
COPY frontend ./frontend
WORKDIR /usr/src/frontend/frontend
RUN yarn release

FROM rust:1.62 as backend

WORKDIR /guilder

COPY src/ ./src
COPY Cargo.toml .
COPY Cargo.lock .
COPY migration ./migration
COPY entity ./entity
COPY data_sources ./data_sources

RUN cargo build --release

FROM debian:buster-slim
ARG APP=/guilder

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata curl \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=guilder

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

RUN mkdir -p ${APP}/public
COPY --from=backend /guilder/target/release/guilder ${APP}/guilder
COPY --from=frontend /usr/src/frontend/public ${APP}/public

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
ENV PATH="/guilder:${PATH}"
HEALTHCHECK CMD curl --fail http://localhost:8000/api/health || exit 1
CMD ["guilder"]
