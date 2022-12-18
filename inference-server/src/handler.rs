use crate::error::Result;
use inference_protocol::{ModelInferRequest, ModelInferResponse};
use tract_onnx::prelude::*;

#[tonic::async_trait]
pub trait InferenceHandler: std::fmt::Display + Send + Sync + std::fmt::Debug + 'static {
    async fn predict(&self, request: ModelInferRequest) -> Result<ModelInferResponse>;
}

#[derive(Debug)]
pub struct OnnxInferenceHandler {}

impl std::fmt::Display for OnnxInferenceHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OnnxInferenceHandler()")
    }
}

impl OnnxInferenceHandler {
    pub async fn try_new() -> Result<Self> {
        Ok(Self {})
    }
}

#[tonic::async_trait]
impl InferenceHandler for OnnxInferenceHandler {
    async fn predict(&self, request: ModelInferRequest) -> Result<ModelInferResponse> {
        let model = onnx()
            .model_for_path("inference-server/tests/data/model.onnx")?
            .into_runnable()?;

        // Input the generated data into the model
        let result = model.run(request.try_into()?)?;
        let to_show = result[0].to_array_view::<f32>()?;

        println!("result: {:?}", to_show);

        todo!()
    }
}
