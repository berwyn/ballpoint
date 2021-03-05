use serde::ser::SerializeMap;

use super::Container;

pub struct Story {
    ink_version: u16,
    root: Container,
}

impl serde::Serialize for Story {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("inkVersion", &self.ink_version)?;
        map.serialize_entry("root", &self.root)?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_serialize() {
        let story = Story {
            ink_version: 20,
            root: Default::default(),
        };

        let json = serde_json::to_value(&story).expect("Unable to serialize story");

        let expected = serde_json::json!({
            "inkVersion": 20,
            "root": []
        });

        pretty_assertions::assert_eq!(expected, json);
    }
}
