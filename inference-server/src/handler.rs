use crate::error::*;
use inference_protocol::*;
use tract_onnx::prelude::*;

#[tonic::async_trait]
pub trait InferenceHandler {
    async fn infer(request: ModelInferRequest) -> Result<ModelInferResponse>;
}

pub struct OnnxInferenceHandler {}

#[tonic::async_trait]
impl InferenceHandler for OnnxInferenceHandler {
    async fn infer(request: ModelInferRequest) -> Result<ModelInferResponse> {
        let model = onnx()
            .model_for_path("/home/robstar/github/datafusion-onnx/notebooks/rf_iris.onnx")?
            .into_runnable()?;

        // Input the generated data into the model
        let result = model.run(request.try_into()?)?;
        let to_show = result[0].to_array_view::<i64>()?;

        println!("result: {:?}", to_show);

        todo!()
    }
}
