_default:
    just --list

install:
    poetry install
    pre-commit install

# generate V2 inference API types
generate-api:
    buf generate buf.build/mlfusion/inference
