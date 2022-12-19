_default:
    just --list

install:
    poetry install
    poetry run pre-commit install

# generate V2 inference API types
generate:
    buf generate buf.build/robstar/inference-protocol
    buf generate buf.build/robstar/mlflow
    cd python && buf generate buf.build/robstar/inference-protocol

    poetry run black python/
    poetry run ruff --fix python/

run:
    RUST_LOG="debug" cargo run -p inference-server
