_default:
    just --list

generate_api:
    buf generate buf.build/mlfusion/inference
