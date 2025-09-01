pub trait IntoTagsFilter<Tags>
where
    Tags: 'static,
{
    fn into_filter(self) -> impl for<'a> Fn(&'a Tags) -> bool + Send + Sync;

    fn chain(self, other: impl IntoTagsFilter<Tags>) -> impl for<'a> Fn(&'a Tags) -> bool + Send + Sync
    where
        Self: Sized,
    {
        let this = self.into_filter();
        let other = other.into_filter();

        move |tags| this(tags) && other(tags)
    }
}

fn main() {}