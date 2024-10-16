fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    use pocket_ic::PocketIcBuilder;
    use std::str::FromStr;
    let server_url = std::env::var("POCKET_IC_URL").unwrap();
    let pic = PocketIcBuilder::new()
        .with_server_url(reqwest::Url::from_str(&server_url).unwrap())
        .with_nns_subnet()
        .build();
    let _canister_id = pic.create_canister();
}
