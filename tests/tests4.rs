struct Rectangle {
  width: i32,
  height: i32,
}

impl Rectangle {
  pub fn new(width: i32, height: i32) -> Self {
    if width <= 0 || height <= 0 {
      panic!("Rectangle width and height cannot be negative!")
    }
    Rectangle { width, height }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn correct_width_and_height() {
    let rect = Rectangle::new(10, 20);
    assert_eq!(rect.width, 10);
    assert_eq!(rect.height, 20);
  }

  #[test]
  #[should_panic]
  fn negative_width() {
    let _rect = Rectangle::new(-10, 10);
  }

  #[test]
  #[should_panic]
  fn negative_height() {
    let _rect = Rectangle::new(10, -10);
  }
}
