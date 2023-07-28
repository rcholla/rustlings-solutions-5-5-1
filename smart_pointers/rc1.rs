use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
  Mercury(Rc<Sun>),
  Venus(Rc<Sun>),
  Earth(Rc<Sun>),
  Mars(Rc<Sun>),
  Jupiter(Rc<Sun>),
  Saturn(Rc<Sun>),
  Uranus(Rc<Sun>),
  Neptune(Rc<Sun>),
}

impl Planet {
  fn details(&self) {
    println!("Hi from {:?}!", self)
  }
}

fn main() {
  let sun = Rc::new(Sun {});
  println!("reference count = {}", Rc::strong_count(&sun));

  let mercury = Planet::Mercury(Rc::clone(&sun));
  println!("reference count = {}", Rc::strong_count(&sun));
  mercury.details();

  let venus = Planet::Venus(Rc::clone(&sun));
  println!("reference count = {}", Rc::strong_count(&sun));
  venus.details();

  let earth = Planet::Earth(Rc::clone(&sun));
  println!("reference count = {}", Rc::strong_count(&sun));
  earth.details();

  let mars = Planet::Mars(Rc::clone(&sun));
  println!("reference count = {}", Rc::strong_count(&sun));
  mars.details();

  let jupiter = Planet::Jupiter(Rc::clone(&sun));
  println!("reference count = {}", Rc::strong_count(&sun));
  jupiter.details();

  let saturn = Planet::Saturn(Rc::clone(&sun));
  println!("reference count = {}", Rc::strong_count(&sun));
  saturn.details();

  let uranus = Planet::Uranus(Rc::clone(&sun));
  println!("reference count = {}", Rc::strong_count(&sun));
  uranus.details();

  let neptune = Planet::Neptune(Rc::clone(&sun));
  println!("reference count = {}", Rc::strong_count(&sun));
  neptune.details();

  assert_eq!(Rc::strong_count(&sun), 9);

  drop(neptune);
  println!("reference count = {}", Rc::strong_count(&sun));

  drop(uranus);
  println!("reference count = {}", Rc::strong_count(&sun));

  drop(saturn);
  println!("reference count = {}", Rc::strong_count(&sun));

  drop(jupiter);
  println!("reference count = {}", Rc::strong_count(&sun));

  drop(mars);
  println!("reference count = {}", Rc::strong_count(&sun));

  drop(earth);
  println!("reference count = {}", Rc::strong_count(&sun));

  drop(venus);
  println!("reference count = {}", Rc::strong_count(&sun));

  drop(mercury);
  println!("reference count = {}", Rc::strong_count(&sun));

  assert_eq!(Rc::strong_count(&sun), 1);
}
