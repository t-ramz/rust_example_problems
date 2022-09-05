use std::thread;
use std::time::Duration;

fn main() {
}



// caching struct for closure
struct Cacher<T>
    where T: Fn(u32) -> u32     // to create a struct for a closure, we must know its type
                                // each closure has its own anonymous type
                                // the Fn trait tells us that what is passed must be a function
{
    calculation: T,
    value: Option<u32>,
}

impl <T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// Limitations of Cacher Implementation

// cacher will only store first value calculated. will not be sensitive to args
#[test]
fn cache_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
