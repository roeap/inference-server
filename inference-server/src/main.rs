pub mod error;
pub mod handler;

use inference_protocol::inference_service_server::*;
use inference_protocol::*;

use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct InferenceServiceImpl {}

#[tonic::async_trait]
impl InferenceService for InferenceServiceImpl {
    async fn server_live(
        &self,
        request: Request<ServerLiveRequest>,
    ) -> std::result::Result<Response<ServerLiveResponse>, Status> {
        let _live_request = request.into_inner();
        todo!()
    }

    async fn server_ready(
        &self,
        request: Request<ServerReadyRequest>,
    ) -> std::result::Result<Response<ServerReadyResponse>, Status> {
        let _ready_request = request.into_inner();
        todo!()
    }

    async fn model_ready(
        &self,
        request: Request<ModelReadyRequest>,
    ) -> std::result::Result<Response<ModelReadyResponse>, Status> {
        let _ready_request = request.into_inner();
        todo!()
    }

    async fn server_metadata(
        &self,
        request: Request<ServerMetadataRequest>,
    ) -> std::result::Result<Response<ServerMetadataResponse>, Status> {
        let _meta_request = request.into_inner();
        todo!()
    }

    async fn model_metadata(
        &self,
        request: Request<ModelMetadataRequest>,
    ) -> std::result::Result<Response<ModelMetadataResponse>, Status> {
        let _meta_request = request.into_inner();
        todo!()
    }

    async fn model_infer(
        &self,
        request: Request<ModelInferRequest>,
    ) -> std::result::Result<Response<ModelInferResponse>, Status> {
        let _infer_request = request.into_inner();
        todo!()
    }
}

fn main() {
    use tract_onnx::prelude::*;

    let model = onnx()
        .model_for_path("inference-server/tests/data/model.onnx")
        .unwrap()
        .into_runnable()
        .unwrap();

    let data: Vec<f32> = vec![
        1.8, 2.8, 3.8, 1.1, 1.2, 1.3, 1.8, 2.8, 3.8, 1.1, 1.2, 1.3, 1.8,
    ];
    let inputs = tract_ndarray::arr1(&data)
        .into_shape((1, 13))
        .unwrap()
        .into_tensor();
    let result = model.run(tvec![inputs]).unwrap();

    let to_show = result[0].to_array_view::<f32>().unwrap();

    println!("result: {:?}", to_show);
}
