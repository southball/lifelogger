use derive_more::Display;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use uuid::{Uuid, Version};

#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Hash)]
pub struct TodoID(pub String);

#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Hash)]
pub struct TopicID(pub String);
#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Hash)]
pub struct EventID(pub String);
#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Hash)]
pub struct ActivityID(pub String);

pub trait Abbreviatable {
    fn is_abbreviated_by(&self, abbreviation: &str) -> bool;
    fn abbreviate(&self) -> &str;
}

macro_rules! derive_properties_for_id {
    ($type: tt, $visitor_type: tt, $prefix: tt) => {
        impl Abbreviatable for $type {
            fn is_abbreviated_by(&self, prefix: &str) -> bool {
                self.0[$prefix.len() + 1..]
                    .to_lowercase()
                    .starts_with(&prefix.to_lowercase())
            }

            fn abbreviate(&self) -> &str {
                &self.0[$prefix.len() + 1..$prefix.len() + 8]
            }
        }

        impl Default for $type {
            fn default() -> Self {
                $type(format!(
                    concat!($prefix, "-{}"),
                    Uuid::new_v4().simple().to_string()
                ))
            }
        }

        impl FromStr for $type {
            type Err = &'static str;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.starts_with(concat!($prefix, "-")) {
                    let uuid = Uuid::try_parse(&s[($prefix.len() + 1)..]);
                    if let Ok(uuid) = uuid {
                        if uuid.get_version() == Some(Version::Random) {
                            return Ok($type(format!(
                                concat!($prefix, "-{}"),
                                uuid.simple().to_string()
                            )));
                        }
                    }
                }

                Err(concat!("Invalid ", stringify!($tt)))
            }
        }

        struct $visitor_type;
        impl<'de> Visitor<'de> for $visitor_type {
            type Value = $type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(concat!("\"", $prefix, "\" followed by UUID-v4"))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v.parse::<$type>() {
                    Ok(id) => Ok(id),
                    Err(_) => Err(E::custom(format!(
                        concat!("Invalid ", stringify!($type), ": {}"),
                        v
                    ))),
                }
            }
        }

        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<$type, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str($visitor_type)
            }
        }
    };
}

derive_properties_for_id!(TodoID, TodoIDVisitor, "todo");
derive_properties_for_id!(TopicID, TopicIDVisitor, "tpic");
derive_properties_for_id!(EventID, EventIDVisitor, "evnt");
derive_properties_for_id!(ActivityID, ActivityIDVisitor, "actv");
