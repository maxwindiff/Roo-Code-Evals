/// What should the type of _function be?
pub fn map<T, U>(input: Vec<T>, mut _function: impl FnMut(T) -> U) -> Vec<U> {
    let mut ret = Vec::with_capacity(input.len());
    for elem in input {
        ret.push(_function(elem))
    }
    ret
}
