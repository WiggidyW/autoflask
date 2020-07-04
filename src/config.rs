use super::*;
use serde::Deserialize;
use std::time::Duration;
use std::convert::TryFrom;
use std::path::Path;
use serde_yaml::Value as YamlValue;
use serde_yaml::Number as YamlNumber;

#[derive(Deserialize)]
pub struct FlasksConfig {
    key: String,
    flasks: [FlaskConfig; 5],
}

#[derive(Clone, Deserialize)]
pub struct FlaskConfig {
    timing: Option<YamlValue>,
    after: Option<YamlValue>,
    sleep_interval: [u64; 2],
    key: String,
}

impl FlasksConfig {
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Self {
        let file = std::fs::File::open(path).unwrap();
        serde_yaml::from_reader(file).unwrap()
    }
    pub fn into_flasks_and_key<K: Key + TryFrom<String>>(self) -> (Flasks<K>, K) {
        let config_to_flask = |config: FlaskConfig, order: Ordering| {
            Flask::new(
                match config.timing {
                    Some(YamlValue::Number(n)) if n.is_u64() => Some(Duration::from_millis(n.as_u64().unwrap())),
                    Some(YamlValue::Null) | None => None,
                    other => panic!("Invalid Config Timing value: {:?}", other),
                },
                order,
                match config.after {
                    Some(YamlValue::Number(n)) if n == YamlNumber::from(1) => Some(Ordering::First),
                    Some(YamlValue::Number(n)) if n == YamlNumber::from(2) => Some(Ordering::Second),
                    Some(YamlValue::Number(n)) if n == YamlNumber::from(3) => Some(Ordering::Third),
                    Some(YamlValue::Number(n)) if n == YamlNumber::from(4) => Some(Ordering::Fourth),
                    Some(YamlValue::Number(n)) if n == YamlNumber::from(5) => Some(Ordering::Fifth),
                    Some(YamlValue::Null) | None => None,
                    other => panic!("Invalid Config After value: {:?}", other),
                },
                (config.sleep_interval[0], config.sleep_interval[1]),
                match K::try_from(config.key.clone()) {
                    Ok(k) => k,
                    Err(_) => panic!("Invalid Key in Config: {}", &config.key),
                }
            )
        };
        let key = match K::try_from(self.key.clone()) {
            Ok(k) => k,
            Err(_) => panic!("Invalid Key in Config: {}", &self.key),
        };
        let flasks = Flasks::new([
            config_to_flask(self.flasks[0].clone(), Ordering::First),
            config_to_flask(self.flasks[1].clone(), Ordering::Second),
            config_to_flask(self.flasks[2].clone(), Ordering::Third),
            config_to_flask(self.flasks[3].clone(), Ordering::Fourth),
            config_to_flask(self.flasks[4].clone(), Ordering::Fifth),
        ]);
        (flasks, key)
    }
}