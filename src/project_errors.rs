use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct OutOfIndexError;
impl error::Error for OutOfIndexError {}

impl fmt::Display for OutOfIndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index out of the range.")
    }
}


// 
#[derive(Debug, Clone)]
pub struct NotValidIndexError;
impl error::Error for NotValidIndexError {}

impl fmt::Display for NotValidIndexError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Not a valid index.")
  }
}

#[derive(Debug, Clone)]
pub struct EmptyList;

impl error::Error for EmptyList {}

impl fmt::Display for EmptyList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Empty list.")
  }
}

#[derive(Debug, Clone)]
pub struct TestCustomError;

impl fmt::Display for TestCustomError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "something")
  }
}

impl error::Error for TestCustomError {}