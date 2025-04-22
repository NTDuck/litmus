use std::ops::Deref;

use crate::utils::aliases::{HashSet, MaybeOwnedStr};

pub struct Tags(HashSet<MaybeOwnedStr>);

impl Deref for Tags {
    type Target = HashSet<MaybeOwnedStr>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
