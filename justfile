_default:
    just --list

install:
    poetry install
    poetry run pre-commit install

# generate V2 inference API types
generate:
    cd inference-protocol && buf generate buf.build/robstar/inference-protocol
    cd inference-protocol && buf generate buf.build/robstar/mlflow
    cd mlflow-client && buf generate buf.build/robstar/mlflow
    cd python && buf generate buf.build/robstar/inference-protocol

    poetry run black python/
    poetry run ruff --fix python/

_run-server:
    RUST_LOG="debug" cargo run -p inference-server

_run-mlflow:
    mkdir --parents .mlflow
    poetry run mlflow ui --host "127.0.0.1" --port 5000 \
        --backend-store-uri "sqlite:///.mlflow/mlruns.sqlite" \
        --default-artifact-root "file://{{ justfile_directory() }}/.mlflow/mlruns"

run command:
    just _run-{{ command }}

docker tag:
    docker build -t inference-server:{{ tag }} .
