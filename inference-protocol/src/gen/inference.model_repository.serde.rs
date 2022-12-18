// @generated
impl serde::Serialize for RepositoryIndexRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.repository_name.is_empty() {
            len += 1;
        }
        if self.ready {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.model_repository.RepositoryIndexRequest", len)?;
        if !self.repository_name.is_empty() {
            struct_ser.serialize_field("repositoryName", &self.repository_name)?;
        }
        if self.ready {
            struct_ser.serialize_field("ready", &self.ready)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryIndexRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "repository_name",
            "repositoryName",
            "ready",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RepositoryName,
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
                            "repositoryName" | "repository_name" => Ok(GeneratedField::RepositoryName),
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
            type Value = RepositoryIndexRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.model_repository.RepositoryIndexRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RepositoryIndexRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut repository_name__ = None;
                let mut ready__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RepositoryName => {
                            if repository_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repositoryName"));
                            }
                            repository_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ready => {
                            if ready__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ready"));
                            }
                            ready__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RepositoryIndexRequest {
                    repository_name: repository_name__.unwrap_or_default(),
                    ready: ready__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.model_repository.RepositoryIndexRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RepositoryIndexResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.models.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.model_repository.RepositoryIndexResponse", len)?;
        if !self.models.is_empty() {
            struct_ser.serialize_field("models", &self.models)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryIndexResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "models",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Models,
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
                            "models" => Ok(GeneratedField::Models),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RepositoryIndexResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.model_repository.RepositoryIndexResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RepositoryIndexResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut models__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Models => {
                            if models__.is_some() {
                                return Err(serde::de::Error::duplicate_field("models"));
                            }
                            models__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RepositoryIndexResponse {
                    models: models__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.model_repository.RepositoryIndexResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for repository_index_response::ModelIndex {
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
        if !self.state.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.model_repository.RepositoryIndexResponse.ModelIndex", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.state.is_empty() {
            struct_ser.serialize_field("state", &self.state)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for repository_index_response::ModelIndex {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "version",
            "state",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Version,
            State,
            Reason,
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
                            "state" => Ok(GeneratedField::State),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = repository_index_response::ModelIndex;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.model_repository.RepositoryIndexResponse.ModelIndex")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<repository_index_response::ModelIndex, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut version__ = None;
                let mut state__ = None;
                let mut reason__ = None;
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
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(repository_index_response::ModelIndex {
                    name: name__.unwrap_or_default(),
                    version: version__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.model_repository.RepositoryIndexResponse.ModelIndex", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RepositoryModelLoadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.repository_name.is_empty() {
            len += 1;
        }
        if !self.model_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.model_repository.RepositoryModelLoadRequest", len)?;
        if !self.repository_name.is_empty() {
            struct_ser.serialize_field("repositoryName", &self.repository_name)?;
        }
        if !self.model_name.is_empty() {
            struct_ser.serialize_field("modelName", &self.model_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryModelLoadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "repository_name",
            "repositoryName",
            "model_name",
            "modelName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RepositoryName,
            ModelName,
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
                            "repositoryName" | "repository_name" => Ok(GeneratedField::RepositoryName),
                            "modelName" | "model_name" => Ok(GeneratedField::ModelName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RepositoryModelLoadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.model_repository.RepositoryModelLoadRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RepositoryModelLoadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut repository_name__ = None;
                let mut model_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RepositoryName => {
                            if repository_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repositoryName"));
                            }
                            repository_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModelName => {
                            if model_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelName"));
                            }
                            model_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RepositoryModelLoadRequest {
                    repository_name: repository_name__.unwrap_or_default(),
                    model_name: model_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.model_repository.RepositoryModelLoadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RepositoryModelLoadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("inference.model_repository.RepositoryModelLoadResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryModelLoadResponse {
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
            type Value = RepositoryModelLoadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.model_repository.RepositoryModelLoadResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RepositoryModelLoadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RepositoryModelLoadResponse {
                })
            }
        }
        deserializer.deserialize_struct("inference.model_repository.RepositoryModelLoadResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RepositoryModelUnloadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.repository_name.is_empty() {
            len += 1;
        }
        if !self.model_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("inference.model_repository.RepositoryModelUnloadRequest", len)?;
        if !self.repository_name.is_empty() {
            struct_ser.serialize_field("repositoryName", &self.repository_name)?;
        }
        if !self.model_name.is_empty() {
            struct_ser.serialize_field("modelName", &self.model_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryModelUnloadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "repository_name",
            "repositoryName",
            "model_name",
            "modelName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RepositoryName,
            ModelName,
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
                            "repositoryName" | "repository_name" => Ok(GeneratedField::RepositoryName),
                            "modelName" | "model_name" => Ok(GeneratedField::ModelName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RepositoryModelUnloadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.model_repository.RepositoryModelUnloadRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RepositoryModelUnloadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut repository_name__ = None;
                let mut model_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RepositoryName => {
                            if repository_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repositoryName"));
                            }
                            repository_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModelName => {
                            if model_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelName"));
                            }
                            model_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RepositoryModelUnloadRequest {
                    repository_name: repository_name__.unwrap_or_default(),
                    model_name: model_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("inference.model_repository.RepositoryModelUnloadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RepositoryModelUnloadResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("inference.model_repository.RepositoryModelUnloadResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryModelUnloadResponse {
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
            type Value = RepositoryModelUnloadResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct inference.model_repository.RepositoryModelUnloadResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RepositoryModelUnloadResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RepositoryModelUnloadResponse {
                })
            }
        }
        deserializer.deserialize_struct("inference.model_repository.RepositoryModelUnloadResponse", FIELDS, GeneratedVisitor)
    }
}
