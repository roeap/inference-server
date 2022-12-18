// @generated
impl serde::Serialize for ApiVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.major.is_some() {
            len += 1;
        }
        if self.minor.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ApiVersion", len)?;
        if let Some(v) = self.major.as_ref() {
            struct_ser.serialize_field("major", v)?;
        }
        if let Some(v) = self.minor.as_ref() {
            struct_ser.serialize_field("minor", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ApiVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "major",
            "minor",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Major,
            Minor,
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
                            "major" => Ok(GeneratedField::Major),
                            "minor" => Ok(GeneratedField::Minor),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ApiVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ApiVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ApiVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut major__ = None;
                let mut minor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Major => {
                            if major__.is_some() {
                                return Err(serde::de::Error::duplicate_field("major"));
                            }
                            major__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Minor => {
                            if minor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minor"));
                            }
                            minor__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ApiVersion {
                    major: major__,
                    minor: minor__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ApiVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArtifactCredentialInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.path.is_some() {
            len += 1;
        }
        if self.signed_uri.is_some() {
            len += 1;
        }
        if !self.headers.is_empty() {
            len += 1;
        }
        if self.r#type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ArtifactCredentialInfo", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        if let Some(v) = self.signed_uri.as_ref() {
            struct_ser.serialize_field("signedUri", v)?;
        }
        if !self.headers.is_empty() {
            struct_ser.serialize_field("headers", &self.headers)?;
        }
        if let Some(v) = self.r#type.as_ref() {
            let v = ArtifactCredentialType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ArtifactCredentialInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "path",
            "signed_uri",
            "signedUri",
            "headers",
            "type",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            Path,
            SignedUri,
            Headers,
            Type,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "path" => Ok(GeneratedField::Path),
                            "signedUri" | "signed_uri" => Ok(GeneratedField::SignedUri),
                            "headers" => Ok(GeneratedField::Headers),
                            "type" => Ok(GeneratedField::Type),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArtifactCredentialInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ArtifactCredentialInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ArtifactCredentialInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut path__ = None;
                let mut signed_uri__ = None;
                let mut headers__ = None;
                let mut r#type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map.next_value()?;
                        }
                        GeneratedField::SignedUri => {
                            if signed_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signedUri"));
                            }
                            signed_uri__ = map.next_value()?;
                        }
                        GeneratedField::Headers => {
                            if headers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("headers"));
                            }
                            headers__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map.next_value::<::std::option::Option<ArtifactCredentialType>>()?.map(|x| x as i32);
                        }
                    }
                }
                Ok(ArtifactCredentialInfo {
                    run_id: run_id__,
                    path: path__,
                    signed_uri: signed_uri__,
                    headers: headers__.unwrap_or_default(),
                    r#type: r#type__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ArtifactCredentialInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for artifact_credential_info::HttpHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ArtifactCredentialInfo.HttpHeader", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for artifact_credential_info::HttpHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
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
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = artifact_credential_info::HttpHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ArtifactCredentialInfo.HttpHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<artifact_credential_info::HttpHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(artifact_credential_info::HttpHeader {
                    name: name__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ArtifactCredentialInfo.HttpHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ArtifactCredentialType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AzureSasUri => "AZURE_SAS_URI",
            Self::AwsPresignedUrl => "AWS_PRESIGNED_URL",
            Self::GcpSignedUrl => "GCP_SIGNED_URL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ArtifactCredentialType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AZURE_SAS_URI",
            "AWS_PRESIGNED_URL",
            "GCP_SIGNED_URL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ArtifactCredentialType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ArtifactCredentialType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ArtifactCredentialType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "AZURE_SAS_URI" => Ok(ArtifactCredentialType::AzureSasUri),
                    "AWS_PRESIGNED_URL" => Ok(ArtifactCredentialType::AwsPresignedUrl),
                    "GCP_SIGNED_URL" => Ok(ArtifactCredentialType::GcpSignedUrl),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreateExperiment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.artifact_location.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.CreateExperiment", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.artifact_location.as_ref() {
            struct_ser.serialize_field("artifactLocation", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateExperiment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "artifact_location",
            "artifactLocation",
            "tags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ArtifactLocation,
            Tags,
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
                            "artifactLocation" | "artifact_location" => Ok(GeneratedField::ArtifactLocation),
                            "tags" => Ok(GeneratedField::Tags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateExperiment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.CreateExperiment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateExperiment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut artifact_location__ = None;
                let mut tags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::ArtifactLocation => {
                            if artifact_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactLocation"));
                            }
                            artifact_location__ = map.next_value()?;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CreateExperiment {
                    name: name__,
                    artifact_location: artifact_location__,
                    tags: tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.CreateExperiment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_experiment::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.CreateExperiment.Response", len)?;
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_experiment::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_id",
            "experimentId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentId,
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
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_experiment::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.CreateExperiment.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<create_experiment::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(create_experiment::Response {
                    experiment_id: experiment_id__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.CreateExperiment.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateModelVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        if self.run_id.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.run_link.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.CreateModelVersion", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.run_link.as_ref() {
            struct_ser.serialize_field("runLink", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateModelVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "source",
            "run_id",
            "runId",
            "tags",
            "run_link",
            "runLink",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Source,
            RunId,
            Tags,
            RunLink,
            Description,
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
                            "source" => Ok(GeneratedField::Source),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "tags" => Ok(GeneratedField::Tags),
                            "runLink" | "run_link" => Ok(GeneratedField::RunLink),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateModelVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.CreateModelVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateModelVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut source__ = None;
                let mut run_id__ = None;
                let mut tags__ = None;
                let mut run_link__ = None;
                let mut description__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map.next_value()?;
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::RunLink => {
                            if run_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runLink"));
                            }
                            run_link__ = map.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateModelVersion {
                    name: name__,
                    source: source__,
                    run_id: run_id__,
                    tags: tags__.unwrap_or_default(),
                    run_link: run_link__,
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.CreateModelVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_model_version::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.model_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.CreateModelVersion.Response", len)?;
        if let Some(v) = self.model_version.as_ref() {
            struct_ser.serialize_field("modelVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_model_version::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_version",
            "modelVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelVersion,
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
                            "modelVersion" | "model_version" => Ok(GeneratedField::ModelVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_model_version::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.CreateModelVersion.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<create_model_version::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModelVersion => {
                            if model_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelVersion"));
                            }
                            model_version__ = map.next_value()?;
                        }
                    }
                }
                Ok(create_model_version::Response {
                    model_version: model_version__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.CreateModelVersion.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRegisteredModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.CreateRegisteredModel", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRegisteredModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "tags",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Tags,
            Description,
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
                            "tags" => Ok(GeneratedField::Tags),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRegisteredModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.CreateRegisteredModel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateRegisteredModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut tags__ = None;
                let mut description__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreateRegisteredModel {
                    name: name__,
                    tags: tags__.unwrap_or_default(),
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.CreateRegisteredModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_registered_model::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.registered_model.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.CreateRegisteredModel.Response", len)?;
        if let Some(v) = self.registered_model.as_ref() {
            struct_ser.serialize_field("registeredModel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_registered_model::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "registered_model",
            "registeredModel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredModel,
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
                            "registeredModel" | "registered_model" => Ok(GeneratedField::RegisteredModel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_registered_model::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.CreateRegisteredModel.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<create_registered_model::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut registered_model__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredModel => {
                            if registered_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredModel"));
                            }
                            registered_model__ = map.next_value()?;
                        }
                    }
                }
                Ok(create_registered_model::Response {
                    registered_model: registered_model__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.CreateRegisteredModel.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateRun {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_id.is_some() {
            len += 1;
        }
        if self.user_id.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.CreateRun", len)?;
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", ToString::to_string(&v).as_str())?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateRun {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_id",
            "experimentId",
            "user_id",
            "userId",
            "start_time",
            "startTime",
            "tags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentId,
            UserId,
            StartTime,
            Tags,
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
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "tags" => Ok(GeneratedField::Tags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateRun;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.CreateRun")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreateRun, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_id__ = None;
                let mut user_id__ = None;
                let mut start_time__ = None;
                let mut tags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map.next_value()?;
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CreateRun {
                    experiment_id: experiment_id__,
                    user_id: user_id__,
                    start_time: start_time__,
                    tags: tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.CreateRun", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for create_run::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.CreateRun.Response", len)?;
        if let Some(v) = self.run.as_ref() {
            struct_ser.serialize_field("run", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for create_run::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Run,
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
                            "run" => Ok(GeneratedField::Run),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = create_run::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.CreateRun.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<create_run::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Run => {
                            if run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run"));
                            }
                            run__ = map.next_value()?;
                        }
                    }
                }
                Ok(create_run::Response {
                    run: run__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.CreateRun.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatabricksRpcOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.endpoints.is_empty() {
            len += 1;
        }
        if self.visibility.is_some() {
            len += 1;
        }
        if !self.error_codes.is_empty() {
            len += 1;
        }
        if self.rate_limit.is_some() {
            len += 1;
        }
        if self.rpc_doc_title.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DatabricksRpcOptions", len)?;
        if !self.endpoints.is_empty() {
            struct_ser.serialize_field("endpoints", &self.endpoints)?;
        }
        if let Some(v) = self.visibility.as_ref() {
            let v = Visibility::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("visibility", &v)?;
        }
        if !self.error_codes.is_empty() {
            let v = self.error_codes.iter().cloned().map(|v| {
                ErrorCode::from_i32(v)
                    .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", v)))
                }).collect::<Result<Vec<_>, _>>()?;
            struct_ser.serialize_field("errorCodes", &v)?;
        }
        if let Some(v) = self.rate_limit.as_ref() {
            struct_ser.serialize_field("rateLimit", v)?;
        }
        if let Some(v) = self.rpc_doc_title.as_ref() {
            struct_ser.serialize_field("rpcDocTitle", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatabricksRpcOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "endpoints",
            "visibility",
            "error_codes",
            "errorCodes",
            "rate_limit",
            "rateLimit",
            "rpc_doc_title",
            "rpcDocTitle",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Endpoints,
            Visibility,
            ErrorCodes,
            RateLimit,
            RpcDocTitle,
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
                            "endpoints" => Ok(GeneratedField::Endpoints),
                            "visibility" => Ok(GeneratedField::Visibility),
                            "errorCodes" | "error_codes" => Ok(GeneratedField::ErrorCodes),
                            "rateLimit" | "rate_limit" => Ok(GeneratedField::RateLimit),
                            "rpcDocTitle" | "rpc_doc_title" => Ok(GeneratedField::RpcDocTitle),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatabricksRpcOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DatabricksRpcOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DatabricksRpcOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut endpoints__ = None;
                let mut visibility__ = None;
                let mut error_codes__ = None;
                let mut rate_limit__ = None;
                let mut rpc_doc_title__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Endpoints => {
                            if endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoints"));
                            }
                            endpoints__ = Some(map.next_value()?);
                        }
                        GeneratedField::Visibility => {
                            if visibility__.is_some() {
                                return Err(serde::de::Error::duplicate_field("visibility"));
                            }
                            visibility__ = map.next_value::<::std::option::Option<Visibility>>()?.map(|x| x as i32);
                        }
                        GeneratedField::ErrorCodes => {
                            if error_codes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorCodes"));
                            }
                            error_codes__ = Some(map.next_value::<Vec<ErrorCode>>()?.into_iter().map(|x| x as i32).collect());
                        }
                        GeneratedField::RateLimit => {
                            if rate_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateLimit"));
                            }
                            rate_limit__ = map.next_value()?;
                        }
                        GeneratedField::RpcDocTitle => {
                            if rpc_doc_title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rpcDocTitle"));
                            }
                            rpc_doc_title__ = map.next_value()?;
                        }
                    }
                }
                Ok(DatabricksRpcOptions {
                    endpoints: endpoints__.unwrap_or_default(),
                    visibility: visibility__,
                    error_codes: error_codes__.unwrap_or_default(),
                    rate_limit: rate_limit__,
                    rpc_doc_title: rpc_doc_title__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DatabricksRpcOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatabricksServiceExceptionProto {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.error_code.is_some() {
            len += 1;
        }
        if self.message.is_some() {
            len += 1;
        }
        if self.stack_trace.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DatabricksServiceExceptionProto", len)?;
        if let Some(v) = self.error_code.as_ref() {
            let v = ErrorCode::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("errorCode", &v)?;
        }
        if let Some(v) = self.message.as_ref() {
            struct_ser.serialize_field("message", v)?;
        }
        if let Some(v) = self.stack_trace.as_ref() {
            struct_ser.serialize_field("stackTrace", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatabricksServiceExceptionProto {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "error_code",
            "errorCode",
            "message",
            "stack_trace",
            "stackTrace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ErrorCode,
            Message,
            StackTrace,
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
                            "errorCode" | "error_code" => Ok(GeneratedField::ErrorCode),
                            "message" => Ok(GeneratedField::Message),
                            "stackTrace" | "stack_trace" => Ok(GeneratedField::StackTrace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatabricksServiceExceptionProto;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DatabricksServiceExceptionProto")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DatabricksServiceExceptionProto, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut error_code__ = None;
                let mut message__ = None;
                let mut stack_trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ErrorCode => {
                            if error_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorCode"));
                            }
                            error_code__ = map.next_value::<::std::option::Option<ErrorCode>>()?.map(|x| x as i32);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = map.next_value()?;
                        }
                        GeneratedField::StackTrace => {
                            if stack_trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stackTrace"));
                            }
                            stack_trace__ = map.next_value()?;
                        }
                    }
                }
                Ok(DatabricksServiceExceptionProto {
                    error_code: error_code__,
                    message: message__,
                    stack_trace: stack_trace__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DatabricksServiceExceptionProto", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteExperiment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DeleteExperiment", len)?;
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteExperiment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_id",
            "experimentId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentId,
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
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteExperiment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteExperiment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteExperiment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteExperiment {
                    experiment_id: experiment_id__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteExperiment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for delete_experiment::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.DeleteExperiment.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for delete_experiment::Response {
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
            type Value = delete_experiment::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteExperiment.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<delete_experiment::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(delete_experiment::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteExperiment.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteModelVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DeleteModelVersion", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteModelVersion {
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
            type Value = DeleteModelVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteModelVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteModelVersion, V::Error>
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
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteModelVersion {
                    name: name__,
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteModelVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for delete_model_version::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.DeleteModelVersion.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for delete_model_version::Response {
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
            type Value = delete_model_version::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteModelVersion.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<delete_model_version::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(delete_model_version::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteModelVersion.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteModelVersionTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DeleteModelVersionTag", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteModelVersionTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteModelVersionTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteModelVersionTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteModelVersionTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteModelVersionTag {
                    name: name__,
                    version: version__,
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteModelVersionTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for delete_model_version_tag::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.DeleteModelVersionTag.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for delete_model_version_tag::Response {
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
            type Value = delete_model_version_tag::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteModelVersionTag.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<delete_model_version_tag::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(delete_model_version_tag::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteModelVersionTag.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRegisteredModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DeleteRegisteredModel", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRegisteredModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRegisteredModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteRegisteredModel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRegisteredModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteRegisteredModel {
                    name: name__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteRegisteredModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for delete_registered_model::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.DeleteRegisteredModel.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for delete_registered_model::Response {
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
            type Value = delete_registered_model::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteRegisteredModel.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<delete_registered_model::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(delete_registered_model::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteRegisteredModel.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRegisteredModelTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DeleteRegisteredModelTag", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRegisteredModelTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Key,
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
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRegisteredModelTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteRegisteredModelTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRegisteredModelTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteRegisteredModelTag {
                    name: name__,
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteRegisteredModelTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for delete_registered_model_tag::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.DeleteRegisteredModelTag.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for delete_registered_model_tag::Response {
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
            type Value = delete_registered_model_tag::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteRegisteredModelTag.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<delete_registered_model_tag::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(delete_registered_model_tag::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteRegisteredModelTag.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRun {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DeleteRun", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRun {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRun;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteRun")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRun, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteRun {
                    run_id: run_id__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteRun", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for delete_run::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.DeleteRun.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for delete_run::Response {
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
            type Value = delete_run::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteRun.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<delete_run::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(delete_run::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteRun.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DeleteTag", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            Key,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteTag {
                    run_id: run_id__,
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for delete_tag::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.DeleteTag.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for delete_tag::Response {
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
            type Value = delete_tag::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DeleteTag.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<delete_tag::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(delete_tag::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DeleteTag.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DocumentationMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.docstring.is_some() {
            len += 1;
        }
        if self.lead_doc.is_some() {
            len += 1;
        }
        if self.visibility.is_some() {
            len += 1;
        }
        if !self.original_proto_path.is_empty() {
            len += 1;
        }
        if self.position.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.DocumentationMetadata", len)?;
        if let Some(v) = self.docstring.as_ref() {
            struct_ser.serialize_field("docstring", v)?;
        }
        if let Some(v) = self.lead_doc.as_ref() {
            struct_ser.serialize_field("leadDoc", v)?;
        }
        if let Some(v) = self.visibility.as_ref() {
            let v = Visibility::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("visibility", &v)?;
        }
        if !self.original_proto_path.is_empty() {
            struct_ser.serialize_field("originalProtoPath", &self.original_proto_path)?;
        }
        if let Some(v) = self.position.as_ref() {
            struct_ser.serialize_field("position", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DocumentationMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "docstring",
            "lead_doc",
            "leadDoc",
            "visibility",
            "original_proto_path",
            "originalProtoPath",
            "position",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Docstring,
            LeadDoc,
            Visibility,
            OriginalProtoPath,
            Position,
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
                            "docstring" => Ok(GeneratedField::Docstring),
                            "leadDoc" | "lead_doc" => Ok(GeneratedField::LeadDoc),
                            "visibility" => Ok(GeneratedField::Visibility),
                            "originalProtoPath" | "original_proto_path" => Ok(GeneratedField::OriginalProtoPath),
                            "position" => Ok(GeneratedField::Position),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DocumentationMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.DocumentationMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DocumentationMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut docstring__ = None;
                let mut lead_doc__ = None;
                let mut visibility__ = None;
                let mut original_proto_path__ = None;
                let mut position__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Docstring => {
                            if docstring__.is_some() {
                                return Err(serde::de::Error::duplicate_field("docstring"));
                            }
                            docstring__ = map.next_value()?;
                        }
                        GeneratedField::LeadDoc => {
                            if lead_doc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("leadDoc"));
                            }
                            lead_doc__ = map.next_value()?;
                        }
                        GeneratedField::Visibility => {
                            if visibility__.is_some() {
                                return Err(serde::de::Error::duplicate_field("visibility"));
                            }
                            visibility__ = map.next_value::<::std::option::Option<Visibility>>()?.map(|x| x as i32);
                        }
                        GeneratedField::OriginalProtoPath => {
                            if original_proto_path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originalProtoPath"));
                            }
                            original_proto_path__ = Some(map.next_value()?);
                        }
                        GeneratedField::Position => {
                            if position__.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            position__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(DocumentationMetadata {
                    docstring: docstring__,
                    lead_doc: lead_doc__,
                    visibility: visibility__,
                    original_proto_path: original_proto_path__.unwrap_or_default(),
                    position: position__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.DocumentationMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ErrorCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::InternalError => "INTERNAL_ERROR",
            Self::TemporarilyUnavailable => "TEMPORARILY_UNAVAILABLE",
            Self::IoError => "IO_ERROR",
            Self::BadRequest => "BAD_REQUEST",
            Self::InvalidParameterValue => "INVALID_PARAMETER_VALUE",
            Self::EndpointNotFound => "ENDPOINT_NOT_FOUND",
            Self::MalformedRequest => "MALFORMED_REQUEST",
            Self::InvalidState => "INVALID_STATE",
            Self::PermissionDenied => "PERMISSION_DENIED",
            Self::FeatureDisabled => "FEATURE_DISABLED",
            Self::CustomerUnauthorized => "CUSTOMER_UNAUTHORIZED",
            Self::RequestLimitExceeded => "REQUEST_LIMIT_EXCEEDED",
            Self::InvalidStateTransition => "INVALID_STATE_TRANSITION",
            Self::CouldNotAcquireLock => "COULD_NOT_ACQUIRE_LOCK",
            Self::ResourceAlreadyExists => "RESOURCE_ALREADY_EXISTS",
            Self::ResourceDoesNotExist => "RESOURCE_DOES_NOT_EXIST",
            Self::QuotaExceeded => "QUOTA_EXCEEDED",
            Self::MaxBlockSizeExceeded => "MAX_BLOCK_SIZE_EXCEEDED",
            Self::MaxReadSizeExceeded => "MAX_READ_SIZE_EXCEEDED",
            Self::DryRunFailed => "DRY_RUN_FAILED",
            Self::ResourceLimitExceeded => "RESOURCE_LIMIT_EXCEEDED",
            Self::DirectoryNotEmpty => "DIRECTORY_NOT_EMPTY",
            Self::DirectoryProtected => "DIRECTORY_PROTECTED",
            Self::MaxNotebookSizeExceeded => "MAX_NOTEBOOK_SIZE_EXCEEDED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ErrorCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INTERNAL_ERROR",
            "TEMPORARILY_UNAVAILABLE",
            "IO_ERROR",
            "BAD_REQUEST",
            "INVALID_PARAMETER_VALUE",
            "ENDPOINT_NOT_FOUND",
            "MALFORMED_REQUEST",
            "INVALID_STATE",
            "PERMISSION_DENIED",
            "FEATURE_DISABLED",
            "CUSTOMER_UNAUTHORIZED",
            "REQUEST_LIMIT_EXCEEDED",
            "INVALID_STATE_TRANSITION",
            "COULD_NOT_ACQUIRE_LOCK",
            "RESOURCE_ALREADY_EXISTS",
            "RESOURCE_DOES_NOT_EXIST",
            "QUOTA_EXCEEDED",
            "MAX_BLOCK_SIZE_EXCEEDED",
            "MAX_READ_SIZE_EXCEEDED",
            "DRY_RUN_FAILED",
            "RESOURCE_LIMIT_EXCEEDED",
            "DIRECTORY_NOT_EMPTY",
            "DIRECTORY_PROTECTED",
            "MAX_NOTEBOOK_SIZE_EXCEEDED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ErrorCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ErrorCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ErrorCode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "INTERNAL_ERROR" => Ok(ErrorCode::InternalError),
                    "TEMPORARILY_UNAVAILABLE" => Ok(ErrorCode::TemporarilyUnavailable),
                    "IO_ERROR" => Ok(ErrorCode::IoError),
                    "BAD_REQUEST" => Ok(ErrorCode::BadRequest),
                    "INVALID_PARAMETER_VALUE" => Ok(ErrorCode::InvalidParameterValue),
                    "ENDPOINT_NOT_FOUND" => Ok(ErrorCode::EndpointNotFound),
                    "MALFORMED_REQUEST" => Ok(ErrorCode::MalformedRequest),
                    "INVALID_STATE" => Ok(ErrorCode::InvalidState),
                    "PERMISSION_DENIED" => Ok(ErrorCode::PermissionDenied),
                    "FEATURE_DISABLED" => Ok(ErrorCode::FeatureDisabled),
                    "CUSTOMER_UNAUTHORIZED" => Ok(ErrorCode::CustomerUnauthorized),
                    "REQUEST_LIMIT_EXCEEDED" => Ok(ErrorCode::RequestLimitExceeded),
                    "INVALID_STATE_TRANSITION" => Ok(ErrorCode::InvalidStateTransition),
                    "COULD_NOT_ACQUIRE_LOCK" => Ok(ErrorCode::CouldNotAcquireLock),
                    "RESOURCE_ALREADY_EXISTS" => Ok(ErrorCode::ResourceAlreadyExists),
                    "RESOURCE_DOES_NOT_EXIST" => Ok(ErrorCode::ResourceDoesNotExist),
                    "QUOTA_EXCEEDED" => Ok(ErrorCode::QuotaExceeded),
                    "MAX_BLOCK_SIZE_EXCEEDED" => Ok(ErrorCode::MaxBlockSizeExceeded),
                    "MAX_READ_SIZE_EXCEEDED" => Ok(ErrorCode::MaxReadSizeExceeded),
                    "DRY_RUN_FAILED" => Ok(ErrorCode::DryRunFailed),
                    "RESOURCE_LIMIT_EXCEEDED" => Ok(ErrorCode::ResourceLimitExceeded),
                    "DIRECTORY_NOT_EMPTY" => Ok(ErrorCode::DirectoryNotEmpty),
                    "DIRECTORY_PROTECTED" => Ok(ErrorCode::DirectoryProtected),
                    "MAX_NOTEBOOK_SIZE_EXCEEDED" => Ok(ErrorCode::MaxNotebookSizeExceeded),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Experiment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_id.is_some() {
            len += 1;
        }
        if self.name.is_some() {
            len += 1;
        }
        if self.artifact_location.is_some() {
            len += 1;
        }
        if self.lifecycle_stage.is_some() {
            len += 1;
        }
        if self.last_update_time.is_some() {
            len += 1;
        }
        if self.creation_time.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.Experiment", len)?;
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.artifact_location.as_ref() {
            struct_ser.serialize_field("artifactLocation", v)?;
        }
        if let Some(v) = self.lifecycle_stage.as_ref() {
            struct_ser.serialize_field("lifecycleStage", v)?;
        }
        if let Some(v) = self.last_update_time.as_ref() {
            struct_ser.serialize_field("lastUpdateTime", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.creation_time.as_ref() {
            struct_ser.serialize_field("creationTime", ToString::to_string(&v).as_str())?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Experiment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_id",
            "experimentId",
            "name",
            "artifact_location",
            "artifactLocation",
            "lifecycle_stage",
            "lifecycleStage",
            "last_update_time",
            "lastUpdateTime",
            "creation_time",
            "creationTime",
            "tags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentId,
            Name,
            ArtifactLocation,
            LifecycleStage,
            LastUpdateTime,
            CreationTime,
            Tags,
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
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            "name" => Ok(GeneratedField::Name),
                            "artifactLocation" | "artifact_location" => Ok(GeneratedField::ArtifactLocation),
                            "lifecycleStage" | "lifecycle_stage" => Ok(GeneratedField::LifecycleStage),
                            "lastUpdateTime" | "last_update_time" => Ok(GeneratedField::LastUpdateTime),
                            "creationTime" | "creation_time" => Ok(GeneratedField::CreationTime),
                            "tags" => Ok(GeneratedField::Tags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Experiment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.Experiment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Experiment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_id__ = None;
                let mut name__ = None;
                let mut artifact_location__ = None;
                let mut lifecycle_stage__ = None;
                let mut last_update_time__ = None;
                let mut creation_time__ = None;
                let mut tags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::ArtifactLocation => {
                            if artifact_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactLocation"));
                            }
                            artifact_location__ = map.next_value()?;
                        }
                        GeneratedField::LifecycleStage => {
                            if lifecycle_stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycleStage"));
                            }
                            lifecycle_stage__ = map.next_value()?;
                        }
                        GeneratedField::LastUpdateTime => {
                            if last_update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdateTime"));
                            }
                            last_update_time__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::CreationTime => {
                            if creation_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTime"));
                            }
                            creation_time__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Experiment {
                    experiment_id: experiment_id__,
                    name: name__,
                    artifact_location: artifact_location__,
                    lifecycle_stage: lifecycle_stage__,
                    last_update_time: last_update_time__,
                    creation_time: creation_time__,
                    tags: tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.Experiment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExperimentTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ExperimentTag", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExperimentTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ExperimentTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExperimentTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExperimentTag {
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ExperimentTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FileInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.path.is_some() {
            len += 1;
        }
        if self.is_dir.is_some() {
            len += 1;
        }
        if self.file_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.FileInfo", len)?;
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        if let Some(v) = self.is_dir.as_ref() {
            struct_ser.serialize_field("isDir", v)?;
        }
        if let Some(v) = self.file_size.as_ref() {
            struct_ser.serialize_field("fileSize", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FileInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "path",
            "is_dir",
            "isDir",
            "file_size",
            "fileSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Path,
            IsDir,
            FileSize,
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
                            "path" => Ok(GeneratedField::Path),
                            "isDir" | "is_dir" => Ok(GeneratedField::IsDir),
                            "fileSize" | "file_size" => Ok(GeneratedField::FileSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FileInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.FileInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<FileInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut path__ = None;
                let mut is_dir__ = None;
                let mut file_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map.next_value()?;
                        }
                        GeneratedField::IsDir => {
                            if is_dir__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isDir"));
                            }
                            is_dir__ = map.next_value()?;
                        }
                        GeneratedField::FileSize => {
                            if file_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileSize"));
                            }
                            file_size__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(FileInfo {
                    path: path__,
                    is_dir: is_dir__,
                    file_size: file_size__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.FileInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCredentialsForRead {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetCredentialsForRead", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCredentialsForRead {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "path",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            Path,
            PageToken,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "path" => Ok(GeneratedField::Path),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCredentialsForRead;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetCredentialsForRead")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetCredentialsForRead, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut path__ = None;
                let mut page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetCredentialsForRead {
                    run_id: run_id__,
                    path: path__.unwrap_or_default(),
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetCredentialsForRead", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_credentials_for_read::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.credential_infos.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetCredentialsForRead.Response", len)?;
        if !self.credential_infos.is_empty() {
            struct_ser.serialize_field("credentialInfos", &self.credential_infos)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_credentials_for_read::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credential_infos",
            "credentialInfos",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CredentialInfos,
            NextPageToken,
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
                            "credentialInfos" | "credential_infos" => Ok(GeneratedField::CredentialInfos),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_credentials_for_read::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetCredentialsForRead.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_credentials_for_read::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credential_infos__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CredentialInfos => {
                            if credential_infos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentialInfos"));
                            }
                            credential_infos__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(get_credentials_for_read::Response {
                    credential_infos: credential_infos__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetCredentialsForRead.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetCredentialsForWrite {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetCredentialsForWrite", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetCredentialsForWrite {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "path",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            Path,
            PageToken,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "path" => Ok(GeneratedField::Path),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetCredentialsForWrite;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetCredentialsForWrite")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetCredentialsForWrite, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut path__ = None;
                let mut page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetCredentialsForWrite {
                    run_id: run_id__,
                    path: path__.unwrap_or_default(),
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetCredentialsForWrite", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_credentials_for_write::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.credential_infos.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetCredentialsForWrite.Response", len)?;
        if !self.credential_infos.is_empty() {
            struct_ser.serialize_field("credentialInfos", &self.credential_infos)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_credentials_for_write::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credential_infos",
            "credentialInfos",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CredentialInfos,
            NextPageToken,
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
                            "credentialInfos" | "credential_infos" => Ok(GeneratedField::CredentialInfos),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_credentials_for_write::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetCredentialsForWrite.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_credentials_for_write::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credential_infos__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CredentialInfos => {
                            if credential_infos__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credentialInfos"));
                            }
                            credential_infos__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(get_credentials_for_write::Response {
                    credential_infos: credential_infos__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetCredentialsForWrite.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExperiment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetExperiment", len)?;
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExperiment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_id",
            "experimentId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentId,
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
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetExperiment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetExperiment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetExperiment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetExperiment {
                    experiment_id: experiment_id__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetExperiment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_experiment::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment.is_some() {
            len += 1;
        }
        if !self.runs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetExperiment.Response", len)?;
        if let Some(v) = self.experiment.as_ref() {
            struct_ser.serialize_field("experiment", v)?;
        }
        if !self.runs.is_empty() {
            struct_ser.serialize_field("runs", &self.runs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_experiment::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment",
            "runs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Experiment,
            Runs,
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
                            "experiment" => Ok(GeneratedField::Experiment),
                            "runs" => Ok(GeneratedField::Runs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_experiment::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetExperiment.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_experiment::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment__ = None;
                let mut runs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Experiment => {
                            if experiment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experiment"));
                            }
                            experiment__ = map.next_value()?;
                        }
                        GeneratedField::Runs => {
                            if runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runs"));
                            }
                            runs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(get_experiment::Response {
                    experiment: experiment__,
                    runs: runs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetExperiment.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetExperimentByName {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetExperimentByName", len)?;
        if let Some(v) = self.experiment_name.as_ref() {
            struct_ser.serialize_field("experimentName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetExperimentByName {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_name",
            "experimentName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentName,
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
                            "experimentName" | "experiment_name" => Ok(GeneratedField::ExperimentName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetExperimentByName;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetExperimentByName")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetExperimentByName, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentName => {
                            if experiment_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentName"));
                            }
                            experiment_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetExperimentByName {
                    experiment_name: experiment_name__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetExperimentByName", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_experiment_by_name::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetExperimentByName.Response", len)?;
        if let Some(v) = self.experiment.as_ref() {
            struct_ser.serialize_field("experiment", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_experiment_by_name::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Experiment,
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
                            "experiment" => Ok(GeneratedField::Experiment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_experiment_by_name::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetExperimentByName.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_experiment_by_name::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Experiment => {
                            if experiment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experiment"));
                            }
                            experiment__ = map.next_value()?;
                        }
                    }
                }
                Ok(get_experiment_by_name::Response {
                    experiment: experiment__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetExperimentByName.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLatestVersions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if !self.stages.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetLatestVersions", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if !self.stages.is_empty() {
            struct_ser.serialize_field("stages", &self.stages)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLatestVersions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "stages",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Stages,
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
                            "stages" => Ok(GeneratedField::Stages),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLatestVersions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetLatestVersions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetLatestVersions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut stages__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Stages => {
                            if stages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stages"));
                            }
                            stages__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetLatestVersions {
                    name: name__,
                    stages: stages__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetLatestVersions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_latest_versions::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.model_versions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetLatestVersions.Response", len)?;
        if !self.model_versions.is_empty() {
            struct_ser.serialize_field("modelVersions", &self.model_versions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_latest_versions::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_versions",
            "modelVersions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelVersions,
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
                            "modelVersions" | "model_versions" => Ok(GeneratedField::ModelVersions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_latest_versions::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetLatestVersions.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_latest_versions::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_versions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModelVersions => {
                            if model_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelVersions"));
                            }
                            model_versions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(get_latest_versions::Response {
                    model_versions: model_versions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetLatestVersions.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetMetricHistory {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.run_uuid.is_some() {
            len += 1;
        }
        if self.metric_key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetMetricHistory", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.run_uuid.as_ref() {
            struct_ser.serialize_field("runUuid", v)?;
        }
        if let Some(v) = self.metric_key.as_ref() {
            struct_ser.serialize_field("metricKey", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetMetricHistory {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "run_uuid",
            "runUuid",
            "metric_key",
            "metricKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            RunUuid,
            MetricKey,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "runUuid" | "run_uuid" => Ok(GeneratedField::RunUuid),
                            "metricKey" | "metric_key" => Ok(GeneratedField::MetricKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetMetricHistory;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetMetricHistory")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetMetricHistory, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut run_uuid__ = None;
                let mut metric_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::RunUuid => {
                            if run_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runUuid"));
                            }
                            run_uuid__ = map.next_value()?;
                        }
                        GeneratedField::MetricKey => {
                            if metric_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metricKey"));
                            }
                            metric_key__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetMetricHistory {
                    run_id: run_id__,
                    run_uuid: run_uuid__,
                    metric_key: metric_key__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetMetricHistory", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_metric_history::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metrics.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetMetricHistory.Response", len)?;
        if !self.metrics.is_empty() {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_metric_history::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metrics",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metrics,
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
                            "metrics" => Ok(GeneratedField::Metrics),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_metric_history::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetMetricHistory.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_metric_history::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metrics__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(get_metric_history::Response {
                    metrics: metrics__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetMetricHistory.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetModelVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetModelVersion", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetModelVersion {
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
            type Value = GetModelVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetModelVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetModelVersion, V::Error>
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
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetModelVersion {
                    name: name__,
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetModelVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_model_version::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.model_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetModelVersion.Response", len)?;
        if let Some(v) = self.model_version.as_ref() {
            struct_ser.serialize_field("modelVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_model_version::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_version",
            "modelVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelVersion,
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
                            "modelVersion" | "model_version" => Ok(GeneratedField::ModelVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_model_version::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetModelVersion.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_model_version::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModelVersion => {
                            if model_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelVersion"));
                            }
                            model_version__ = map.next_value()?;
                        }
                    }
                }
                Ok(get_model_version::Response {
                    model_version: model_version__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetModelVersion.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetModelVersionDownloadUri {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetModelVersionDownloadUri", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetModelVersionDownloadUri {
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
            type Value = GetModelVersionDownloadUri;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetModelVersionDownloadUri")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetModelVersionDownloadUri, V::Error>
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
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetModelVersionDownloadUri {
                    name: name__,
                    version: version__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetModelVersionDownloadUri", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_model_version_download_uri::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.artifact_uri.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetModelVersionDownloadUri.Response", len)?;
        if let Some(v) = self.artifact_uri.as_ref() {
            struct_ser.serialize_field("artifactUri", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_model_version_download_uri::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "artifact_uri",
            "artifactUri",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ArtifactUri,
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
                            "artifactUri" | "artifact_uri" => Ok(GeneratedField::ArtifactUri),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_model_version_download_uri::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetModelVersionDownloadUri.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_model_version_download_uri::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut artifact_uri__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ArtifactUri => {
                            if artifact_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactUri"));
                            }
                            artifact_uri__ = map.next_value()?;
                        }
                    }
                }
                Ok(get_model_version_download_uri::Response {
                    artifact_uri: artifact_uri__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetModelVersionDownloadUri.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisteredModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetRegisteredModel", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisteredModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisteredModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetRegisteredModel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRegisteredModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetRegisteredModel {
                    name: name__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetRegisteredModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_registered_model::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.registered_model.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetRegisteredModel.Response", len)?;
        if let Some(v) = self.registered_model.as_ref() {
            struct_ser.serialize_field("registeredModel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_registered_model::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "registered_model",
            "registeredModel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredModel,
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
                            "registeredModel" | "registered_model" => Ok(GeneratedField::RegisteredModel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_registered_model::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetRegisteredModel.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_registered_model::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut registered_model__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredModel => {
                            if registered_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredModel"));
                            }
                            registered_model__ = map.next_value()?;
                        }
                    }
                }
                Ok(get_registered_model::Response {
                    registered_model: registered_model__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetRegisteredModel.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRun {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.run_uuid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetRun", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.run_uuid.as_ref() {
            struct_ser.serialize_field("runUuid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRun {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "run_uuid",
            "runUuid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            RunUuid,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "runUuid" | "run_uuid" => Ok(GeneratedField::RunUuid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRun;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetRun")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRun, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut run_uuid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::RunUuid => {
                            if run_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runUuid"));
                            }
                            run_uuid__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetRun {
                    run_id: run_id__,
                    run_uuid: run_uuid__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetRun", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_run::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.GetRun.Response", len)?;
        if let Some(v) = self.run.as_ref() {
            struct_ser.serialize_field("run", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_run::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Run,
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
                            "run" => Ok(GeneratedField::Run),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_run::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.GetRun.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_run::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Run => {
                            if run__.is_some() {
                                return Err(serde::de::Error::duplicate_field("run"));
                            }
                            run__ = map.next_value()?;
                        }
                    }
                }
                Ok(get_run::Response {
                    run: run__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.GetRun.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for HttpEndpoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.method.is_some() {
            len += 1;
        }
        if self.path.is_some() {
            len += 1;
        }
        if self.since.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.HttpEndpoint", len)?;
        if let Some(v) = self.method.as_ref() {
            struct_ser.serialize_field("method", v)?;
        }
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        if let Some(v) = self.since.as_ref() {
            struct_ser.serialize_field("since", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HttpEndpoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "method",
            "path",
            "since",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Method,
            Path,
            Since,
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
                            "method" => Ok(GeneratedField::Method),
                            "path" => Ok(GeneratedField::Path),
                            "since" => Ok(GeneratedField::Since),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HttpEndpoint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.HttpEndpoint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<HttpEndpoint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut method__ = None;
                let mut path__ = None;
                let mut since__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Method => {
                            if method__.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method__ = map.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map.next_value()?;
                        }
                        GeneratedField::Since => {
                            if since__.is_some() {
                                return Err(serde::de::Error::duplicate_field("since"));
                            }
                            since__ = map.next_value()?;
                        }
                    }
                }
                Ok(HttpEndpoint {
                    method: method__,
                    path: path__,
                    since: since__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.HttpEndpoint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListArtifacts {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.run_uuid.is_some() {
            len += 1;
        }
        if self.path.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ListArtifacts", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.run_uuid.as_ref() {
            struct_ser.serialize_field("runUuid", v)?;
        }
        if let Some(v) = self.path.as_ref() {
            struct_ser.serialize_field("path", v)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListArtifacts {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "run_uuid",
            "runUuid",
            "path",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            RunUuid,
            Path,
            PageToken,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "runUuid" | "run_uuid" => Ok(GeneratedField::RunUuid),
                            "path" => Ok(GeneratedField::Path),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListArtifacts;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ListArtifacts")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListArtifacts, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut run_uuid__ = None;
                let mut path__ = None;
                let mut page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::RunUuid => {
                            if run_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runUuid"));
                            }
                            run_uuid__ = map.next_value()?;
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = map.next_value()?;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(ListArtifacts {
                    run_id: run_id__,
                    run_uuid: run_uuid__,
                    path: path__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ListArtifacts", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for list_artifacts::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.root_uri.is_some() {
            len += 1;
        }
        if !self.files.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ListArtifacts.Response", len)?;
        if let Some(v) = self.root_uri.as_ref() {
            struct_ser.serialize_field("rootUri", v)?;
        }
        if !self.files.is_empty() {
            struct_ser.serialize_field("files", &self.files)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for list_artifacts::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root_uri",
            "rootUri",
            "files",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RootUri,
            Files,
            NextPageToken,
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
                            "rootUri" | "root_uri" => Ok(GeneratedField::RootUri),
                            "files" => Ok(GeneratedField::Files),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = list_artifacts::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ListArtifacts.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<list_artifacts::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root_uri__ = None;
                let mut files__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RootUri => {
                            if root_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootUri"));
                            }
                            root_uri__ = map.next_value()?;
                        }
                        GeneratedField::Files => {
                            if files__.is_some() {
                                return Err(serde::de::Error::duplicate_field("files"));
                            }
                            files__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(list_artifacts::Response {
                    root_uri: root_uri__,
                    files: files__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ListArtifacts.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListExperiments {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.view_type.is_some() {
            len += 1;
        }
        if self.max_results.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ListExperiments", len)?;
        if let Some(v) = self.view_type.as_ref() {
            let v = ViewType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("viewType", &v)?;
        }
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListExperiments {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view_type",
            "viewType",
            "max_results",
            "maxResults",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ViewType,
            MaxResults,
            PageToken,
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
                            "viewType" | "view_type" => Ok(GeneratedField::ViewType),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListExperiments;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ListExperiments")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListExperiments, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view_type__ = None;
                let mut max_results__ = None;
                let mut page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ViewType => {
                            if view_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("viewType"));
                            }
                            view_type__ = map.next_value::<::std::option::Option<ViewType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(ListExperiments {
                    view_type: view_type__,
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ListExperiments", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for list_experiments::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.experiments.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ListExperiments.Response", len)?;
        if !self.experiments.is_empty() {
            struct_ser.serialize_field("experiments", &self.experiments)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for list_experiments::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiments",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Experiments,
            NextPageToken,
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
                            "experiments" => Ok(GeneratedField::Experiments),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = list_experiments::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ListExperiments.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<list_experiments::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiments__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Experiments => {
                            if experiments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experiments"));
                            }
                            experiments__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(list_experiments::Response {
                    experiments: experiments__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ListExperiments.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListRegisteredModels {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_results.is_some() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ListRegisteredModels", len)?;
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListRegisteredModels {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_results",
            "maxResults",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxResults,
            PageToken,
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
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListRegisteredModels;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ListRegisteredModels")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ListRegisteredModels, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_results__ = None;
                let mut page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(ListRegisteredModels {
                    max_results: max_results__,
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ListRegisteredModels", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for list_registered_models::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.registered_models.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ListRegisteredModels.Response", len)?;
        if !self.registered_models.is_empty() {
            struct_ser.serialize_field("registeredModels", &self.registered_models)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for list_registered_models::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "registered_models",
            "registeredModels",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredModels,
            NextPageToken,
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
                            "registeredModels" | "registered_models" => Ok(GeneratedField::RegisteredModels),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = list_registered_models::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ListRegisteredModels.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<list_registered_models::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut registered_models__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredModels => {
                            if registered_models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredModels"));
                            }
                            registered_models__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(list_registered_models::Response {
                    registered_models: registered_models__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ListRegisteredModels.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if !self.metrics.is_empty() {
            len += 1;
        }
        if !self.params.is_empty() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.LogBatch", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if !self.metrics.is_empty() {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        if !self.params.is_empty() {
            struct_ser.serialize_field("params", &self.params)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "metrics",
            "params",
            "tags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            Metrics,
            Params,
            Tags,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "metrics" => Ok(GeneratedField::Metrics),
                            "params" => Ok(GeneratedField::Params),
                            "tags" => Ok(GeneratedField::Tags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.LogBatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LogBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut metrics__ = None;
                let mut params__ = None;
                let mut tags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(map.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LogBatch {
                    run_id: run_id__,
                    metrics: metrics__.unwrap_or_default(),
                    params: params__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.LogBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for log_batch::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.LogBatch.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for log_batch::Response {
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
            type Value = log_batch::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.LogBatch.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<log_batch::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(log_batch::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.LogBatch.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogMetric {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.run_uuid.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.step.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.LogMetric", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.run_uuid.as_ref() {
            struct_ser.serialize_field("runUuid", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.step.as_ref() {
            struct_ser.serialize_field("step", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogMetric {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "run_uuid",
            "runUuid",
            "key",
            "value",
            "timestamp",
            "step",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            RunUuid,
            Key,
            Value,
            Timestamp,
            Step,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "runUuid" | "run_uuid" => Ok(GeneratedField::RunUuid),
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "step" => Ok(GeneratedField::Step),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogMetric;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.LogMetric")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LogMetric, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut run_uuid__ = None;
                let mut key__ = None;
                let mut value__ = None;
                let mut timestamp__ = None;
                let mut step__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::RunUuid => {
                            if run_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runUuid"));
                            }
                            run_uuid__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Step => {
                            if step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("step"));
                            }
                            step__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(LogMetric {
                    run_id: run_id__,
                    run_uuid: run_uuid__,
                    key: key__,
                    value: value__,
                    timestamp: timestamp__,
                    step: step__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.LogMetric", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for log_metric::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.LogMetric.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for log_metric::Response {
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
            type Value = log_metric::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.LogMetric.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<log_metric::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(log_metric::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.LogMetric.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.model_json.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.LogModel", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.model_json.as_ref() {
            struct_ser.serialize_field("modelJson", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "model_json",
            "modelJson",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            ModelJson,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "modelJson" | "model_json" => Ok(GeneratedField::ModelJson),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.LogModel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LogModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut model_json__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::ModelJson => {
                            if model_json__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelJson"));
                            }
                            model_json__ = map.next_value()?;
                        }
                    }
                }
                Ok(LogModel {
                    run_id: run_id__,
                    model_json: model_json__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.LogModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for log_model::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.LogModel.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for log_model::Response {
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
            type Value = log_model::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.LogModel.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<log_model::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(log_model::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.LogModel.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LogParam {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.run_uuid.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.LogParam", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.run_uuid.as_ref() {
            struct_ser.serialize_field("runUuid", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LogParam {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "run_uuid",
            "runUuid",
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            RunUuid,
            Key,
            Value,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "runUuid" | "run_uuid" => Ok(GeneratedField::RunUuid),
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LogParam;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.LogParam")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<LogParam, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut run_uuid__ = None;
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::RunUuid => {
                            if run_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runUuid"));
                            }
                            run_uuid__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(LogParam {
                    run_id: run_id__,
                    run_uuid: run_uuid__,
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.LogParam", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for log_param::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.LogParam.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for log_param::Response {
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
            type Value = log_param::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.LogParam.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<log_param::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(log_param::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.LogParam.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Metric {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if self.step.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.Metric", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.step.as_ref() {
            struct_ser.serialize_field("step", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Metric {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
            "timestamp",
            "step",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
            Timestamp,
            Step,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "step" => Ok(GeneratedField::Step),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Metric;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.Metric")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Metric, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                let mut timestamp__ = None;
                let mut step__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Step => {
                            if step__.is_some() {
                                return Err(serde::de::Error::duplicate_field("step"));
                            }
                            step__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(Metric {
                    key: key__,
                    value: value__,
                    timestamp: timestamp__,
                    step: step__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.Metric", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.creation_timestamp.is_some() {
            len += 1;
        }
        if self.last_updated_timestamp.is_some() {
            len += 1;
        }
        if self.user_id.is_some() {
            len += 1;
        }
        if self.current_stage.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.source.is_some() {
            len += 1;
        }
        if self.run_id.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.status_message.is_some() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        if self.run_link.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ModelVersion", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.creation_timestamp.as_ref() {
            struct_ser.serialize_field("creationTimestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.last_updated_timestamp.as_ref() {
            struct_ser.serialize_field("lastUpdatedTimestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.current_stage.as_ref() {
            struct_ser.serialize_field("currentStage", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.source.as_ref() {
            struct_ser.serialize_field("source", v)?;
        }
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = ModelVersionStatus::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.status_message.as_ref() {
            struct_ser.serialize_field("statusMessage", v)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        if let Some(v) = self.run_link.as_ref() {
            struct_ser.serialize_field("runLink", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "creation_timestamp",
            "creationTimestamp",
            "last_updated_timestamp",
            "lastUpdatedTimestamp",
            "user_id",
            "userId",
            "current_stage",
            "currentStage",
            "description",
            "source",
            "run_id",
            "runId",
            "status",
            "status_message",
            "statusMessage",
            "tags",
            "run_link",
            "runLink",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            CreationTimestamp,
            LastUpdatedTimestamp,
            UserId,
            CurrentStage,
            Description,
            Source,
            RunId,
            Status,
            StatusMessage,
            Tags,
            RunLink,
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
                            "creationTimestamp" | "creation_timestamp" => Ok(GeneratedField::CreationTimestamp),
                            "lastUpdatedTimestamp" | "last_updated_timestamp" => Ok(GeneratedField::LastUpdatedTimestamp),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "currentStage" | "current_stage" => Ok(GeneratedField::CurrentStage),
                            "description" => Ok(GeneratedField::Description),
                            "source" => Ok(GeneratedField::Source),
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "status" => Ok(GeneratedField::Status),
                            "statusMessage" | "status_message" => Ok(GeneratedField::StatusMessage),
                            "tags" => Ok(GeneratedField::Tags),
                            "runLink" | "run_link" => Ok(GeneratedField::RunLink),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ModelVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut creation_timestamp__ = None;
                let mut last_updated_timestamp__ = None;
                let mut user_id__ = None;
                let mut current_stage__ = None;
                let mut description__ = None;
                let mut source__ = None;
                let mut run_id__ = None;
                let mut status__ = None;
                let mut status_message__ = None;
                let mut tags__ = None;
                let mut run_link__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::CreationTimestamp => {
                            if creation_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTimestamp"));
                            }
                            creation_timestamp__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LastUpdatedTimestamp => {
                            if last_updated_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdatedTimestamp"));
                            }
                            last_updated_timestamp__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map.next_value()?;
                        }
                        GeneratedField::CurrentStage => {
                            if current_stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("currentStage"));
                            }
                            current_stage__ = map.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map.next_value()?;
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = map.next_value()?;
                        }
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value::<::std::option::Option<ModelVersionStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::StatusMessage => {
                            if status_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusMessage"));
                            }
                            status_message__ = map.next_value()?;
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                        GeneratedField::RunLink => {
                            if run_link__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runLink"));
                            }
                            run_link__ = map.next_value()?;
                        }
                    }
                }
                Ok(ModelVersion {
                    name: name__,
                    version: version__,
                    creation_timestamp: creation_timestamp__,
                    last_updated_timestamp: last_updated_timestamp__,
                    user_id: user_id__,
                    current_stage: current_stage__,
                    description: description__,
                    source: source__,
                    run_id: run_id__,
                    status: status__,
                    status_message: status_message__,
                    tags: tags__.unwrap_or_default(),
                    run_link: run_link__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ModelVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModelVersionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::PendingRegistration => "PENDING_REGISTRATION",
            Self::FailedRegistration => "FAILED_REGISTRATION",
            Self::Ready => "READY",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ModelVersionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PENDING_REGISTRATION",
            "FAILED_REGISTRATION",
            "READY",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelVersionStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ModelVersionStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ModelVersionStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PENDING_REGISTRATION" => Ok(ModelVersionStatus::PendingRegistration),
                    "FAILED_REGISTRATION" => Ok(ModelVersionStatus::FailedRegistration),
                    "READY" => Ok(ModelVersionStatus::Ready),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ModelVersionTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.ModelVersionTag", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModelVersionTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModelVersionTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.ModelVersionTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ModelVersionTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(ModelVersionTag {
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.ModelVersionTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Param {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.Param", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Param {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Param;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.Param")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Param, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(Param {
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.Param", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RateLimit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_burst.is_some() {
            len += 1;
        }
        if self.max_sustained_per_second.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RateLimit", len)?;
        if let Some(v) = self.max_burst.as_ref() {
            struct_ser.serialize_field("maxBurst", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.max_sustained_per_second.as_ref() {
            struct_ser.serialize_field("maxSustainedPerSecond", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RateLimit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_burst",
            "maxBurst",
            "max_sustained_per_second",
            "maxSustainedPerSecond",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxBurst,
            MaxSustainedPerSecond,
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
                            "maxBurst" | "max_burst" => Ok(GeneratedField::MaxBurst),
                            "maxSustainedPerSecond" | "max_sustained_per_second" => Ok(GeneratedField::MaxSustainedPerSecond),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RateLimit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RateLimit")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RateLimit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_burst__ = None;
                let mut max_sustained_per_second__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxBurst => {
                            if max_burst__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxBurst"));
                            }
                            max_burst__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::MaxSustainedPerSecond => {
                            if max_sustained_per_second__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSustainedPerSecond"));
                            }
                            max_sustained_per_second__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(RateLimit {
                    max_burst: max_burst__,
                    max_sustained_per_second: max_sustained_per_second__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RateLimit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisteredModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.creation_timestamp.is_some() {
            len += 1;
        }
        if self.last_updated_timestamp.is_some() {
            len += 1;
        }
        if self.user_id.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if !self.latest_versions.is_empty() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RegisteredModel", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.creation_timestamp.as_ref() {
            struct_ser.serialize_field("creationTimestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.last_updated_timestamp.as_ref() {
            struct_ser.serialize_field("lastUpdatedTimestamp", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.latest_versions.is_empty() {
            struct_ser.serialize_field("latestVersions", &self.latest_versions)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "creation_timestamp",
            "creationTimestamp",
            "last_updated_timestamp",
            "lastUpdatedTimestamp",
            "user_id",
            "userId",
            "description",
            "latest_versions",
            "latestVersions",
            "tags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CreationTimestamp,
            LastUpdatedTimestamp,
            UserId,
            Description,
            LatestVersions,
            Tags,
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
                            "creationTimestamp" | "creation_timestamp" => Ok(GeneratedField::CreationTimestamp),
                            "lastUpdatedTimestamp" | "last_updated_timestamp" => Ok(GeneratedField::LastUpdatedTimestamp),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "description" => Ok(GeneratedField::Description),
                            "latestVersions" | "latest_versions" => Ok(GeneratedField::LatestVersions),
                            "tags" => Ok(GeneratedField::Tags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisteredModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RegisteredModel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RegisteredModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut creation_timestamp__ = None;
                let mut last_updated_timestamp__ = None;
                let mut user_id__ = None;
                let mut description__ = None;
                let mut latest_versions__ = None;
                let mut tags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::CreationTimestamp => {
                            if creation_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationTimestamp"));
                            }
                            creation_timestamp__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::LastUpdatedTimestamp => {
                            if last_updated_timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdatedTimestamp"));
                            }
                            last_updated_timestamp__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map.next_value()?;
                        }
                        GeneratedField::LatestVersions => {
                            if latest_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latestVersions"));
                            }
                            latest_versions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RegisteredModel {
                    name: name__,
                    creation_timestamp: creation_timestamp__,
                    last_updated_timestamp: last_updated_timestamp__,
                    user_id: user_id__,
                    description: description__,
                    latest_versions: latest_versions__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RegisteredModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RegisteredModelTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RegisteredModelTag", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegisteredModelTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegisteredModelTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RegisteredModelTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RegisteredModelTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(RegisteredModelTag {
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RegisteredModelTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RenameRegisteredModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.new_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RenameRegisteredModel", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.new_name.as_ref() {
            struct_ser.serialize_field("newName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RenameRegisteredModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "new_name",
            "newName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            NewName,
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
                            "newName" | "new_name" => Ok(GeneratedField::NewName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RenameRegisteredModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RenameRegisteredModel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RenameRegisteredModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut new_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::NewName => {
                            if new_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newName"));
                            }
                            new_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(RenameRegisteredModel {
                    name: name__,
                    new_name: new_name__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RenameRegisteredModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for rename_registered_model::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.registered_model.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RenameRegisteredModel.Response", len)?;
        if let Some(v) = self.registered_model.as_ref() {
            struct_ser.serialize_field("registeredModel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for rename_registered_model::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "registered_model",
            "registeredModel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredModel,
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
                            "registeredModel" | "registered_model" => Ok(GeneratedField::RegisteredModel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = rename_registered_model::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RenameRegisteredModel.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<rename_registered_model::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut registered_model__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredModel => {
                            if registered_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredModel"));
                            }
                            registered_model__ = map.next_value()?;
                        }
                    }
                }
                Ok(rename_registered_model::Response {
                    registered_model: registered_model__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RenameRegisteredModel.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestoreExperiment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RestoreExperiment", len)?;
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestoreExperiment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_id",
            "experimentId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentId,
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
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RestoreExperiment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RestoreExperiment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RestoreExperiment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(RestoreExperiment {
                    experiment_id: experiment_id__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RestoreExperiment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for restore_experiment::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.RestoreExperiment.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for restore_experiment::Response {
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
            type Value = restore_experiment::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RestoreExperiment.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<restore_experiment::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(restore_experiment::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RestoreExperiment.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestoreRun {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RestoreRun", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestoreRun {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RestoreRun;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RestoreRun")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RestoreRun, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(RestoreRun {
                    run_id: run_id__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RestoreRun", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for restore_run::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.RestoreRun.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for restore_run::Response {
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
            type Value = restore_run::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RestoreRun.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<restore_run::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(restore_run::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RestoreRun.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Run {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.info.is_some() {
            len += 1;
        }
        if self.data.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.Run", len)?;
        if let Some(v) = self.info.as_ref() {
            struct_ser.serialize_field("info", v)?;
        }
        if let Some(v) = self.data.as_ref() {
            struct_ser.serialize_field("data", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Run {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "info",
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Info,
            Data,
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
                            "info" => Ok(GeneratedField::Info),
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Run;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.Run")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Run, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut info__ = None;
                let mut data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = map.next_value()?;
                        }
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = map.next_value()?;
                        }
                    }
                }
                Ok(Run {
                    info: info__,
                    data: data__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.Run", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RunData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.metrics.is_empty() {
            len += 1;
        }
        if !self.params.is_empty() {
            len += 1;
        }
        if !self.tags.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RunData", len)?;
        if !self.metrics.is_empty() {
            struct_ser.serialize_field("metrics", &self.metrics)?;
        }
        if !self.params.is_empty() {
            struct_ser.serialize_field("params", &self.params)?;
        }
        if !self.tags.is_empty() {
            struct_ser.serialize_field("tags", &self.tags)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RunData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metrics",
            "params",
            "tags",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metrics,
            Params,
            Tags,
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
                            "metrics" => Ok(GeneratedField::Metrics),
                            "params" => Ok(GeneratedField::Params),
                            "tags" => Ok(GeneratedField::Tags),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RunData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RunData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metrics__ = None;
                let mut params__ = None;
                let mut tags__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Metrics => {
                            if metrics__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metrics"));
                            }
                            metrics__ = Some(map.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = Some(map.next_value()?);
                        }
                        GeneratedField::Tags => {
                            if tags__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tags"));
                            }
                            tags__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RunData {
                    metrics: metrics__.unwrap_or_default(),
                    params: params__.unwrap_or_default(),
                    tags: tags__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RunData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RunInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.run_uuid.is_some() {
            len += 1;
        }
        if self.experiment_id.is_some() {
            len += 1;
        }
        if self.user_id.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.start_time.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        if self.artifact_uri.is_some() {
            len += 1;
        }
        if self.lifecycle_stage.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RunInfo", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.run_uuid.as_ref() {
            struct_ser.serialize_field("runUuid", v)?;
        }
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("userId", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = RunStatus::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.start_time.as_ref() {
            struct_ser.serialize_field("startTime", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.artifact_uri.as_ref() {
            struct_ser.serialize_field("artifactUri", v)?;
        }
        if let Some(v) = self.lifecycle_stage.as_ref() {
            struct_ser.serialize_field("lifecycleStage", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RunInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "run_uuid",
            "runUuid",
            "experiment_id",
            "experimentId",
            "user_id",
            "userId",
            "status",
            "start_time",
            "startTime",
            "end_time",
            "endTime",
            "artifact_uri",
            "artifactUri",
            "lifecycle_stage",
            "lifecycleStage",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            RunUuid,
            ExperimentId,
            UserId,
            Status,
            StartTime,
            EndTime,
            ArtifactUri,
            LifecycleStage,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "runUuid" | "run_uuid" => Ok(GeneratedField::RunUuid),
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            "userId" | "user_id" => Ok(GeneratedField::UserId),
                            "status" => Ok(GeneratedField::Status),
                            "startTime" | "start_time" => Ok(GeneratedField::StartTime),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            "artifactUri" | "artifact_uri" => Ok(GeneratedField::ArtifactUri),
                            "lifecycleStage" | "lifecycle_stage" => Ok(GeneratedField::LifecycleStage),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RunInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RunInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut run_uuid__ = None;
                let mut experiment_id__ = None;
                let mut user_id__ = None;
                let mut status__ = None;
                let mut start_time__ = None;
                let mut end_time__ = None;
                let mut artifact_uri__ = None;
                let mut lifecycle_stage__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::RunUuid => {
                            if run_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runUuid"));
                            }
                            run_uuid__ = map.next_value()?;
                        }
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userId"));
                            }
                            user_id__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value::<::std::option::Option<RunStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::StartTime => {
                            if start_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startTime"));
                            }
                            start_time__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::ArtifactUri => {
                            if artifact_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("artifactUri"));
                            }
                            artifact_uri__ = map.next_value()?;
                        }
                        GeneratedField::LifecycleStage => {
                            if lifecycle_stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycleStage"));
                            }
                            lifecycle_stage__ = map.next_value()?;
                        }
                    }
                }
                Ok(RunInfo {
                    run_id: run_id__,
                    run_uuid: run_uuid__,
                    experiment_id: experiment_id__,
                    user_id: user_id__,
                    status: status__,
                    start_time: start_time__,
                    end_time: end_time__,
                    artifact_uri: artifact_uri__,
                    lifecycle_stage: lifecycle_stage__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RunInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RunStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Running => "RUNNING",
            Self::Scheduled => "SCHEDULED",
            Self::Finished => "FINISHED",
            Self::Failed => "FAILED",
            Self::Killed => "KILLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for RunStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RUNNING",
            "SCHEDULED",
            "FINISHED",
            "FAILED",
            "KILLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(RunStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(RunStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RUNNING" => Ok(RunStatus::Running),
                    "SCHEDULED" => Ok(RunStatus::Scheduled),
                    "FINISHED" => Ok(RunStatus::Finished),
                    "FAILED" => Ok(RunStatus::Failed),
                    "KILLED" => Ok(RunStatus::Killed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RunTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.RunTag", len)?;
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RunTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RunTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.RunTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RunTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(RunTag {
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.RunTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchModelVersions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        if self.max_results.is_some() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SearchModelVersions", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", ToString::to_string(&v).as_str())?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchModelVersions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
            "max_results",
            "maxResults",
            "order_by",
            "orderBy",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filter,
            MaxResults,
            OrderBy,
            PageToken,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchModelVersions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SearchModelVersions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchModelVersions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                let mut max_results__ = None;
                let mut order_by__ = None;
                let mut page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map.next_value()?;
                        }
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(SearchModelVersions {
                    filter: filter__,
                    max_results: max_results__,
                    order_by: order_by__.unwrap_or_default(),
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SearchModelVersions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for search_model_versions::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.model_versions.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SearchModelVersions.Response", len)?;
        if !self.model_versions.is_empty() {
            struct_ser.serialize_field("modelVersions", &self.model_versions)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for search_model_versions::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_versions",
            "modelVersions",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelVersions,
            NextPageToken,
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
                            "modelVersions" | "model_versions" => Ok(GeneratedField::ModelVersions),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = search_model_versions::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SearchModelVersions.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<search_model_versions::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_versions__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModelVersions => {
                            if model_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelVersions"));
                            }
                            model_versions__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(search_model_versions::Response {
                    model_versions: model_versions__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SearchModelVersions.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchRegisteredModels {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.filter.is_some() {
            len += 1;
        }
        if self.max_results.is_some() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SearchRegisteredModels", len)?;
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", ToString::to_string(&v).as_str())?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchRegisteredModels {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "filter",
            "max_results",
            "maxResults",
            "order_by",
            "orderBy",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Filter,
            MaxResults,
            OrderBy,
            PageToken,
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
                            "filter" => Ok(GeneratedField::Filter),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchRegisteredModels;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SearchRegisteredModels")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchRegisteredModels, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut filter__ = None;
                let mut max_results__ = None;
                let mut order_by__ = None;
                let mut page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map.next_value()?;
                        }
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(SearchRegisteredModels {
                    filter: filter__,
                    max_results: max_results__,
                    order_by: order_by__.unwrap_or_default(),
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SearchRegisteredModels", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for search_registered_models::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.registered_models.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SearchRegisteredModels.Response", len)?;
        if !self.registered_models.is_empty() {
            struct_ser.serialize_field("registeredModels", &self.registered_models)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for search_registered_models::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "registered_models",
            "registeredModels",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredModels,
            NextPageToken,
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
                            "registeredModels" | "registered_models" => Ok(GeneratedField::RegisteredModels),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = search_registered_models::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SearchRegisteredModels.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<search_registered_models::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut registered_models__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredModels => {
                            if registered_models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredModels"));
                            }
                            registered_models__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(search_registered_models::Response {
                    registered_models: registered_models__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SearchRegisteredModels.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SearchRuns {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.experiment_ids.is_empty() {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if self.run_view_type.is_some() {
            len += 1;
        }
        if self.max_results.is_some() {
            len += 1;
        }
        if !self.order_by.is_empty() {
            len += 1;
        }
        if self.page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SearchRuns", len)?;
        if !self.experiment_ids.is_empty() {
            struct_ser.serialize_field("experimentIds", &self.experiment_ids)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.run_view_type.as_ref() {
            let v = ViewType::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("runViewType", &v)?;
        }
        if let Some(v) = self.max_results.as_ref() {
            struct_ser.serialize_field("maxResults", v)?;
        }
        if !self.order_by.is_empty() {
            struct_ser.serialize_field("orderBy", &self.order_by)?;
        }
        if let Some(v) = self.page_token.as_ref() {
            struct_ser.serialize_field("pageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SearchRuns {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_ids",
            "experimentIds",
            "filter",
            "run_view_type",
            "runViewType",
            "max_results",
            "maxResults",
            "order_by",
            "orderBy",
            "page_token",
            "pageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentIds,
            Filter,
            RunViewType,
            MaxResults,
            OrderBy,
            PageToken,
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
                            "experimentIds" | "experiment_ids" => Ok(GeneratedField::ExperimentIds),
                            "filter" => Ok(GeneratedField::Filter),
                            "runViewType" | "run_view_type" => Ok(GeneratedField::RunViewType),
                            "maxResults" | "max_results" => Ok(GeneratedField::MaxResults),
                            "orderBy" | "order_by" => Ok(GeneratedField::OrderBy),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SearchRuns;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SearchRuns")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SearchRuns, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_ids__ = None;
                let mut filter__ = None;
                let mut run_view_type__ = None;
                let mut max_results__ = None;
                let mut order_by__ = None;
                let mut page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentIds => {
                            if experiment_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentIds"));
                            }
                            experiment_ids__ = Some(map.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map.next_value()?;
                        }
                        GeneratedField::RunViewType => {
                            if run_view_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runViewType"));
                            }
                            run_view_type__ = map.next_value::<::std::option::Option<ViewType>>()?.map(|x| x as i32);
                        }
                        GeneratedField::MaxResults => {
                            if max_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxResults"));
                            }
                            max_results__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OrderBy => {
                            if order_by__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderBy"));
                            }
                            order_by__ = Some(map.next_value()?);
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(SearchRuns {
                    experiment_ids: experiment_ids__.unwrap_or_default(),
                    filter: filter__,
                    run_view_type: run_view_type__,
                    max_results: max_results__,
                    order_by: order_by__.unwrap_or_default(),
                    page_token: page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SearchRuns", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for search_runs::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.runs.is_empty() {
            len += 1;
        }
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SearchRuns.Response", len)?;
        if !self.runs.is_empty() {
            struct_ser.serialize_field("runs", &self.runs)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for search_runs::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "runs",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Runs,
            NextPageToken,
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
                            "runs" => Ok(GeneratedField::Runs),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = search_runs::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SearchRuns.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<search_runs::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut runs__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Runs => {
                            if runs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runs"));
                            }
                            runs__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = map.next_value()?;
                        }
                    }
                }
                Ok(search_runs::Response {
                    runs: runs__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SearchRuns.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetExperimentTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_id.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SetExperimentTag", len)?;
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetExperimentTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_id",
            "experimentId",
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentId,
            Key,
            Value,
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
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetExperimentTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SetExperimentTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetExperimentTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_id__ = None;
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetExperimentTag {
                    experiment_id: experiment_id__,
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SetExperimentTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for set_experiment_tag::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.SetExperimentTag.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for set_experiment_tag::Response {
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
            type Value = set_experiment_tag::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SetExperimentTag.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<set_experiment_tag::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(set_experiment_tag::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SetExperimentTag.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetModelVersionTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SetModelVersionTag", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetModelVersionTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetModelVersionTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SetModelVersionTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetModelVersionTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetModelVersionTag {
                    name: name__,
                    version: version__,
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SetModelVersionTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for set_model_version_tag::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.SetModelVersionTag.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for set_model_version_tag::Response {
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
            type Value = set_model_version_tag::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SetModelVersionTag.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<set_model_version_tag::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(set_model_version_tag::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SetModelVersionTag.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetRegisteredModelTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SetRegisteredModelTag", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetRegisteredModelTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Key,
            Value,
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
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetRegisteredModelTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SetRegisteredModelTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetRegisteredModelTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetRegisteredModelTag {
                    name: name__,
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SetRegisteredModelTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for set_registered_model_tag::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.SetRegisteredModelTag.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for set_registered_model_tag::Response {
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
            type Value = set_registered_model_tag::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SetRegisteredModelTag.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<set_registered_model_tag::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(set_registered_model_tag::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SetRegisteredModelTag.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetTag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.run_uuid.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.SetTag", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.run_uuid.as_ref() {
            struct_ser.serialize_field("runUuid", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetTag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "run_uuid",
            "runUuid",
            "key",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            RunUuid,
            Key,
            Value,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "runUuid" | "run_uuid" => Ok(GeneratedField::RunUuid),
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetTag;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SetTag")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetTag, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut run_uuid__ = None;
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::RunUuid => {
                            if run_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runUuid"));
                            }
                            run_uuid__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetTag {
                    run_id: run_id__,
                    run_uuid: run_uuid__,
                    key: key__,
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SetTag", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for set_tag::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.SetTag.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for set_tag::Response {
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
            type Value = set_tag::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.SetTag.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<set_tag::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(set_tag::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.SetTag.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SourceType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Notebook => "NOTEBOOK",
            Self::Job => "JOB",
            Self::Project => "PROJECT",
            Self::Local => "LOCAL",
            Self::Unknown => "UNKNOWN",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for SourceType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "NOTEBOOK",
            "JOB",
            "PROJECT",
            "LOCAL",
            "UNKNOWN",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SourceType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(SourceType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(SourceType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "NOTEBOOK" => Ok(SourceType::Notebook),
                    "JOB" => Ok(SourceType::Job),
                    "PROJECT" => Ok(SourceType::Project),
                    "LOCAL" => Ok(SourceType::Local),
                    "UNKNOWN" => Ok(SourceType::Unknown),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TransitionModelVersionStage {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.stage.is_some() {
            len += 1;
        }
        if self.archive_existing_versions.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.TransitionModelVersionStage", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.stage.as_ref() {
            struct_ser.serialize_field("stage", v)?;
        }
        if let Some(v) = self.archive_existing_versions.as_ref() {
            struct_ser.serialize_field("archiveExistingVersions", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransitionModelVersionStage {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "stage",
            "archive_existing_versions",
            "archiveExistingVersions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            Stage,
            ArchiveExistingVersions,
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
                            "stage" => Ok(GeneratedField::Stage),
                            "archiveExistingVersions" | "archive_existing_versions" => Ok(GeneratedField::ArchiveExistingVersions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransitionModelVersionStage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.TransitionModelVersionStage")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TransitionModelVersionStage, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut stage__ = None;
                let mut archive_existing_versions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Stage => {
                            if stage__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stage"));
                            }
                            stage__ = map.next_value()?;
                        }
                        GeneratedField::ArchiveExistingVersions => {
                            if archive_existing_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("archiveExistingVersions"));
                            }
                            archive_existing_versions__ = map.next_value()?;
                        }
                    }
                }
                Ok(TransitionModelVersionStage {
                    name: name__,
                    version: version__,
                    stage: stage__,
                    archive_existing_versions: archive_existing_versions__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.TransitionModelVersionStage", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transition_model_version_stage::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.model_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.TransitionModelVersionStage.Response", len)?;
        if let Some(v) = self.model_version.as_ref() {
            struct_ser.serialize_field("modelVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transition_model_version_stage::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_version",
            "modelVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelVersion,
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
                            "modelVersion" | "model_version" => Ok(GeneratedField::ModelVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transition_model_version_stage::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.TransitionModelVersionStage.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<transition_model_version_stage::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModelVersion => {
                            if model_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelVersion"));
                            }
                            model_version__ = map.next_value()?;
                        }
                    }
                }
                Ok(transition_model_version_stage::Response {
                    model_version: model_version__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.TransitionModelVersionStage.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateExperiment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.experiment_id.is_some() {
            len += 1;
        }
        if self.new_name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.UpdateExperiment", len)?;
        if let Some(v) = self.experiment_id.as_ref() {
            struct_ser.serialize_field("experimentId", v)?;
        }
        if let Some(v) = self.new_name.as_ref() {
            struct_ser.serialize_field("newName", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateExperiment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "experiment_id",
            "experimentId",
            "new_name",
            "newName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExperimentId,
            NewName,
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
                            "experimentId" | "experiment_id" => Ok(GeneratedField::ExperimentId),
                            "newName" | "new_name" => Ok(GeneratedField::NewName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateExperiment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.UpdateExperiment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateExperiment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut experiment_id__ = None;
                let mut new_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExperimentId => {
                            if experiment_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentId"));
                            }
                            experiment_id__ = map.next_value()?;
                        }
                        GeneratedField::NewName => {
                            if new_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newName"));
                            }
                            new_name__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateExperiment {
                    experiment_id: experiment_id__,
                    new_name: new_name__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.UpdateExperiment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for update_experiment::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("mlflow.UpdateExperiment.Response", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for update_experiment::Response {
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
            type Value = update_experiment::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.UpdateExperiment.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<update_experiment::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(update_experiment::Response {
                })
            }
        }
        deserializer.deserialize_struct("mlflow.UpdateExperiment.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateModelVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.version.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.UpdateModelVersion", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.version.as_ref() {
            struct_ser.serialize_field("version", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateModelVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            Description,
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
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateModelVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.UpdateModelVersion")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateModelVersion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut description__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = map.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateModelVersion {
                    name: name__,
                    version: version__,
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.UpdateModelVersion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for update_model_version::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.model_version.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.UpdateModelVersion.Response", len)?;
        if let Some(v) = self.model_version.as_ref() {
            struct_ser.serialize_field("modelVersion", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for update_model_version::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "model_version",
            "modelVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModelVersion,
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
                            "modelVersion" | "model_version" => Ok(GeneratedField::ModelVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = update_model_version::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.UpdateModelVersion.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<update_model_version::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut model_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModelVersion => {
                            if model_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelVersion"));
                            }
                            model_version__ = map.next_value()?;
                        }
                    }
                }
                Ok(update_model_version::Response {
                    model_version: model_version__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.UpdateModelVersion.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRegisteredModel {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.UpdateRegisteredModel", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRegisteredModel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Description,
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
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRegisteredModel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.UpdateRegisteredModel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateRegisteredModel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut description__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map.next_value()?;
                        }
                    }
                }
                Ok(UpdateRegisteredModel {
                    name: name__,
                    description: description__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.UpdateRegisteredModel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for update_registered_model::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.registered_model.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.UpdateRegisteredModel.Response", len)?;
        if let Some(v) = self.registered_model.as_ref() {
            struct_ser.serialize_field("registeredModel", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for update_registered_model::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "registered_model",
            "registeredModel",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RegisteredModel,
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
                            "registeredModel" | "registered_model" => Ok(GeneratedField::RegisteredModel),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = update_registered_model::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.UpdateRegisteredModel.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<update_registered_model::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut registered_model__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RegisteredModel => {
                            if registered_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registeredModel"));
                            }
                            registered_model__ = map.next_value()?;
                        }
                    }
                }
                Ok(update_registered_model::Response {
                    registered_model: registered_model__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.UpdateRegisteredModel.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateRun {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_id.is_some() {
            len += 1;
        }
        if self.run_uuid.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        if self.end_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.UpdateRun", len)?;
        if let Some(v) = self.run_id.as_ref() {
            struct_ser.serialize_field("runId", v)?;
        }
        if let Some(v) = self.run_uuid.as_ref() {
            struct_ser.serialize_field("runUuid", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            let v = RunStatus::from_i32(*v)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", *v)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.end_time.as_ref() {
            struct_ser.serialize_field("endTime", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateRun {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_id",
            "runId",
            "run_uuid",
            "runUuid",
            "status",
            "end_time",
            "endTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunId,
            RunUuid,
            Status,
            EndTime,
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
                            "runId" | "run_id" => Ok(GeneratedField::RunId),
                            "runUuid" | "run_uuid" => Ok(GeneratedField::RunUuid),
                            "status" => Ok(GeneratedField::Status),
                            "endTime" | "end_time" => Ok(GeneratedField::EndTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateRun;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.UpdateRun")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateRun, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_id__ = None;
                let mut run_uuid__ = None;
                let mut status__ = None;
                let mut end_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunId => {
                            if run_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runId"));
                            }
                            run_id__ = map.next_value()?;
                        }
                        GeneratedField::RunUuid => {
                            if run_uuid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runUuid"));
                            }
                            run_uuid__ = map.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map.next_value::<::std::option::Option<RunStatus>>()?.map(|x| x as i32);
                        }
                        GeneratedField::EndTime => {
                            if end_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endTime"));
                            }
                            end_time__ =
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(UpdateRun {
                    run_id: run_id__,
                    run_uuid: run_uuid__,
                    status: status__,
                    end_time: end_time__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.UpdateRun", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for update_run::Response {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.run_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("mlflow.UpdateRun.Response", len)?;
        if let Some(v) = self.run_info.as_ref() {
            struct_ser.serialize_field("runInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for update_run::Response {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "run_info",
            "runInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RunInfo,
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
                            "runInfo" | "run_info" => Ok(GeneratedField::RunInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = update_run::Response;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct mlflow.UpdateRun.Response")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<update_run::Response, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut run_info__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RunInfo => {
                            if run_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("runInfo"));
                            }
                            run_info__ = map.next_value()?;
                        }
                    }
                }
                Ok(update_run::Response {
                    run_info: run_info__,
                })
            }
        }
        deserializer.deserialize_struct("mlflow.UpdateRun.Response", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ViewType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::ActiveOnly => "ACTIVE_ONLY",
            Self::DeletedOnly => "DELETED_ONLY",
            Self::All => "ALL",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ViewType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTIVE_ONLY",
            "DELETED_ONLY",
            "ALL",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ViewType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ViewType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ViewType::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ACTIVE_ONLY" => Ok(ViewType::ActiveOnly),
                    "DELETED_ONLY" => Ok(ViewType::DeletedOnly),
                    "ALL" => Ok(ViewType::All),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Visibility {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Public => "PUBLIC",
            Self::Internal => "INTERNAL",
            Self::PublicUndocumented => "PUBLIC_UNDOCUMENTED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Visibility {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PUBLIC",
            "INTERNAL",
            "PUBLIC_UNDOCUMENTED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Visibility;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(Visibility::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(Visibility::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PUBLIC" => Ok(Visibility::Public),
                    "INTERNAL" => Ok(Visibility::Internal),
                    "PUBLIC_UNDOCUMENTED" => Ok(Visibility::PublicUndocumented),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
