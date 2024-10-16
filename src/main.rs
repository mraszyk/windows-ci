fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    use pocket_ic::PocketIcBuilder;
    use std::str::FromStr;
    let server_host = std::fs::read_to_string("host.txt").unwrap();
    let server_port = std::fs::read_to_string("port.txt").unwrap();
    println!("host: {}", server_host);
    println!("port: {}", server_port);
    let pic = PocketIcBuilder::new()
        .with_server_url(
            reqwest::Url::from_str(&format!("http://{}:{}", server_host, server_port)).unwrap(),
        )
        .with_nns_subnet()
        .build();
    let _canister_id = pic.create_canister();
}
