use serde::{Deserialize, Serialize};
use serde_json::{Map, Result, Value};

use super::{config::PHP_API, misc::read_from_web};

#[derive(Serialize, Deserialize, Debug)]
pub struct Zip {
    pub path: String,
    pub size: String,
    pub sha256: String,
}
impl Zip {
    fn get_paths(values: &Map<String, Value>) -> anyhow::Result<Zip> {
        let keys = vec!["zip"];

        for key in keys {
            if let Some(value) = values.get(key) {
                // Użyj `serde_json::from_value` do deserializacji z `Value`
                let json: Zip = serde_json::from_value(value.clone())?;
                return Ok(json);
            }
        }
        Err(anyhow::anyhow!("Key not found in JSON"))
    }
}
pub struct PhpVersion {
    pub version: String,
    pub path: Vec<Zip>,
}
impl PhpVersion {
    pub fn get_versions() -> anyhow::Result<Vec<PhpVersion>> {
        let mut versions: Vec<PhpVersion> = vec![];
        let json: Value = serde_json::from_str(&read_from_web(PHP_API.to_string())?)?;
        if let Some(json_main) = json.as_object() {
            for (_main_key, main_values) in json_main {
                let mut version_value: Option<String> = None;
                let mut path_value: Vec<Zip> = vec![];

                let version: Value = serde_json::from_str(&main_values.to_string())?;
                if let Some(json_version_value) = version.as_object() {
                    for (json_version_keys, json_version_values) in json_version_value {
                        match json_version_keys.as_str() {
                            "version" => {
                                if let Some(version_val) = json_version_values.as_str() {
                                    version_value = Some(version_val.to_string())
                                }
                            }
                            s if s.starts_with("ts") => {
                                if let Some(path_val) = json_version_values.as_object() {
                                    path_value.push(Zip::get_paths(path_val)?);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                match (version_value, path_value) {
                    (Some(ver), path) => {
                        versions.push(PhpVersion {
                            version: ver,
                            path: path,
                        });
                    }
                    _ => todo!(),
                }
            }
        }
        Ok(versions)
    }
}
struct Php {
    versions: Vec<PhpVersion>,
}
