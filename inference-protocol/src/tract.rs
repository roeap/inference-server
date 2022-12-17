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
