use serde::Deserialize;

/// Represents a service our users wish to monitor.  A service can be composed of multiple checks
/// against multiple hosts.
/// 
/// The Uptimer configuration will be comprised of a list of services to monitor.
#[derive(Deserialize, Debug)]
struct Service {
    name: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
    checks: Vec<Check>,
    hosts: Vec<String>,
}

/// A named check to perform against each host in a service.
///
/// `Checker` is flattened so that the JSON will be
///
///     "name": "my check",
///     "tcp": { "port": 22 }
///
/// instead of
///
///     "name": "my check",
///     "checker": { "tcp": { "port": 22 } }
#[derive(Deserialize, Debug)]
struct Check {
    name: String,

    #[serde(flatten)]
    checker: Checker,
}

/// The type and coniguration of a specific check.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Checker {
    Dummy {},
    TCP { port: u16 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_deserializes_from_minimal_json() {
        let json = r#"{
            "name": "my svc",
            "checks": [],
            "hosts": []
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
                    "dummy": {}
                },
                {
                    "name": "cn2",
                    "tcp": { "port": 22 }
                }
            ],
            "hosts": [
                "localhost"
            ]
        }"#;

        let actual: Service = serde_json::from_str(&json).unwrap();

        assert_eq!(actual.name, "sn");
        assert_eq!(actual.description, Some("sd".to_string()));

        // Verify tags.
        let actual_tags = actual.tags.expect("tags should not be None");
        assert_eq!(actual_tags.len(), 2);
        assert_eq!(actual_tags[0], "t1");
        assert_eq!(actual_tags[1], "t2");

        // Verify checks.
        assert_eq!(actual.checks.len(), 2);

        let cn1 = &actual.checks[0];
        assert_eq!(cn1.name, "cn1");
        assert_eq!(cn1.checker, Checker::Dummy {});

        let cn2 = &actual.checks[1];
        assert_eq!(cn2.name, "cn2");
        assert_eq!(cn2.checker, Checker::TCP { port: 22 });

        // Verify hosts.
        assert_eq!(actual.hosts.len(), 1);
        assert_eq!(actual.hosts[0], "localhost");
    }
}
