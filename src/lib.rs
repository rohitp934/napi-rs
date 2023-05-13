#![deny(clippy::all)]

use napi::Error;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn divide(a: u32, b: u32) -> Result<u32, Error> {
    a.checked_div(b).ok_or(Error::from_reason(format!("Divide by zero error.")))
}

#[napi]
pub fn greetings(name: String) -> String {
    format!("hola {}!", name)
}
