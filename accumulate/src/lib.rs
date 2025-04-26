pub fn map<T, U, F>(input: Vec<T>, function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    input.into_iter().map(function).collect()
}
