fn byte_counter<T>(arg: T) -> usize
where
  T: AsRef<str>,
{
  arg.as_ref().as_bytes().len()
}

fn char_counter<T>(arg: T) -> usize
where
  T: AsRef<str>,
{
  arg.as_ref().chars().count()
}

fn num_sq<T>(arg: &mut T)
where
  T: AsMut<u32>,
{
  *arg.as_mut() *= *arg.as_mut();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn different_counts() {
    let s = "Café au lait";
    assert_ne!(char_counter(s), byte_counter(s));
  }

  #[test]
  fn same_counts() {
    let s = "Cafe au lait";
    assert_eq!(char_counter(s), byte_counter(s));
  }

  #[test]
  fn different_counts_using_string() {
    let s = String::from("Café au lait");
    assert_ne!(char_counter(s.clone()), byte_counter(s));
  }

  #[test]
  fn same_counts_using_string() {
    let s = String::from("Cafe au lait");
    assert_eq!(char_counter(s.clone()), byte_counter(s));
  }

  #[test]
  fn mult_box() {
    let mut num: Box<u32> = Box::new(3);
    num_sq(&mut num);
    assert_eq!(*num, 9);
  }
}
