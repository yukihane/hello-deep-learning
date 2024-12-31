use std::any::type_name;

/// dtype の代わり。型を返す。
pub fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}
