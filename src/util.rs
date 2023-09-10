pub mod serializers {
    use chrono::Duration;
    use serde::{ser::SerializeTuple, Serializer};

    pub fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert the chrono::Duration to a tuple (hours, minutes, seconds, milliseconds).
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() - hours * 60;
        let seconds = duration.num_seconds() - (hours * 3600 + minutes * 60);
        let milliseconds =
            duration.num_milliseconds() - (hours * 3600_000 + minutes * 60_000 + seconds * 1000);

        // Create a tuple serializer with 4 elements (hours, minutes, seconds, milliseconds).
        let mut tuple_serializer = serializer.serialize_tuple(4)?;

        tuple_serializer.serialize_element(&hours)?;
        tuple_serializer.serialize_element(&minutes)?;
        tuple_serializer.serialize_element(&seconds)?;
        tuple_serializer.serialize_element(&milliseconds)?;

        tuple_serializer.end()
    }
}

pub mod wrappers {
    use serde::{ser::SerializeTuple, Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Debug, Copy, Clone)]
    pub struct SerDeDuration(pub chrono::Duration);

    impl Serialize for SerDeDuration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            // Convert the chrono::Duration to a tuple (hours, minutes, seconds, milliseconds).
            let hours = self.0.num_hours();
            let minutes = self.0.num_minutes() - hours * 60;
            let seconds = self.0.num_seconds() - (hours * 3600 + minutes * 60);
            let milliseconds =
                self.0.num_milliseconds() - (hours * 3600_000 + minutes * 60_000 + seconds * 1000);

            // Create a tuple serializer with 4 elements (hours, minutes, seconds, milliseconds).
            let mut tuple_serializer = serializer.serialize_tuple(4)?;

            tuple_serializer.serialize_element(&hours)?;
            tuple_serializer.serialize_element(&minutes)?;
            tuple_serializer.serialize_element(&seconds)?;
            tuple_serializer.serialize_element(&milliseconds)?;

            tuple_serializer.end()
        }
    }

    impl<'de> Deserialize<'de> for SerDeDuration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            match s.parse::<iso8601_duration::Duration>() {
                Ok(duration) => Ok(SerDeDuration(
                    chrono::Duration::from_std(duration.to_std().unwrap()).unwrap(), // TODO: do this feckin proper
                )),
                Err(_) => Err(serde::de::Error::custom("Invalid duration format")),
            }
        }
    }
}
