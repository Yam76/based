const BASE57: &'static str = "23456789abcdefghijkmnpqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ";

type Result<T> = std::result::Result<T, based::StrError>;

macro_rules! test {
  ($type:ty) => {
    use based::*;

    #[test]
    fn check () {
      let base = based::Base::new(super::BASE57);
      let val: $type = 60;

      assert_eq!(base.encode(val).unwrap(), "35");

      let target: $type = base.decode("35").unwrap();
      assert_eq!(target, val);

      let failure: super::Result<$type> = base.decode("35[");
      assert!(failure.is_err());
    }
  }
}

mod test_usize {test!{usize}}
mod test_u8 {test!{u8}}
mod test_u16 {test!{u16}}
mod test_u32 {test!{u32}}
mod test_u64{test!{u64}}
mod test_u128 {test!{u128}}

mod test_isize {test!{isize}}
mod test_i8 {test!{i8}}
mod test_i16 {test!{i16}}
mod test_i32 {test!{u32}}
mod test_i64{test!{i64}}
mod test_i128 {test!{i128}}

mod test_overflow {
  use based::*;

  #[test]
  fn check () {
    let base = based::Base::new(super::BASE57);
    let val: i8 = -1;

    assert_eq!(base.encode(val).unwrap(), "6v");

    let failure: super::Result<i8> = base.decode("6v");
    assert!(failure.is_err());
  }

}


/*
const BASE62: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE59: &'static str = "0123456789abcdefghijkmnpqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ";
*/