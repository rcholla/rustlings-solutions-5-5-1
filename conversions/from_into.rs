#[derive(Debug)]
struct Person {
  name: String,
  age: usize,
}

impl Default for Person {
  fn default() -> Person {
    Person {
      name: String::from("John"),
      age: 30,
    }
  }
}

impl From<&str> for Person {
  fn from(s: &str) -> Person {
    if let Some((name, age)) = s.split_once(',') {
      if !name.is_empty() {
        if let Ok(age) = age.parse::<usize>() {
          return Person {
            name: name.into(),
            age,
          };
        }
      }
    };

    Person::default()
  }
}

fn main() {
  let p1 = Person::from("Mark,20");
  let p2: Person = "Gerald,70".into();

  println!("{:?}", p1);
  println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default() {
    let dp = Person::default();
    assert_eq!(dp.name, "John");
    assert_eq!(dp.age, 30);
  }

  #[test]
  fn test_bad_convert() {
    let p = Person::from("");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }

  #[test]
  fn test_good_convert() {
    let p = Person::from("Mark,20");
    assert_eq!(p.name, "Mark");
    assert_eq!(p.age, 20);
  }

  #[test]
  fn test_bad_age() {
    let p = Person::from("Mark,twenty");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }

  #[test]
  fn test_missing_comma_and_age() {
    let p: Person = Person::from("Mark");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }

  #[test]
  fn test_missing_age() {
    let p: Person = Person::from("Mark,");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }

  #[test]
  fn test_missing_name() {
    let p: Person = Person::from(",1");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }

  #[test]
  fn test_missing_name_and_age() {
    let p: Person = Person::from(",");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }

  #[test]
  fn test_missing_name_and_invalid_age() {
    let p: Person = Person::from(",one");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }

  #[test]
  fn test_trailing_comma() {
    let p: Person = Person::from("Mike,32,");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }

  #[test]
  fn test_trailing_comma_and_some_string() {
    let p: Person = Person::from("Mike,32,man");
    assert_eq!(p.name, "John");
    assert_eq!(p.age, 30);
  }
}
