# inference-server

Playground for an experimental inference server based on [V2 inference protocol][v2] and [onnx][onnx].

## Development

### Prerequisites

- [Poetry](https://python-poetry.org/) `>= 1.3`
- [just](https://just.systems/)
- [rust toolchain](https://www.rust-lang.org/tools/install)
- [skaffold](https://skaffold.dev)

### Setup

To install all workspace dependencies, run:

```sh
just install
```

### Local cluster

Large parts of the functionality can only really be evaluated
when running in a kubernetes cluster. For local development,
we use `minikube`.

```sh
just cluster start
```

When done developing, the cluster can be shut down via.

```sh
just cluster stop
```

[v2]: https://kserve.github.io/website/modelserving/inference_api/#grpc
[onnx]: https://onnx.ai/
