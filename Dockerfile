FROM rust:1.66 as build-env

RUN apt-get update && apt-get install -y cmake curl unzip

WORKDIR /app
COPY ./inference-server /app/inference-server
COPY ./inference-protocol /app/inference-protocol
COPY ./mlflow-client /app/mlflow-client
COPY Cargo.toml Cargo.lock /app/
RUN cargo build --package inference-server --release --locked

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/inference-server /opt/app/

WORKDIR /app/

CMD ["./inference-server"]
