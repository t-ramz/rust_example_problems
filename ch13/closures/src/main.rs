fn main() {
}

use std::hash::Hash;
use std::collections::HashMap;
// caching struct for closure
struct Cacher<T, U>
    where T: Fn(U) -> U, // assume return self
        U: PartialOrd + Eq + Hash + Copy
{
    calculation: T,
    value_map: HashMap<U, U>,
}

impl <T,U> Cacher<T,U>
    where T: Fn(U) -> U,
          U: PartialOrd + Eq + Hash + Copy
{
    fn new(calculation: T) -> Cacher<T,U> {
        Cacher {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        let value = self.value_map.entry(arg).or_insert((self.calculation)(arg));
        *value
    }
}

// Limitations of Cacher Implementation

// cacher will only store first value calculated. will not be sensitive to args
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
    let mut c = Cacher::new(|string| string);

    let val1 = c.value("hello");
    let val2 = c.value("world");

    assert_eq!(val1, "hello");
    assert_eq!(val2, "world");
}
