use serde::de::{self, Visitor};

use super::*;

pub const DEFAULT_KEYMAP: &[u8] =
    include_bytes!("../../data/keymap_default.toml");

impl KeyMap {
    pub(super) fn try_init(config_dir: &Path) -> Result<Self> {
        let config_src = config::Config::builder()
            .add_source(
                config::File::from(config_dir)
                    .format(config::FileFormat::Toml)
                    .required(false),
            )
            .build()?;

        Ok(config_src.try_deserialize::<KeyMap>()?)
    }
    pub fn get() -> Option<&'static Self> {
        Some(&CONFIG.get()?.keymap)
    }
    pub fn get_action(&self, key: KeyEvent) -> Option<Action> {
        self.0.get(&key).copied()
    }
}

impl<'de> Deserialize<'de> for KeyMap {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(KeyMapVisitor)
    }
}

struct KeyMapVisitor;

impl<'de> Visitor<'de> for KeyMapVisitor {
    type Value = KeyMap;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "A hashmap of KeyCode -> Action pairs")
    }

    fn visit_map<A>(
        self,
        mut map: A,
    ) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut keymap = HashMap::new();

        while let Some((action, strs)) =
            map.next_entry::<Action, Vec<String>>()?
        {
            for str in strs {
                let key_event =
                    parse_key_event(&str).map_err(de::Error::custom)?;
                keymap.insert(key_event, action);
            }
        }
        Ok(KeyMap(keymap))
    }
}
