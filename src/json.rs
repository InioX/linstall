use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Config {
    distribution: Distribution,
    init: InitSystem,
    partitions: Vec<Partition>,
    packages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Partition {
    uuid: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Distribution {
    Arch,
    Artix,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum InitSystem {
    Runit,
    Openrc,
    S6,
    SixtySix,
    Dinit,
    Systemd,
}

pub fn read_config(data: &str) -> Config {
    let config: Config = serde_json::from_str(data).unwrap();

    return config;
}

pub fn write_config(
    distribution: Distribution,
    init: InitSystem,
    partitions: Vec<Partition>,
    packages: Vec<&str>,
) -> Value {
    return json!({
        "distribution": distribution,
        "init": init,
        "partitions": partitions,
        "packages": packages,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_read_and_write_equal() {
        let data = r#"
        {
            "distribution": "Artix",
            "init": "Runit",
            "partitions": [
                {
                    "uuid": "uuid1",
                    "path": "path1"
                },
                {
                    "uuid": "uuid2",
                    "path": "path2"
                }
            ],
            "packages": [
                "package1",
                "package2"
            ]
        }"#;

        let distribution = Distribution::Artix;
        let init = InitSystem::Runit;
        let partitions: Vec<Partition> = vec![
            Partition {
                uuid: "uuid1".to_string(),
                path: "path1".to_string(),
            },
            Partition {
                uuid: "uuid2".to_string(),
                path: "path2".to_string(),
            },
        ];
        let packages = vec!["package1", "package2"];

        let config1 = read_config(data);
        let config2 = write_config(distribution, init, partitions, packages);

        assert_eq!(
            config1,
            serde_json::from_str(config2.to_string().as_str()).unwrap(),
        )
    }
}
