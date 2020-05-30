use crate::{NumeralSystem, StrError};
use std::convert::TryFrom;
use std::num::TryFromIntError;
use std::collections::HashMap;

/** `Base` represents a numeral system with single-character digits.

  When creating a new numeral system from a given string slice,
  the value of each character is its index in the slice,
  e.g. the first character has value `0`, the second has value `1`, etc.

  If the given slice has at least one duplicate character, parsing will
  produce an `Err`.

  # Examples
  
  ```
  use based::*;
  
  let base16: based::Base = "0123456789abcdef".parse().unwrap();
  assert_eq!("a", base16.encode(10).unwrap());
  ```
*/
pub struct Base {
  base: Vec<char>,
  vals: std::collections::HashMap<char, usize>,
}

impl std::fmt::Display for Base {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", self.base.iter().collect::<String>())
  }
}

/**
`DuplicateCharacterError` is the error type produced when trying to create
a [`Base`](Base) from a string with at least one duplicate character.
*/
#[derive(Debug)]
pub struct DuplicateCharacterError {
  dup: char,
  first: usize,
  second: usize,
}


impl std::fmt::Display for DuplicateCharacterError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Encountered duplicate character '{}' at {} and {}", self.dup, self.first, self.second)
  }
}

impl std::error::Error for DuplicateCharacterError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}


impl std::str::FromStr for Base {
  type Err = DuplicateCharacterError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut base: Vec<char> = s.chars().collect();

    let mut vals: std::collections::HashMap<char, usize> = HashMap::new();

    for (i, c) in base.iter().enumerate() {
      if let Some(first) = vals.insert(*c, i) {
        return Err(
          DuplicateCharacterError{
            dup: *c,
            first,
            second: i,
          }
        )
      }
    }
    
    vals.shrink_to_fit();
    base.shrink_to_fit();

    Ok(Base { base, vals })
  }
}

impl NumeralSystem<usize> for Base {
  fn decode(&self, rep: &str) -> Result<usize, StrError> {
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
  fn encode(&self, val: usize) -> Result<String, TryFromIntError> {
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

macro_rules! decode {
  ($type:ty) => {
    fn decode(&self, rep: &str) -> Result<$type, StrError> {
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
      decode!{$type}
    
      /// Never produces `Err`.
      fn encode(&self, val: $type) -> Result<String, std::num::TryFromIntError> {
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
      decode!{$type}
    
      fn encode(&self, val: $type) -> Result<String, std::num::TryFromIntError> {
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
      decode!{$itype}
    
      fn encode(&self, val: $itype) -> Result<String, std::num::TryFromIntError> {
        self.encode(val as $utype)
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