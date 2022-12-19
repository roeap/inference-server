_default:
    just --list

install:
    poetry install
    poetry run pre-commit install

# generate V2 inference API types
generate:
    buf generate buf.build/mlfusion/inference
    buf generate buf.build/mlfusion/mlflow
    cd python && buf generate buf.build/mlfusion/inference

    poetry run black python/
    poetry run ruff --fix python/

run:
    RUST_LOG="debug" cargo run -p inference-server
