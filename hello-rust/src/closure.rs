use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct CacherSimple<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> CacherSimple<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> CacherSimple<T> {
        CacherSimple {
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

struct Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: Eq + Hash + Clone,
    V: Clone,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: Eq + Hash + Clone,
    V: Clone,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        if let Some(v) = self.values.get(&arg).map(V::clone) {
            v
        } else {
            let v = (self.calculation)(&arg);
            self.values.insert(arg, v.clone());
            v
        }
    }
}

#[test]
fn simple_same_value() {
    let mut c = CacherSimple::new(|a| a);
    let v1 = c.value(1);

    assert_eq!(v1, 1);
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|&a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn call_with_different_types() {
    let mut c = Cacher::new(|&a| a);

    let v1 = c.value("this");
    let v2 = c.value("that");

    assert_eq!(v2, "that")
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = CacherSimple::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    // let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 2, 3];
    // capture the environment.
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn main() {
    run();
}
