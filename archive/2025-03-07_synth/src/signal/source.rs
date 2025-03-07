use std::ops::DerefMut;

pub trait SignalSource<T> {
    fn next_value(&mut self) -> T;
}

impl<S, T> SignalSource<T> for Box<S>
where
    S: SignalSource<T> + ?Sized,
{
    fn next_value(&mut self) -> T {
        self.deref_mut().next_value()
    }
}

pub struct Fn<F>(pub F);

impl<F, T> SignalSource<T> for Fn<F>
where
    F: FnMut() -> T,
{
    fn next_value(&mut self) -> T {
        self.0()
    }
}
