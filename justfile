_default:
    just --list

# generate V2 inference API types
generate-api:
    buf generate buf.build/mlfusion/inference
