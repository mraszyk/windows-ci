fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    use pocket_ic::PocketIcBuilder;
    use std::str::FromStr;
    let pic = PocketIcBuilder::new()
        .with_server_url(reqwest::Url::from_str("http://localhost:8000").unwrap())
        .with_nns_subnet()
        .build();
    let _canister_id = pic.create_canister();
}
