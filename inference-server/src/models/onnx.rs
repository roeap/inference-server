//! handlers
use std::sync::Arc;

use crate::error::ModelServerResult;
use crate::models::InferenceHandler;

use inference_protocol::{
    InferOutputTensor, InferTensorContents, ModelInferRequest, ModelInferResponse,
};
use tract_hir::infer::InferenceOp;
use tract_onnx::prelude::*;

/// Inference handler for ONNX models.
#[derive(Debug, Clone)]
pub struct OnnxInferenceHandler {
    model:
        SimplePlan<InferenceFact, Box<dyn InferenceOp>, Graph<InferenceFact, Box<dyn InferenceOp>>>,
}

impl std::fmt::Display for OnnxInferenceHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OnnxInferenceHandler()")
    }
}

impl OnnxInferenceHandler {
    pub async fn try_new(path: impl AsRef<std::path::Path>) -> ModelServerResult<Self> {
        let model = onnx().model_for_path(path)?.into_runnable()?;
        Ok(Self { model })
    }
}

#[tonic::async_trait]
impl InferenceHandler for OnnxInferenceHandler {
    async fn predict(&self, request: ModelInferRequest) -> ModelServerResult<ModelInferResponse> {
        // Input the generated data into the model
        let result = self.model.run(request.try_into()?)?;
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
