use core::panic;

use serde::de::{self, Visitor};

use super::*;

impl Config {
    fn new() -> Self {
        let config_dir = match Self::check() {
            Ok(p) => p,
            Err(e) => {
                panic!("Issue with checking config dir: {e}.")
            }
        };
        let config = match config::Config::builder()
            .add_source(
                config::File::from(config_dir)
                    .format(config::FileFormat::Toml)
                    .required(false),
            )
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                panic!("Issue building config struct: {e}.")
            }
        };
        match config.try_deserialize::<Self>() {
            Ok(c) => c,
            Err(e) => {
                panic!("Issue deserializing config file: {e}")
            }
        }
    }
    pub fn get_config() -> &'static Self {
        CONFIG.get_or_init(Self::new)
    }
}

impl<'de> Deserialize<'de> for KeyMap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct KeyMapVisitor;

        impl<'de> Visitor<'de> for KeyMapVisitor {
            type Value = KeyMap;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(formatter, "A hashmap of String -> Action pairs")
            }

            fn visit_map<A>(
                self,
                mut map: A,
            ) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut keymap = HashMap::new();

                while let Some((key_str, action)) =
                    map.next_entry::<String, Action>()?
                {
                    let key_event =
                        parse_key_event(&key_str).map_err(de::Error::custom)?;
                    keymap.insert(key_event, action);
                }
                Ok(KeyMap(keymap))
            }
        }
        deserializer.deserialize_map(KeyMapVisitor)
    }
}
