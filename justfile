_default:
    just --list

install:
    poetry install

# generate V2 inference API types
generate-api:
    buf generate buf.build/mlfusion/inference
