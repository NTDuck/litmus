use std::ops::Deref;

pub enum Steps<Callback, Wrapped: WrappedCallback<Callback>> {
    One(Callback),
    Many(Vec<Wrapped>),
}

pub trait WrappedCallback<Callback>: Deref<Target = Callback> {
    
}