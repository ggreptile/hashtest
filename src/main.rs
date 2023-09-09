use blake3::Hasher;

fn main() {
    let mut hasher_first = Hasher::new();
    let mut hasher_second = Hasher::new();
    
    hasher_first.update(b"12");
    hasher_first.update(b"3");

    hasher_second.update(b"1");
    hasher_second.update(b"23");

    let res_first = hasher_first.finalize();
    let res_second = hasher_second.finalize();

    assert_eq!(res_first, res_second);
    assert_eq!(res_first, blake3::hash(b"123"));
}
