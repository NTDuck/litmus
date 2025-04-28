use std::ops::Deref;

use crate::utils::aliases::{HashSet, MaybeOwnedStr};

pub(crate) struct Tags(HashSet<Tag>);

pub type Tag = MaybeOwnedStr;

impl FromIterator<Tag> for Tags {
    fn from_iter<Iter: IntoIterator<Item = Tag>>(iter: Iter) -> Self {
        let tags = HashSet::from_iter(iter);

        Self(tags)
    }
}

impl Deref for Tags {
    type Target = HashSet<Tag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
