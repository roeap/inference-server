//! handlers
use crate::error::ModelServerResult;
use crate::service::InferenceHandler;
use inference_protocol::{
    InferOutputTensor, InferTensorContents, ModelInferRequest, ModelInferResponse,
};
use tract_onnx::prelude::*;

/// Inference handler for ONNX models.
#[derive(Debug)]
pub struct OnnxInferenceHandler {}

impl std::fmt::Display for OnnxInferenceHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OnnxInferenceHandler()")
    }
}

impl OnnxInferenceHandler {
    pub async fn try_new() -> ModelServerResult<Self> {
        Ok(Self {})
    }
}

#[tonic::async_trait]
impl InferenceHandler for OnnxInferenceHandler {
    async fn predict(&self, request: ModelInferRequest) -> ModelServerResult<ModelInferResponse> {
        let model = onnx()
            .model_for_path("inference-server/tests/data/model.onnx")?
            .into_runnable()?;

        // Input the generated data into the model
        let result = model.run(request.try_into()?)?;
        let to_show = result[0].to_array_view::<f32>()?;
        let data = to_show.as_slice().unwrap().to_vec();

        let response = ModelInferResponse {
            model_name: "model_name".into(),
            outputs: vec![InferOutputTensor {
                name: "output".into(),
                datatype: "F32".into(),
                shape: vec![1, 13],
                contents: Some(InferTensorContents {
                    fp32_contents: data,
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        Ok(response)
    }
}
