fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    use pocket_ic::PocketIcBuilder;
    use std::str::FromStr;
    let _ = std::panic::catch_unwind(|| {
        let pic = PocketIcBuilder::new()
            .with_server_url(reqwest::Url::from_str("http://localhost:8057").unwrap())
            .with_nns_subnet()
            .build();
    });
}
