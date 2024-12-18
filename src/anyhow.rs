use anyhow::{anyhow, bail, Context, Result};

// anyhow::Result<T> same as Result<T, anyhow::Error>
fn some() -> Result<()> {
    let path = "filename.txt";

    let content = std::fs::read("filename.txt").context("Failed to detach the important thing")?;
    let content =
        std::fs::read(path).with_context(|| format!("Failed to read instrs from {}", path))?;
    // bail!("Missing attribute: {}", path); // early return
    Ok(())
}

// Robast:
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum DataStoreError {
//     #[error("data store disconnected")]
//     Disconnect(#[from] std::io::Error), // source of the error only
//     #[error("the data for key `{0}` is not available")]
//     Redaction(String),
//     #[error("invalid header (expected {expected:?}, found {found:?})")]
//     InvalidHeader { expected: String, found: String },
//     #[error("invalid index {idx}, expected at least {} and at most {}", .limits.lo, .limits.hi)]
//     OutOfBounds { idx: usize, limits: Limits },
//     #[error("unknown data store error")]
//     Unknown,
// }

use crate::this_error::DataStoreError;

fn failing_function() -> Result<String> {
    Err(anyhow!(DataStoreError::Redaction(
        "wronge inferance".to_string()
    )))
}

fn main() {
    let some_text = "this is some".to_string();
    match failing_function() {
        Ok(s) => println!("{s}"),
        Err(e) => match e.downcast_ref() {
            Some(DataStoreError::Disconnect(e)) => (),
            Some(DataStoreError::Redaction(some_text)) => (),
            Some(DataStoreError::Unknown) => (),
            None => (),
        },
    }
}
// ###############################
// ###############################
// Map for transforming operation.
fn main_2() {
    fn string_to_plus_one(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>().map(|num| num + 1) // Result::map converts the Ok value if it exists or if None returns None
    }

    // Result::ok is useful for converting Results to Options
    // ok to option some // Result<T, E> to Option<T>.
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));

    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.ok(), None);

    // Option::ok_or_else is useful for Options to Results
    // Some(v) to Ok(v) & None to Err(err())
    let x = Some("foo");
    assert_eq!(x.ok_or_else(|| "didn't convertend to ok"), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(
        x.ok_or_else(|| "didn't convertend to ok"),
        Err("didn't convertend to ok")
    );

    // expect: Some value to consuming the self value. // same thing with the unwrap() panics without panic message
    // if None, panics with the message
    let x = Some("value");
    assert_eq!(x.expect("fruits are healthy"), "value");

    // unwrap_or: if None this default otherways some value
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    // unwrap_or_else: if None this default with closer otherways some value
    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);

    // unwrap_or_default: if None return 0 otherways some value
    let x: Option<u32> = None;
    assert_eq!(x.unwrap_or_default(), 0);
    assert_eq!(Some(13).unwrap_or_default(), 12);

    // map_or: if None this default otherways this closer with the value [[map it to the acutal value or this]]
    // both default & return type has to be the same.
    let x: Option<&str> = None;
    assert_eq!(x.map_or(42, |v| v.len()), 42);

    // *****map_or_else: if any closer (with the value), if none closer
    // both closer return type has to be the same.
    let k = 21;
    let x = Some("foo");
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);

    // ok_or: convert option to Result and if None, with this default error
    // ok_or_else: with clouser for both.
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0)); // error value

    ////////////////////////////// for result
    // is_err_and: error and something is true then true
    // is_ok_and: ok value and something is true then true.
    use std::io::{Error, ErrorKind};

    let x: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "!"));
    assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), true);

    let x: Result<u32, Error> = Err(Error::new(ErrorKind::PermissionDenied, "!"));
    assert_eq!(x.is_err_and(|x| x.kind() == ErrorKind::NotFound), false);

    // map_or: if Err this default otherways this closer with the value [[map it to the acutal value or this]]
    let x: Result<_, &str> = Ok("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);

    // map_or_else: if any closer, if none closer. (with the error value or ok value access)
    // both closer return type has to be the same.
    // unpack a successful result while handling an error.
    let k = 21;

    let x: Result<_, &str> = Ok("foo");
    assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 3);

    let x: Result<&str, _> = Err("bar");
    assert_eq!(x.map_or_else(|e| k * 2, |v| v.len()), 42);

    // ********map_err: map to an error only
    // pass through a successful result while handling an error.
    let x: Result<u32, u32> = Err(13);
    assert_eq!(
        x.map_err(|x| format!("error code: {x}")),
        Err("error code: 13".to_string())
    );
    // Or
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));
    fn stringify(x: u32) -> String {
        format!("error code: {x}")
    }
    // let x: Result<u32, u32> = Ok(2);
    // assert_eq!(x.map_err(stringify), Ok(2));
}
