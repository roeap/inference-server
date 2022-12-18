from inference_client.gen.inference import (
    InferTensorContents,
    ModelInferRequestInferInputTensor,
    ModelInferResponse,
    ModelMetadataResponse,
)

from ._utils import run_async
from .services import AsyncInferenceClient


class AsyncModelClient:
    def __init__(self, client: AsyncInferenceClient, name: str, id: str = "", version: str = "") -> None:
        self._client = client
        self._id = id
        self._name = name
        self._version = version

    @property
    def metadata(self) -> ModelMetadataResponse:
        if self._metadata is None:
            self._metadata = run_async(self._client.model_metadata, name=self._name, version=self._version)
        return self._metadata

    async def predict(self) -> ModelInferResponse:
        input = ModelInferRequestInferInputTensor(
            name="field",
            datatype="fp32",
            shape=[1, 13],
            contents=InferTensorContents(
                fp32_contents=[
                    1.8,
                    2.8,
                    3.8,
                    1.1,
                    1.2,
                    1.3,
                    1.8,
                    2.8,
                    3.8,
                    1.1,
                    1.2,
                    1.3,
                    1.8,
                ]
            ),
        )
        return await self._client.model_infer(
            model_name=self._name, model_version=self._version, id=self._id, inputs=[input]
        )
