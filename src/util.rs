#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

/// This function exists to modify a variable in a `HashMap` in which it exists.
///
/// If the variable doesn't exist, it will return the specified error.
///
/// It has quite a lot of Trait constraints, but it's the best I could do.
///
/// Note : I could modify the function to use the `hashMap.get_mut()` function, but I don't know if it's worth it.
/// Might be a neat excuse to learn how to benchmark code and see if one is faster than the other in the future.
pub fn modifyVariable<E, K: Hash + Eq + PartialEq, V: Copy, F: FnOnce(V) -> V>(
    hashMap: &mut HashMap<K, V>,
    key: K,
    func: F,
    err: E,
) -> std::result::Result<(), E> {
    match hashMap.get(&key) {
        Some(o) => {
            hashMap.insert(key, func(*o));
            Ok(())
        }
        None => Err(err),
    }
}
