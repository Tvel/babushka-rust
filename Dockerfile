FROM rust:1.47-buster as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/
RUN mkdir /app
COPY --from=builder /usr/src/myapp/target/release/babushka /app/babushka
ENV DISCORD_PREFIX !
ENV DISCORD_TOKEN changeme
ENV CATAPIKEY changeme
WORKDIR /app
CMD ["./babushka"]