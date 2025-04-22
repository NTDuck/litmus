use crate::utils::aliases::MaybeOwnedStr;

use super::Tags;

pub struct ConfiguredFeature {
    description: Option<MaybeOwnedStr>,
    ignored: Option<bool>,
    tags: Option<Tags>,

        
}