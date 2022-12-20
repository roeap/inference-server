# inference-server

Playground for an experimental inference server based on [V2 inference protocol][v2] and the [onnx][onnx].

## Development

### Prerequisites

- [Poetry](https://python-poetry.org/) `>= 1.3`
- [just](https://just.systems/)
- [rust toolchain](https://www.rust-lang.org/tools/install)
- [skaffold](https://skaffold.dev)

```sh
just install
```

[v2]: https://kserve.github.io/website/modelserving/inference_api/#grpc
[onnx]: https://onnx.ai/
