use std::ops::Deref;

use crate::utils::aliases::{HashSet, MaybeOwnedStr};

pub(crate) struct Tags(HashSet<Tag>);

type Tag = MaybeOwnedStr;

impl Tags {
    pub(crate) fn with(self, tag: Tag) -> Self {
        let mut tags = self.0;
        tags.insert(tag);

        Self(tags)
    }
}

impl From<Tag> for Tags {
    fn from(tag: Tag) -> Self {
        let mut tags = HashSet::new();
        tags.insert(tag);

        Self(tags)
    }
}

impl Deref for Tags {
    type Target = HashSet<Tag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
