use serde::de::{self, Visitor};

use super::*;

impl Config {
    pub fn new() -> Result<Self> {
        config_check()?;

        // TODO merge a default config with any kv's (if any) from a config file.
        let config = config::Config::builder()
            .add_source(
                config::File::from(config_dir()?)
                    .format(config::FileFormat::Toml)
                    .required(false),
            )
            .build()?;

        Ok(config.try_deserialize::<Self>()?)
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
