use std::rc::Rc;

use config::{Config, File, FileFormat};
use serde::de::Visitor;

use super::*;

const DEFAULT_DESC: &str = include_str!("../../data/action_desc.toml");

impl HelpMap {
    pub fn try_init(config_dir: PathBuf) -> Result<Self> {
        // deserialize the config files (Action -> RawStrings)
        let action_keycodes = Config::builder()
            .add_source(File::from(config_dir).format(FileFormat::Toml))
            .build()?
            .try_deserialize::<ActionKey>()?;
        // deserialize descriptions (Action -> descriptions)
        let action_desc = Config::builder()
            .add_source(File::from_str(DEFAULT_DESC, FileFormat::Toml))
            .build()?
            .try_deserialize::<ActionString>()?;
        // Merge (RawStrings, descrpiptionsr) to Action Dictionary
        let res = action_keycodes.0.into_iter().zip(action_desc.0).fold(
            BTreeMap::<Action, ActionDictionary>::new(),
            |mut acc, ((action1, key), (action2, desc))| {
                acc.entry(action1)
                    .and_modify(|dict| dict.add_keycode(key.as_ref()))
                    .or_insert(
                        ActionDictionary::default().with_keycode(key.as_ref()),
                    );
                acc.entry(action2)
                    .and_modify(|dict| dict.add_desc(desc.as_ref()))
                    .or_insert(
                        ActionDictionary::default().with_desc(desc.as_ref()),
                    );
                acc
            },
        );

        Ok(Self(res))
    }
    pub fn get() -> Option<&'static Self> {
        Some(&CONFIG.get()?.helpmap)
    }
    pub fn get_key_str(&self, action: Action) -> Option<&str> {
        self.0.get(&action).map(|f| f.raw_keycode.as_ref())
    }
    pub fn get_lines() -> Vec<Rc<str>> {
        let Some(helpmap) = Self::get() else {
            return Vec::new();
        };
        helpmap
            .0
            .values()
            .map(|dict| {
                let line =
                    format!("{}: {}", dict.raw_keycode, dict.descrpition);
                line.into()
            })
            .collect()
    }
}

impl ActionDictionary {
    fn with_keycode(mut self, keycode: &str) -> Self {
        self.raw_keycode = keycode.into();
        self
    }
    fn with_desc(mut self, desc: &str) -> Self {
        self.descrpition = desc.into();
        self
    }
    fn add_keycode(&mut self, keycode: &str) {
        self.raw_keycode = keycode.into()
    }
    fn add_desc(&mut self, desc: &str) {
        self.descrpition = desc.into()
    }
}

#[derive(Debug, Deserialize)]
/// Itermediate struct for deserializing keystrings and descriptons
struct ActionString(HashMap<Action, String>);

/// Itermediate struct for deserializing keymap config
#[derive(Debug)]
struct ActionKey(HashMap<Action, String>);

struct ActionKeyVisitor;

impl<'de> Deserialize<'de> for ActionKey {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ActionKeyVisitor)
    }
}

impl<'de> Visitor<'de> for ActionKeyVisitor {
    type Value = ActionKey;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "A hashmap of Action -> Key Code pairs")
    }

    fn visit_map<A>(
        self,
        mut map: A,
    ) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut res_map = HashMap::new();

        while let Some((action, strs)) =
            map.next_entry::<Action, Vec<String>>()?
        {
            let merged_key_str = strs.join(", ");
            res_map.insert(action, merged_key_str);
        }
        Ok(ActionKey(res_map))
    }
}
