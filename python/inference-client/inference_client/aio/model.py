from inference_client.gen.inference import ModelInferRequestInferInputTensor, ModelInferResponse, ModelMetadataResponse

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
        _input = ModelInferRequestInferInputTensor(name="field", datatype="fp32", shape=[3, 5])
