_default:
    just --list

install:
    poetry install
    poetry run pre-commit install

# generate V2 inference API types
generate:
    buf generate buf.build/mlfusion/inference
    buf generate buf.build/mlfusion/mlflow

    rm -rf python/inference-client/inference_client/gen/mlflow
    rm -rf python/inference-client/inference_client/gen/scalapb

    poetry run black python/
    poetry run ruff --fix python/
