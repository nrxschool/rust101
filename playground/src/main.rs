fn main() {
    println!("Hello, world!");
}

#[test]
fn broken_test_bit_count() {
    let bits: u8 = 0b0101_0101;
    assert_eq!(bits.count_zeros(), 1);
}
