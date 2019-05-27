#[test]
fn hashmap_test() {
    let mut m = super::crete_hashmap();
    assert!(m.capacity() >= super::N);

    let m = super::write(&mut m);

    assert_eq!(m.len(), super::N);

    let value = m.get("0-string").unwrap();
    assert_eq!(value, "0-string");
}
