/*! 
# based

`based` provides support for custom numeral systems with single-character digits.

`based` does not support multi-character digits.

# Examples

```
use based::{Base, NumeralSystem};

let base16 = Base::new("0123456789abcdef");
let val: usize = base16.decode("10").unwrap();
assert_eq!(val, 16);
assert_eq!(base16.encode(16 as usize).unwrap(), "10")
```
*/

use std::num::TryFromIntError;

mod single_base;
pub use single_base::*;

/**
`StrError` is the error type produced when 
[`NumeralSystem::decode`](NumeralSystem::decode) encounters an unknown character
or fails to convert between two integer types.
*/
#[derive(Debug)]
pub enum StrError {
  /// Contains the unknown character.
  UnknownChar(char),
  Try(TryFromIntError)
}

impl std::fmt::Display for StrError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      StrError::UnknownChar(c) => write!(f, "Encountered char {} not in base", c),
      StrError::Try(t) => t.fmt(f)
    }   
  }
}

impl std::error::Error for StrError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      StrError::UnknownChar(_) => None,
      StrError::Try(t) => Some(t)
    }
  }
}

impl From<TryFromIntError> for StrError {
  fn from(err: TryFromIntError) -> StrError {
    StrError::Try(err)
  }
}

/// `NumeralSystem` provides conversions to and from representations in the given system.
pub trait NumeralSystem<T> {

  /**
  Given a `NumeralSystem` and a number's representation
  in that system, return the number.

  Returns `Err` if this function encounters a character not in the system,
  or if an int to int conversion fails.
  */
  fn decode(&self, rep: &str) -> Result<T, StrError>;

  /** 
  Given a `NumeralSystem` and a number, return the 
  representation of that number in the system.

  Returns `Err` if an int to int conversion fails.

  This will interpret signed integers as if their bits represented their
  unsigned counterparts.
  */
  fn encode(&self, val: T) -> Result<String, TryFromIntError>;
}
