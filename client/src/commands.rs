use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Command {
    GetPing,
    ListContainers,
    StartContainer {
        #[serde(rename = "container")]
        container: String
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::GetPing => {
                write!(f, "Get Ping")
            },
            _ => {
                write!(f, "Not implementerd")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::commands::Command;

    #[test]
    fn serialize() {
        let command = Command::GetPing;
        let result = serde_json::to_string(&command);

        assert_eq!(String::from(r#""GetPing""#), result.expect(""));
    }

    #[test]
    fn serialize_struct() {
        let command = Command::StartContainer {
            container: String::from("test.container")
        };
        let result = serde_json::to_string(&command);

        assert_eq!(String::from(r#"{"StartContainer":{"container":"test.container"}}"#), result.expect(""));
    }

    #[test]
    fn deserialize_struct() {
        let s = r#"{"StartContainer":{"container":"test.container"}}"#;
        let result = serde_json::from_str::<Command>(s);

        assert_eq!(Command::StartContainer {
            container: String::from("test.container")
        }, result.expect(""));
    }

    #[test]
    fn deserialize() {
        let s = r#""GetPing""#;
        let result = serde_json::from_str::<Command>(s);

        assert_eq!(Command::GetPing, result.expect(""));
    }
}
