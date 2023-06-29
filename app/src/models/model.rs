use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue<'a> {
    pub key: Cow<'a, str>,
    pub value: Cow<'a, str>,
}
