_default:
    just --list

generate-api:
    buf generate buf.build/mlfusion/inference
