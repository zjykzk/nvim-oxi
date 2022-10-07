use nvim_types::{
    conversion::{self, FromObject},
    serde::Deserializer,
    Object,
};
use serde::Deserialize;

use super::StatuslineHighlightInfos;

/// Statusline informations returned by
/// [`api::eval_statusline`](crate::api::eval_statusline).
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
pub struct StatuslineInfos {
    /// Vector of highlight informations for the statusline populated if the
    /// [`highlights`](crate::api::opts::EvalStatuslineOptsBuilder::highlights)
    /// field of  was set to `true`.
    #[serde(default)]
    pub highlights: Vec<StatuslineHighlightInfos>,

    /// Characters displayed in the statusline.
    pub str: String,

    /// Display width of the statusline.
    pub width: u32,
}

impl FromObject for StatuslineInfos {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}
