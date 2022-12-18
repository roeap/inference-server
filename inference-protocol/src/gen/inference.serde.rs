// @generated
impl serde::Serialize for InferParameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.parameter_choice.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.InferParameter", len)?;
        if let Some(v) = self.parameter_choice.as_ref() {
            match v {
                infer_parameter::ParameterChoice::BoolParam(v) => {
                    struct_ser.serialize_field("boolParam", v)?;
                }
                infer_parameter::ParameterChoice::Int64Param(v) => {
                    struct_ser.serialize_field("int64Param", ToString::to_string(&v).as_str())?;
                }
                infer_parameter::ParameterChoice::StringParam(v) => {
                    struct_ser.serialize_field("stringParam", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferParameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bool_param",
            "boolParam",
            "int64_param",
            "int64Param",
            "string_param",
            "stringParam",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BoolParam,
            Int64Param,
            StringParam,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "boolParam" | "bool_param" => Ok(GeneratedField::BoolParam),
                            "int64Param" | "int64_param" => Ok(GeneratedField::Int64Param),
                            "stringParam" | "string_param" => Ok(GeneratedField::StringParam),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InferParameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.InferParameter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InferParameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parameter_choice__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BoolParam => {
                            if parameter_choice__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolParam"));
                            }
                            parameter_choice__ = map.next_value::<::std::option::Option<_>>()?.map(infer_parameter::ParameterChoice::BoolParam);
                        }
                        GeneratedField::Int64Param => {
                            if parameter_choice__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Param"));
                            }
                            parameter_choice__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| infer_parameter::ParameterChoice::Int64Param(x.0));
                        }
                        GeneratedField::StringParam => {
                            if parameter_choice__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stringParam"));
                            }
                            parameter_choice__ = map.next_value::<::std::option::Option<_>>()?.map(infer_parameter::ParameterChoice::StringParam);
                        }
                    }
                }
                Ok(InferParameter {
                    parameter_choice: parameter_choice__,
                })
            }
        }
        deserializer.deserialize_struct("inference.InferParameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InferTensorContents {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bool_contents.is_empty() {
            len += 1;
        }
        if !self.int_contents.is_empty() {
            len += 1;
        }
        if !self.int64_contents.is_empty() {
            len += 1;
        }
        if !self.uint_contents.is_empty() {
            len += 1;
        }
        if !self.uint64_contents.is_empty() {
            len += 1;
        }
        if !self.fp32_contents.is_empty() {
            len += 1;
        }
        if !self.fp64_contents.is_empty() {
            len += 1;
        }
        if !self.bytes_contents.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.InferTensorContents", len)?;
        if !self.bool_contents.is_empty() {
            struct_ser.serialize_field("boolContents", &self.bool_contents)?;
        }
        if !self.int_contents.is_empty() {
            struct_ser.serialize_field("intContents", &self.int_contents)?;
        }
        if !self.int64_contents.is_empty() {
            struct_ser.serialize_field("int64Contents", &self.int64_contents.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.uint_contents.is_empty() {
            struct_ser.serialize_field("uintContents", &self.uint_contents)?;
        }
        if !self.uint64_contents.is_empty() {
            struct_ser.serialize_field("uint64Contents", &self.uint64_contents.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.fp32_contents.is_empty() {
            struct_ser.serialize_field("fp32Contents", &self.fp32_contents)?;
        }
        if !self.fp64_contents.is_empty() {
            struct_ser.serialize_field("fp64Contents", &self.fp64_contents)?;
        }
        if !self.bytes_contents.is_empty() {
            struct_ser.serialize_field("bytesContents", &self.bytes_contents.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InferTensorContents {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bool_contents",
            "boolContents",
            "int_contents",
            "intContents",
            "int64_contents",
            "int64Contents",
            "uint_contents",
            "uintContents",
            "uint64_contents",
            "uint64Contents",
            "fp32_contents",
            "fp32Contents",
            "fp64_contents",
            "fp64Contents",
            "bytes_contents",
            "bytesContents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BoolContents,
            IntContents,
            Int64Contents,
            UintContents,
            Uint64Contents,
            Fp32Contents,
            Fp64Contents,
            BytesContents,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "boolContents" | "bool_contents" => Ok(GeneratedField::BoolContents),
                            "intContents" | "int_contents" => Ok(GeneratedField::IntContents),
                            "int64Contents" | "int64_contents" => Ok(GeneratedField::Int64Contents),
                            "uintContents" | "uint_contents" => Ok(GeneratedField::UintContents),
                            "uint64Contents" | "uint64_contents" => Ok(GeneratedField::Uint64Contents),
                            "fp32Contents" | "fp32_contents" => Ok(GeneratedField::Fp32Contents),
                            "fp64Contents" | "fp64_contents" => Ok(GeneratedField::Fp64Contents),
                            "bytesContents" | "bytes_contents" => Ok(GeneratedField::BytesContents),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InferTensorContents;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.InferTensorContents")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<InferTensorContents, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bool_contents__ = None;
                let mut int_contents__ = None;
                let mut int64_contents__ = None;
                let mut uint_contents__ = None;
                let mut uint64_contents__ = None;
                let mut fp32_contents__ = None;
                let mut fp64_contents__ = None;
                let mut bytes_contents__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BoolContents => {
                            if bool_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("boolContents"));
                            }
                            bool_contents__ = Some(map.next_value()?);
                        }
                        GeneratedField::IntContents => {
                            if int_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("intContents"));
                            }
                            int_contents__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Int64Contents => {
                            if int64_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("int64Contents"));
                            }
                            int64_contents__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::UintContents => {
                            if uint_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uintContents"));
                            }
                            uint_contents__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Uint64Contents => {
                            if uint64_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uint64Contents"));
                            }
                            uint64_contents__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Fp32Contents => {
                            if fp32_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fp32Contents"));
                            }
                            fp32_contents__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Fp64Contents => {
                            if fp64_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fp64Contents"));
                            }
                            fp64_contents__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::BytesContents => {
                            if bytes_contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bytesContents"));
                            }
                            bytes_contents__ =
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(InferTensorContents {
                    bool_contents: bool_contents__.unwrap_or_default(),
                    int_contents: int_contents__.unwrap_or_default(),
                    int64_contents: int64_contents__.unwrap_or_default(),
                    uint_contents: uint_contents__.unwrap_or_default(),
                    uint64_contents: uint64_contents__.unwrap_or_default(),
                    fp32_contents: fp32_contents__.unwrap_or_default(),
                    fp64_contents: fp64_contents__.unwrap_or_default(),
                    bytes_contents: bytes_contents__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.InferTensorContents", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelInferRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.model_name.is_empty() {
            len += 1;
        }
        if !self.model_version.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if !self.inputs.is_empty() {
            len += 1;
        }
        if !self.outputs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelInferRequest", len)?;
        if !self.model_name.is_empty() {
            struct_ser.serialize_field("modelName", &self.model_name)?;
        }
        if !self.model_version.is_empty() {
            struct_ser.serialize_field("modelVersion", &self.model_version)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if !self.inputs.is_empty() {
            struct_ser.serialize_field("inputs", &self.inputs)?;
        }
        if !self.outputs.is_empty() {
            struct_ser.serialize_field("outputs", &self.outputs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelInferRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_name",
            "modelName",
            "model_version",
            "modelVersion",
            "id",
            "parameters",
            "inputs",
            "outputs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelName,
            ModelVersion,
            Id,
            Parameters,
            Inputs,
            Outputs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "modelName" | "model_name" => Ok(GeneratedField::ModelName),
                            "modelVersion" | "model_version" => Ok(GeneratedField::ModelVersion),
                            "id" => Ok(GeneratedField::Id),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "inputs" => Ok(GeneratedField::Inputs),
                            "outputs" => Ok(GeneratedField::Outputs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelInferRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelInferRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelInferRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_name__ = None;
                let mut model_version__ = None;
                let mut id__ = None;
                let mut parameters__ = None;
                let mut inputs__ = None;
                let mut outputs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModelName => {
                            if model_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelName"));
                            }
                            model_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModelVersion => {
                            if model_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelVersion"));
                            }
                            model_version__ = Some(map.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Inputs => {
                            if inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputs"));
                            }
                            inputs__ = Some(map.next_value()?);
                        }
                        GeneratedField::Outputs => {
                            if outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputs"));
                            }
                            outputs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ModelInferRequest {
                    model_name: model_name__.unwrap_or_default(),
                    model_version: model_version__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    inputs: inputs__.unwrap_or_default(),
                    outputs: outputs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelInferRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for model_infer_request::InferInputTensor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.datatype.is_empty() {
            len += 1;
        }
        if !self.shape.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if self.contents.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelInferRequest.InferInputTensor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.datatype.is_empty() {
            struct_ser.serialize_field("datatype", &self.datatype)?;
        }
        if !self.shape.is_empty() {
            struct_ser.serialize_field("shape", &self.shape.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if let Some(v) = self.contents.as_ref() {
            struct_ser.serialize_field("contents", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for model_infer_request::InferInputTensor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "datatype",
            "shape",
            "parameters",
            "contents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Datatype,
            Shape,
            Parameters,
            Contents,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "datatype" => Ok(GeneratedField::Datatype),
                            "shape" => Ok(GeneratedField::Shape),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "contents" => Ok(GeneratedField::Contents),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = model_infer_request::InferInputTensor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelInferRequest.InferInputTensor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<model_infer_request::InferInputTensor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut datatype__ = None;
                let mut shape__ = None;
                let mut parameters__ = None;
                let mut contents__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Datatype => {
                            if datatype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("datatype"));
                            }
                            datatype__ = Some(map.next_value()?);
                        }
                        GeneratedField::Shape => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            shape__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Contents => {
                            if contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contents"));
                            }
                            contents__ = map.next_value()?;
                        }
                    }
                }
                Ok(model_infer_request::InferInputTensor {
                    name: name__.unwrap_or_default(),
                    datatype: datatype__.unwrap_or_default(),
                    shape: shape__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    contents: contents__,
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelInferRequest.InferInputTensor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for model_infer_request::InferRequestedOutputTensor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelInferRequest.InferRequestedOutputTensor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for model_infer_request::InferRequestedOutputTensor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = model_infer_request::InferRequestedOutputTensor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelInferRequest.InferRequestedOutputTensor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<model_infer_request::InferRequestedOutputTensor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut parameters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(model_infer_request::InferRequestedOutputTensor {
                    name: name__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelInferRequest.InferRequestedOutputTensor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelInferResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.model_name.is_empty() {
            len += 1;
        }
        if !self.model_version.is_empty() {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if !self.outputs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelInferResponse", len)?;
        if !self.model_name.is_empty() {
            struct_ser.serialize_field("modelName", &self.model_name)?;
        }
        if !self.model_version.is_empty() {
            struct_ser.serialize_field("modelVersion", &self.model_version)?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if !self.outputs.is_empty() {
            struct_ser.serialize_field("outputs", &self.outputs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelInferResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_name",
            "modelName",
            "model_version",
            "modelVersion",
            "id",
            "parameters",
            "outputs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelName,
            ModelVersion,
            Id,
            Parameters,
            Outputs,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "modelName" | "model_name" => Ok(GeneratedField::ModelName),
                            "modelVersion" | "model_version" => Ok(GeneratedField::ModelVersion),
                            "id" => Ok(GeneratedField::Id),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "outputs" => Ok(GeneratedField::Outputs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelInferResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelInferResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelInferResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_name__ = None;
                let mut model_version__ = None;
                let mut id__ = None;
                let mut parameters__ = None;
                let mut outputs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModelName => {
                            if model_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelName"));
                            }
                            model_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModelVersion => {
                            if model_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelVersion"));
                            }
                            model_version__ = Some(map.next_value()?);
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Outputs => {
                            if outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputs"));
                            }
                            outputs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ModelInferResponse {
                    model_name: model_name__.unwrap_or_default(),
                    model_version: model_version__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    outputs: outputs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelInferResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for model_infer_response::InferOutputTensor {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.datatype.is_empty() {
            len += 1;
        }
        if !self.shape.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        if self.contents.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelInferResponse.InferOutputTensor", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.datatype.is_empty() {
            struct_ser.serialize_field("datatype", &self.datatype)?;
        }
        if !self.shape.is_empty() {
            struct_ser.serialize_field("shape", &self.shape.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        if let Some(v) = self.contents.as_ref() {
            struct_ser.serialize_field("contents", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for model_infer_response::InferOutputTensor {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "datatype",
            "shape",
            "parameters",
            "contents",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Datatype,
            Shape,
            Parameters,
            Contents,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "datatype" => Ok(GeneratedField::Datatype),
                            "shape" => Ok(GeneratedField::Shape),
                            "parameters" => Ok(GeneratedField::Parameters),
                            "contents" => Ok(GeneratedField::Contents),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = model_infer_response::InferOutputTensor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelInferResponse.InferOutputTensor")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<model_infer_response::InferOutputTensor, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut datatype__ = None;
                let mut shape__ = None;
                let mut parameters__ = None;
                let mut contents__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Datatype => {
                            if datatype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("datatype"));
                            }
                            datatype__ = Some(map.next_value()?);
                        }
                        GeneratedField::Shape => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            shape__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Contents => {
                            if contents__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contents"));
                            }
                            contents__ = map.next_value()?;
                        }
                    }
                }
                Ok(model_infer_response::InferOutputTensor {
                    name: name__.unwrap_or_default(),
                    datatype: datatype__.unwrap_or_default(),
                    shape: shape__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                    contents: contents__,
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelInferResponse.InferOutputTensor", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelMetadataRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelMetadataRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelMetadataRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelMetadataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelMetadataRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelMetadataRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ModelMetadataRequest {
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelMetadataRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.versions.is_empty() {
            len += 1;
        }
        if !self.platform.is_empty() {
            len += 1;
        }
        if !self.inputs.is_empty() {
            len += 1;
        }
        if !self.outputs.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelMetadataResponse", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.versions.is_empty() {
            struct_ser.serialize_field("versions", &self.versions)?;
        }
        if !self.platform.is_empty() {
            struct_ser.serialize_field("platform", &self.platform)?;
        }
        if !self.inputs.is_empty() {
            struct_ser.serialize_field("inputs", &self.inputs)?;
        }
        if !self.outputs.is_empty() {
            struct_ser.serialize_field("outputs", &self.outputs)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelMetadataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "versions",
            "platform",
            "inputs",
            "outputs",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Versions,
            Platform,
            Inputs,
            Outputs,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "versions" => Ok(GeneratedField::Versions),
                            "platform" => Ok(GeneratedField::Platform),
                            "inputs" => Ok(GeneratedField::Inputs),
                            "outputs" => Ok(GeneratedField::Outputs),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelMetadataResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelMetadataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut versions__ = None;
                let mut platform__ = None;
                let mut inputs__ = None;
                let mut outputs__ = None;
                let mut parameters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Versions => {
                            if versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versions"));
                            }
                            versions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Platform => {
                            if platform__.is_some() {
                                return Err(serde::de::Error::duplicate_field("platform"));
                            }
                            platform__ = Some(map.next_value()?);
                        }
                        GeneratedField::Inputs => {
                            if inputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputs"));
                            }
                            inputs__ = Some(map.next_value()?);
                        }
                        GeneratedField::Outputs => {
                            if outputs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputs"));
                            }
                            outputs__ = Some(map.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(ModelMetadataResponse {
                    name: name__.unwrap_or_default(),
                    versions: versions__.unwrap_or_default(),
                    platform: platform__.unwrap_or_default(),
                    inputs: inputs__.unwrap_or_default(),
                    outputs: outputs__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelMetadataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for model_metadata_response::TensorMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.datatype.is_empty() {
            len += 1;
        }
        if !self.shape.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelMetadataResponse.TensorMetadata", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.datatype.is_empty() {
            struct_ser.serialize_field("datatype", &self.datatype)?;
        }
        if !self.shape.is_empty() {
            struct_ser.serialize_field("shape", &self.shape.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for model_metadata_response::TensorMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "datatype",
            "shape",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Datatype,
            Shape,
            Parameters,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "datatype" => Ok(GeneratedField::Datatype),
                            "shape" => Ok(GeneratedField::Shape),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = model_metadata_response::TensorMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelMetadataResponse.TensorMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<model_metadata_response::TensorMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut datatype__ = None;
                let mut shape__ = None;
                let mut parameters__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Datatype => {
                            if datatype__.is_some() {
                                return Err(serde::de::Error::duplicate_field("datatype"));
                            }
                            datatype__ = Some(map.next_value()?);
                        }
                        GeneratedField::Shape => {
                            if shape__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            shape__ =
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(model_metadata_response::TensorMetadata {
                    name: name__.unwrap_or_default(),
                    datatype: datatype__.unwrap_or_default(),
                    shape: shape__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelMetadataResponse.TensorMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelReadyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelReadyRequest", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelReadyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "version" => Ok(GeneratedField::Version),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelReadyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelReadyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelReadyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ModelReadyRequest {
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelReadyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelReadyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ready {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ModelReadyResponse", len)?;
        if self.ready {
            struct_ser.serialize_field("ready", &self.ready)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelReadyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ready",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ready,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ready" => Ok(GeneratedField::Ready),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelReadyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ModelReadyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelReadyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ready__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ready => {
                            if ready__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ready"));
                            }
                            ready__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ModelReadyResponse {
                    ready: ready__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ModelReadyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerLiveRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("inference.ServerLiveRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerLiveRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerLiveRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ServerLiveRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerLiveRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ServerLiveRequest {
                })
            }
        }
        deserializer.deserialize_struct("inference.ServerLiveRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerLiveResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.live {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ServerLiveResponse", len)?;
        if self.live {
            struct_ser.serialize_field("live", &self.live)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerLiveResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "live",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Live,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "live" => Ok(GeneratedField::Live),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerLiveResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ServerLiveResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerLiveResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut live__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Live => {
                            if live__.is_some() {
                                return Err(serde::de::Error::duplicate_field("live"));
                            }
                            live__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ServerLiveResponse {
                    live: live__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ServerLiveResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerMetadataRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("inference.ServerMetadataRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerMetadataRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerMetadataRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ServerMetadataRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerMetadataRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ServerMetadataRequest {
                })
            }
        }
        deserializer.deserialize_struct("inference.ServerMetadataRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.version.is_empty() {
            len += 1;
        }
        if !self.extensions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ServerMetadataResponse", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.extensions.is_empty() {
            struct_ser.serialize_field("extensions", &self.extensions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerMetadataResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "extensions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            Extensions,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "version" => Ok(GeneratedField::Version),
                            "extensions" => Ok(GeneratedField::Extensions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ServerMetadataResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerMetadataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut extensions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map.next_value()?);
                        }
                        GeneratedField::Extensions => {
                            if extensions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("extensions"));
                            }
                            extensions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ServerMetadataResponse {
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    extensions: extensions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ServerMetadataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerReadyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("inference.ServerReadyRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerReadyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerReadyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ServerReadyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerReadyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ServerReadyRequest {
                })
            }
        }
        deserializer.deserialize_struct("inference.ServerReadyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerReadyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.ready {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.ServerReadyResponse", len)?;
        if self.ready {
            struct_ser.serialize_field("ready", &self.ready)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerReadyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ready",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Ready,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ready" => Ok(GeneratedField::Ready),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerReadyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.ServerReadyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServerReadyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut ready__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Ready => {
                            if ready__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ready"));
                            }
                            ready__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ServerReadyResponse {
                    ready: ready__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.ServerReadyResponse", FIELDS, GeneratedVisitor)
    }
}
