use std::fmt::{Debug, Display, Formatter};
use std::io::Error;
use thiserror::Error;

use anyhow::*;

fn main() {
    // let s = "s".to_string();
    // let mut p = Person::new(&s, 1);
    // {
    //     let name = "xyz".to_string();
    //     p = Person::new(&name, 123);
    // }
    // println!("{:?}", p.is_adult());
    // dump(p);
}

fn dump<T: Named+Debug>(o: T) {
    println!("{:?} is called {}", o, o.name());
}

trait Named {
    fn name(&self) -> &str;
}

#[derive(Debug)]
pub struct Person<'a> {
    name: &'a String,
    age: u8,
}
impl <'b> Person<'b> {
    pub fn new(name: &'b String, age: u8) -> Person<'b> {
        Person {
            name: name,
            age,
        }
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

impl Named for Person<'_> {
    fn name(&self) -> &str {
        &self.name
    }
}



#[derive(Error, Debug)]
enum MyError {
    #[error("error A")]
    A,
    #[error("error B")]
    B
}



fn with_both() -> Result<(), anyhow::Error> {
    with_my_error().with_context("this is the first line in with_both")?;
    with_io_error()?;
    Ok(())
}

fn with_io_error() -> Result<(), std::io::Error> {
    todo!()
}
fn with_my_error() -> Result<(), MyError> {
    todo!();
}

