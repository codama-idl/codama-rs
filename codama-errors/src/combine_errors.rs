use crate::CodamaError;

pub trait CombineErrors {
    fn combine(&mut self, other: Self);
}

impl CombineErrors for CodamaError {
    fn combine(&mut self, other: Self) {
        if let (CodamaError::Compilation(this), CodamaError::Compilation(that)) = (self, other) {
            this.combine(that)
        }
    }
}

impl CombineErrors for syn::Error {
    fn combine(&mut self, other: Self) {
        syn::Error::combine(self, other)
    }
}

pub trait IteratorCombineErrors<T, E>: Iterator<Item = Result<T, E>>
where
    E: std::error::Error + CombineErrors,
{
    fn collect_and_combine_errors(self) -> Result<Vec<T>, E>
    where
        Self: Sized,
    {
        self.fold(Ok(Vec::new()), |acc, result| match (acc, result) {
            (Ok(mut acc_vec), Ok(parsed)) => {
                acc_vec.push(parsed);
                Ok(acc_vec)
            }
            (Err(mut acc_err), Err(err)) => {
                acc_err.combine(err);
                Err(acc_err)
            }
            (Err(acc_err), _) => Err(acc_err),
            (_, Err(err)) => Err(err),
        })
    }
}

impl<I, T, E> IteratorCombineErrors<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
    E: std::error::Error + CombineErrors,
{
}
