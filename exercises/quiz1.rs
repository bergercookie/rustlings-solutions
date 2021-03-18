// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// Put your function here!
// fn ..... {

// Don't modify this function!

fn calculate_apple_price(num_apples: u32) -> u32 {
    if num_apples < 0 {
        panic!("Invalid value for number of apples!")
    } else if num_apples <= 40 {
        num_apples * 2
    } else {
        num_apples * 1
    }
}

#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
}
