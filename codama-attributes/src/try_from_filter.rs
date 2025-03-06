pub trait TryFromFilter<'a, T>: Sized {
    fn filter(t: T) -> Option<&'a Self>;
}

impl<'a, T: 'a, U> TryFromFilter<'a, U> for T
where
    &'a T: TryFrom<U>,
{
    fn filter(u: U) -> Option<&'a T> {
        <&'a T>::try_from(u).ok()
    }
}
