use std::fmt;
use std::io::Error;

// type Result<T> = Result<T, Error>;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

fn bar() -> ! {
    loop {}
}

// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value."),
//         }
//     }
// }

fn generic<T>(t: T) {}
fn generic_same_above<T: Sized>(t: T) {}
// TはSizedかもしれないし，違うかもしれない．
// Sizedのみに利用可
fn generic_<T: ?Sized>(t: &T) {}

fn run() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    type Thunk = Box<Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn take_long_type(f: Thunk) {}
    fn returns_long_type() -> Thunk {
        Box::new(|| println!("return"))
    }
}

fn main() {
    run();
}
