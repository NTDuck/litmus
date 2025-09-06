pub mod models;

pub use self::models::*;

pub mod marker {
    use ::sealed::sealed;

    #[sealed]
    pub trait IsSet {}

    #[sealed]
    pub trait IsUnset {}

    pub struct Set<T>(::core::marker::PhantomData<T>);

    pub struct Unset<T>(::core::marker::PhantomData<T>);

    #[sealed]
    impl<T> IsSet for Set<T> {}

    #[sealed]
    impl<T> IsUnset for Unset<T> {}
}
