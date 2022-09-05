fn main() {
}

use std::hash::Hash;
use std::collections::HashMap;
// caching struct for closure
struct Cacher<T, U, V>
    where T: Fn(U) -> V,
        U: PartialOrd + Eq + Hash + Copy,
        V: Copy
{
    calculation: T,
    value_map: HashMap<U, V>,
}

impl <T,U,V> Cacher<T, U, V>
    where T: Fn(U) -> V,
          U: PartialOrd + Eq + Hash + Copy,
          V: Copy
{
    fn new(calculation: T) -> Cacher<T,U,V> {
        Cacher {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        let value = self.value_map.entry(arg).or_insert((self.calculation)(arg));
        *value
    }
}


#[test]
fn cache_u32_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);
    let v3 = c.value(1);

    assert_eq!(v2, 2);
    assert_eq!(v1, v3);
}

#[test]
fn cache_str_with_different_values() {
    let mut c = Cacher::new(|string: &str| string.len());

    let val1 = c.value("hello");
    let val2 = c.value("world");

    assert_eq!(val1, 5);
    assert_eq!(val2, 5);
}
