//! # based
//!
//! `based` provides support for custom numeral systems with single-character digits.
//!
//! `based` does not currently support multiple-character digits.

use std::ops::{AddAssign, MulAssign};

#[derive(Debug)]
/// Error type produced when [`Base::from_str`](Base::from_str) encounters an unknown character.
/// Contains the unknown character.
/// 
/// # Examples
/// 
/// ```
/// use based::Base;
///
/// let base16 = Base::new("0123456789abcdef");
/// let sixteen = base16.from_str::<usize>("0n1");
/// assert_eq!(sixteen.err().unwrap().0, 'n');
/// ```
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

/// The representation of a numeral system.
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
  /// Creates a new numeral system from the given string slice.
  /// 
  /// The value of each character is its index in the slice,
  /// e.g. the first character has value 0, the second
  /// value 1, etc.
  /// 
  /// The behavior of this function is undefined when
  /// a character is present more than once in the given
  /// string slice.
  ///
  /// # Examples
  ///
  /// ```
  /// use based::Base;
  ///
  /// let base16 = based::Base::new("0123456789abcdef");
  /// ```
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

  /// Given a base and a number's representation
  /// in that base, return the number.
  ///
  /// Returns `Err` if it encounters a character not in the `Base`.
  ///
  /// # Examples
  /// 
  /// ```
  /// use based::Base;
  ///
  /// let base16 = Base::new("0123456789abcdef");
  /// let sixteen = base16.from_str::<usize>("10");
  /// assert_eq!(sixteen.unwrap(), 16);
  /// ```
  pub fn from_str<T: AddAssign + std::default::Default + MulAssign + std::convert::From<usize>>(&self, rep: &str) -> Result<T, UnknownChar> {
    let mut val: T = Default::default();
    let radix = self.base.len();
    for c in rep.chars() {
      match self.vals.get(&c) {
        None => return Err(UnknownChar(c)),
        Some(v) => {
          val *= T::from(radix);
          val += T::from(*v);
        }
      }
    }
    Ok(val)
  }

  /// Given a base and a number, return the 
  /// representation of the number in the base.
  /// # Examples
  /// 
  /// ```
  /// use based::Base;
  ///
  /// let base16 = Base::new("0123456789abcdef");
  /// let sixteen = base16.rep::<usize>(16);
  /// assert_eq!(sixteen, "10");
  /// ```
  pub fn rep<T: std::convert::Into<usize>>(&self, val: T) -> String {
    let mut stack = Vec::new();
    let radix = self.base.len();
    let into = val.into();
    let mut rem = into % radix;
    stack.push(self.base[rem]);
    let mut div = into / radix;
    while div > 0 {
      rem = div % radix;
      div = div / radix;
      stack.push(self.base[rem]);
    } 
    stack.reverse();
    stack.into_iter().collect()
  }
}