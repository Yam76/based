/*! 
# based

`based` provides support for custom numeral systems with single-character digits.

`based` does not currently support multi-character digits.
*/

// use std::ops::{AddAssign, MulAssign};

#[derive(Debug)]
/**
`UnknownChar` is the error type produced when 
[`Base::from_str`](Base::from_str) encounters an unknown character.
It contains the unknown character.
 
# Examples
 
```
use based::{Base, NumeralSystem};

let base16 = Base::new("0123456789abcdef");
let sixteen = base16.from_str("0n1");
assert_eq!(sixteen.err().unwrap().0, 'n');
```
*/
pub struct UnknownChar(pub char);

impl std::fmt::Display for UnknownChar {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Encountered char {} not in base", self.0)
  }
}

impl std::error::Error for UnknownChar {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
      None
  }
}

/// `Base` represents a single-character per digit numeral system.
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

pub trait NumeralSystem<T> {

  /**
  Given a `NumeralSystem` and a number's representation
  in that system, return the number.

  Returns `Err` if this function encounters a character not in the system.
  */
  fn from_str(&self, rep: &str) -> Result<T, UnknownChar>;
  /** 
  Given a `NumeralSystem` and a number, return the 
  representation of that number in the system.
   */
  fn digits(&self, val: T) -> String;
}

macro_rules! from_str_lossy {
  ($type:ty) => {
    impl NumeralSystem<$type> for Base {
      /// Potentially lossy.
      fn from_str(&self, rep: &str) -> Result<$type, UnknownChar> {
        let mut val = 0;
        let radix = self.base.len();
        for c in rep.chars() {
          match self.vals.get(&c) {
            None => return Err(UnknownChar(c)),
            Some(v) => {
              val *= radix;
              val += *v;
            }
          }
        }
        Ok(val as $type)
      }
    
      fn digits(&self, val: $type) -> String {
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
        stack.into_iter().collect()
      }
    }
  };
}

from_str_lossy!{u8}
from_str_lossy!{u16}

/*
impl NumeralSystem<u8> for Base {
  /// Potentially lossy.
  fn from_str(&self, rep: &str) -> Result<u8, UnknownChar> {
    let mut val = 0;
    let radix = self.base.len();
    for c in rep.chars() {
      match self.vals.get(&c) {
        None => return Err(UnknownChar(c)),
        Some(v) => {
          val *= radix;
          val += *v;
        }
      }
    }
    Ok(val as u8)
  }

  fn digits(&self, val: u8) -> String {
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
    stack.into_iter().collect()
  }
}
*/

impl NumeralSystem<usize> for Base {
  /*
  # Examples
   
  ```
  use based::Base;
  
  let base16 = Base::new("0123456789abcdef");
  let sixteen = base16.from_str::<usize>("10");
  assert_eq!(sixteen.unwrap(), 16);
  ```
  */
  fn from_str(&self, rep: &str) -> Result<usize, UnknownChar> {
    let mut val = 0;
    let radix = self.base.len();
    for c in rep.chars() {
      match self.vals.get(&c) {
        None => return Err(UnknownChar(c)),
        Some(v) => {
          val *= radix;
          val += *v;
        }
      }
    }
    Ok(val)
  }

  /* 
  # Examples
   
  ```
  use based::Base;
  
  let base16 = Base::new("0123456789abcdef");
  let sixteen = base16.digits::<usize>(16);
  assert_eq!(sixteen, "10");
  ```
  */
  fn digits(&self, val: usize) -> String {
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
    stack.into_iter().collect()
  }
}