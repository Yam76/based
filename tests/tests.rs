use based;

const BASE57: &'static str = "23456789abcdefghijkmnpqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ";

#[test]
fn check_multidigit_rep() {
  let base = based::Base::new(BASE57);
  assert_eq!(base.rep::<usize>(60), "35");
}

#[test]
fn check_multidigit_from_str() {
  let base = based::Base::new(BASE57);
  let result = base.from_str::<usize>("35");
  assert!(result.is_ok());
  assert_eq!(result.unwrap(), 60);
}

#[test]
fn check_failing_from_str() {
  let base = based::Base::new(BASE57);
  assert!(base.from_str::<usize>("35]").is_err());
}


/*
const BASE62: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE59: &'static str = "0123456789abcdefghijkmnpqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ";
*/