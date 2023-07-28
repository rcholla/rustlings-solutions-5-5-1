fn calculate_price_of_apples(apple_count: usize) -> usize {
  match apple_count {
    x if x <= 40 => apple_count * 2,
    _ => apple_count,
  }
}

#[test]
fn verify_test() {
  let price1 = calculate_price_of_apples(35);
  let price2 = calculate_price_of_apples(40);
  let price3 = calculate_price_of_apples(41);
  let price4 = calculate_price_of_apples(65);

  assert_eq!(70, price1);
  assert_eq!(80, price2);
  assert_eq!(41, price3);
  assert_eq!(65, price4);
}
