use super::*;
use tract_data::prelude::Tensor;
use tract_onnx::prelude::*;

impl TryFrom<InferInputTensor> for Tensor {
    type Error = TractError;

    fn try_from(value: InferInputTensor) -> Result<Self, Self::Error> {
        let shape = value
            .shape
            .into_iter()
            .map(|val| val as usize)
            .collect::<Vec<_>>();

        let tensor = match value.datatype.to_ascii_lowercase().as_str() {
            "bool" => tract_ndarray::arr1(&value.contents.unwrap().bool_contents)
                .into_shape(shape)?
                .into_tensor(),
            "uint8" | "uint16" | "uint32" => {
                tract_ndarray::arr1(&value.contents.unwrap().uint_contents)
                    .into_shape(shape)?
                    .into_tensor()
            }
            "uint64" => tract_ndarray::arr1(&value.contents.unwrap().uint64_contents)
                .into_shape(shape)?
                .into_tensor(),
            "int8" | "int16" | "int32" => {
                tract_ndarray::arr1(&value.contents.unwrap().int_contents)
                    .into_shape(shape)?
                    .into_tensor()
            }
            "int64" => tract_ndarray::arr1(&value.contents.unwrap().int64_contents)
                .into_shape(shape)?
                .into_tensor(),
            "fp32" => tract_ndarray::arr1(&value.contents.unwrap().fp32_contents)
                .into_shape(shape)?
                .into_tensor(),
            "fp64" => tract_ndarray::arr1(&value.contents.unwrap().fp64_contents)
                .into_shape(shape)?
                .into_tensor(),
            _ => todo!(),
        };

        Ok(tensor)
    }
}

impl TryFrom<ModelInferRequest> for TVec<Tensor> {
    type Error = TractError;

    fn try_from(value: ModelInferRequest) -> Result<Self, Self::Error> {
        value
            .inputs
            .into_iter()
            .map(|t| t.try_into())
            .collect::<TractResult<TVec<Tensor>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_conversion() {
        let values: Vec<f32> = vec![1.1, 1.2, 1.3, 2.1, 2.2, 2.3];
        let input = InferInputTensor {
            name: "tensor".into(),
            datatype: "fp32".into(),
            shape: vec![3, 2],
            parameters: Default::default(),
            contents: Some(InferTensorContents {
                fp32_contents: values.clone(),
                ..Default::default()
            }),
        };
        let reference = tract_ndarray::arr1(&values)
            .into_shape((3, 2))
            .unwrap()
            .into_tensor();
        let tensor: Tensor = input.try_into().unwrap();

        assert_eq!(reference, tensor)
    }

    #[test]
    fn test_infer_request_conversion() {
        let values: Vec<f32> = vec![1.1, 1.2, 1.3, 2.1, 2.2, 2.3];
        let input = InferInputTensor {
            name: "tensor".into(),
            datatype: "fp32".into(),
            shape: vec![3, 2],
            parameters: Default::default(),
            contents: Some(InferTensorContents {
                fp32_contents: values.clone(),
                ..Default::default()
            }),
        };
        let request = ModelInferRequest {
            model_name: "model".into(),
            model_version: "v1".into(),
            inputs: vec![input],
            ..Default::default()
        };

        let reference: TVec<Tensor> = vec![tract_ndarray::arr1(&values)
            .into_shape((3, 2))
            .unwrap()
            .into_tensor()]
        .into();
        let tvec: TVec<Tensor> = request.try_into().unwrap();

        assert_eq!(reference, tvec)
    }
}
