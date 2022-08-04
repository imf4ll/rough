#![allow(unused_imports, dead_code)]

pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
