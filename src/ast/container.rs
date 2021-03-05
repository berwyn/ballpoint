use serde::ser::SerializeSeq;

pub(crate) struct Container {
    entries: Vec<super::Entry>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl serde::Serialize for Container {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        for entry in &self.entries {
            seq.serialize_element(entry)?;
        }

        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_empty() {
        let container = Container::default();

        let json = serde_json::to_value(&container).expect("Unable to serialize");

        let expected = serde_json::json!([]);

        pretty_assertions::assert_eq!(expected, json);
    }
}
