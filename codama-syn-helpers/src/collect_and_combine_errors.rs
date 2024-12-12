pub fn collect_and_combine_errors<T>(
    acc: Result<Vec<T>, syn::Error>,
    result: Result<T, syn::Error>,
) -> Result<Vec<T>, syn::Error> {
    match (acc, result) {
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
    }
}
