pub(crate) enum Entry {
    Evaluation,
    EndEvaluation,
    Str,
    EndStr,
    String(String),
    Newline,
    End,
}

impl serde::Serialize for Entry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Evaluation => serializer.serialize_str("ev"),
            Self::EndEvaluation => serializer.serialize_str("/ev"),
            Self::Str => serializer.serialize_str("str"),
            Self::EndStr => serializer.serialize_str("/str"),
            Self::String(content) => serializer.serialize_str(&format!("^{}", content)),
            Self::Newline => serializer.serialize_str("\n"),
            Self::End => serializer.serialize_str("done"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_evaluation() {
        pretty_assertions::assert_eq!(
            r#""ev""#,
            serde_json::to_string(&Entry::Evaluation).expect("Unable to serialize")
        );
    }

    #[test]
    fn it_serializes_end_evaluation() {
        pretty_assertions::assert_eq!(
            r#""/ev""#,
            serde_json::to_string(&Entry::EndEvaluation).expect("Unable to serialize")
        );
    }

    #[test]
    fn it_serializes_str() {
        pretty_assertions::assert_eq!(
            r#""str""#,
            serde_json::to_string(&Entry::Str).expect("Unable to serialize")
        );
    }

    #[test]
    fn it_serializes_end_str() {
        pretty_assertions::assert_eq!(
            r#""/str""#,
            serde_json::to_string(&Entry::EndStr).expect("Unable to serialize")
        );
    }

    #[test]
    fn it_serializes_strings() {
        pretty_assertions::assert_eq!(
            r#""^test""#,
            serde_json::to_string(&Entry::String("test".into())).expect("Unable to serialize")
        );
    }

    #[test]
    fn it_serializes_newlines() {
        pretty_assertions::assert_eq!(
            r#""\n""#,
            serde_json::to_string(&Entry::Newline).expect("Unable to serialize")
        );
    }

    #[test]
    fn it_serializes_ends() {
        pretty_assertions::assert_eq!(
            r#""done""#,
            serde_json::to_string(&Entry::End).expect("Unable to serialize")
        );
    }
}
