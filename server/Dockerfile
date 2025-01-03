FROM rust:1.83 as builder

RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev postgresql postgresql-contrib
RUN cargo install diesel_cli@2.1.0 --no-default-features --features postgres

WORKDIR /usr/src/server
COPY . .
#RUN cargo build --release
RUN cargo install --path .

FROM debian:11.6-slim
RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install libpq-dev postgresql postgresql-contrib && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /usr/src/server/migrations /usr/local/bin/migrations
COPY --from=builder /usr/src/server/diesel.toml /usr/local/bin/diesel.toml
COPY --from=builder /usr/src/server/src/schema.rs /usr/local/bin/src/schema.rs
EXPOSE 8080
# Run the application
CMD ["server"]
