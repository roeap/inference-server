use std::collections::HashMap;

use mlflow_client::MlFlowClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MlFlowClientBuilder::new()
        .with_allow_http(true)
        .with_service_url("http://127.0.0.1:5000/")
        .build()?;

    let response = client
        .create_registered_model(
            "lightgbm-onnx",
            Some("A model created for developing services."),
            Some(HashMap::from_iter([("tag".into(), "value".into())])),
        )
        .await?;

    println!("crate response: {:?}", response);

    let response = client.get_registered_model("lightgbm-onnx").await?;

    println!("get response: {:?}", response);

    Ok(())
}
