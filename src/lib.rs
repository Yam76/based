/*! 
# based

`based` provides support for custom numeral systems with single-character digits.

`based` does not support multi-character digits.

# Examples

```
use based::{Base, NumeralSystem};

let base16 = Base::new("0123456789abcdef");
let val: usize = base16.from_str("10").unwrap();
assert_eq!(val, 16);
assert_eq!(base16.digits(16 as usize).unwrap(), "10")
```
*/

use std::convert::TryFrom;
use std::num::TryFromIntError;

/**
`StrError` is the error type produced when 
[`Base::from_str`](Base::from_str) encounters an unknown character
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


/// `Base` represents a numeral system with single-character digits.
pub struct Base {
  base: Vec<char>,
  vals: std::collections::HashMap<char, usize>,
}

impl std::fmt::Display for Base {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", self.base.iter().collect::<String>())
  }
}

impl Base {
  /**
  Creates a new numeral system from the given string slice.
  
  The value of each character is its index in the slice,
  e.g. the first character has value `0`, the second value `1`, etc.
   
  The behavior of this function is undefined when
  a character is present more than once in the given string slice.
  
  # Examples
  
  ```
  use based::Base;
  
  let base16 = based::Base::new("0123456789abcdef");
  ```
  */
  pub fn new(base: &str) -> Base {
    let mut vals: std::collections::HashMap<char, usize> = base
    .chars()
    .enumerate()
    .map(|(i, c)| (c, i))
    .collect();
    vals.shrink_to_fit();

    let mut base: Vec<char> = base.chars().collect();
    base.shrink_to_fit();

    Base {
      base,
      vals, 
    }
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
  fn from_str(&self, rep: &str) -> Result<T, StrError>;

  /** 
  Given a `NumeralSystem` and a number, return the 
  representation of that number in the system.

  Returns `Err` if an int to int conversion fails.

  This will interpret signed integers as if their bits represented their
  unsigned counterparts.
  */
  fn digits(&self, val: T) -> Result<String, TryFromIntError>;
}

impl NumeralSystem<usize> for Base {
  fn from_str(&self, rep: &str) -> Result<usize, StrError> {
    let mut val = 0;
    let radix = self.base.len();
    for c in rep.chars() {
      match self.vals.get(&c) {
        None => return Err(StrError::UnknownChar(c)),
        Some(v) => {
          val *= radix;
          val += *v;
        }
      }
    }
    Ok(val)
  }

  /// Never produces `Err`.
  fn digits(&self, val: usize) -> Result<String, TryFromIntError> {
    let mut stack = Vec::new();
    let radix = self.base.len();
    let mut rem = val % radix;
    stack.push(self.base[rem]);
    let mut div = val / radix;
    while div > 0 {
      rem = div % radix;
      div = div / radix;
      stack.push(self.base[rem]);
    } 
    stack.reverse();
    Ok(stack.into_iter().collect())
  }
}

macro_rules! from_str {
  ($type:ty) => {
    fn from_str(&self, rep: &str) -> Result<$type, StrError> {
      let mut val = 0;
      let radix = self.base.len();
      for c in rep.chars() {
        match self.vals.get(&c) {
          None => return Err(StrError::UnknownChar(c)),
          Some(v) => {
            val *= radix;
            val += *v;
          }
        }
      }
      Ok(<$type>::try_from(val)?)
    }
  };
}

macro_rules! small_uint {
  ($type:ty) => {
    impl NumeralSystem<$type> for Base {
      from_str!{$type}
    
      /// Never produces `Err`.
      fn digits(&self, val: $type) -> Result<String, std::num::TryFromIntError> {
        let val: usize = usize::from(val);
        let mut stack = Vec::new();
        let radix = self.base.len();
        let mut rem = val % radix;
        stack.push(self.base[rem]);
        let mut div = val / radix;
        while div > 0 {
          rem = div % radix;
          div = div / radix;
          stack.push(self.base[rem]);
        } 
        stack.reverse();
        Ok(stack.into_iter().collect())
      }
    }
  };
}

small_uint!{u8}
small_uint!{u16}

macro_rules! large_uint {
  ($type:ty) => {
    impl NumeralSystem<$type> for Base {
      from_str!{$type}
    
      fn digits(&self, val: $type) -> Result<String, std::num::TryFromIntError> {
        if std::mem::size_of::<$type>() <= std::mem::size_of::<usize>() {
          let val: usize = usize::try_from(val)?;
          let mut stack = Vec::new();
          let radix = self.base.len();
          let mut rem = val % radix;
          stack.push(self.base[rem]);
          let mut div = val / radix;
          while div > 0 {
            rem = div % radix;
            div = div / radix;
            stack.push(self.base[rem]);
          } 
          stack.reverse();
          Ok(stack.into_iter().collect())
        }
        else {
          let mut stack = Vec::new();
          let radix = <$type>::try_from(self.base.len())?;
          let mut rem = val % radix;
          stack.push(self.base[usize::try_from(rem)?]);
          let mut div = val / radix;
          while div > 0 {
            rem = div % radix;
            div = div / radix;
            stack.push(self.base[usize::try_from(rem)?]);
          } 
          stack.reverse();
          Ok(stack.into_iter().collect())
        }
      }
    }
  };
}

large_uint!{u32}
large_uint!{u64}
large_uint!{u128}

macro_rules! iint {
  ($itype:ty, $utype:ty) => {
    impl NumeralSystem<$itype> for Base {
      from_str!{$itype}
    
      fn digits(&self, val: $itype) -> Result<String, std::num::TryFromIntError> {
        self.digits(val as $utype)
      }
    }
  };
}

iint!{i8, u8}
iint!{i16, u16}
iint!{i32, u32}
iint!{isize, usize}
iint!{i64, u64}
iint!{i128, u128}