use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Service {
    name: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
    checks: Vec<Check>,
}

#[derive(Deserialize, Debug)]
struct Check {
    name: String,
    checker: Checker,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Checker {
    Dummy,
    TCP { host: String, port: u16 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_deserializes_from_minimal_json() {
        let json = r#"{
            "name": "my svc",
            "checks": []
        }"#;

        let actual: Service = serde_json::from_str(&json).unwrap();

        assert_eq!(actual.name, "my svc");
    }

    #[test]
    fn service_deserializes_from_complete_json() {
        let json = r#"{
            "name": "sn",
            "description": "sd",
            "tags": ["t1", "t2"],
            "checks": [
                {
                    "name": "cn1",
                    "checker": "dummy"
                },
                {
                    "name": "cn2",
                    "checker": {
                        "tcp": {
                            "host": "localhost",
                            "port": 22
                        }
                    }
                }
            ]
        }"#;

        let actual: Service = serde_json::from_str(&json).unwrap();

        assert_eq!(actual.name, "sn");
        assert_eq!(actual.description, Some("sd".to_string()));
        let actual_tags = actual.tags.expect("tags should not be None");
        assert_eq!(actual_tags.len(), 2);
        assert_eq!(actual_tags[0], "t1");
        assert_eq!(actual_tags[1], "t2");
    }
}
