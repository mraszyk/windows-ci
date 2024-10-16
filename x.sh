sudo apt update && sudo apt install -y wget curl
wget https://download.dfinity.systems/ic/4bd8e1d8430ee17c7e3be47732d1786074ecb592/binaries/x86_64-linux/pocket-ic.gz
gzip -d pocket-ic.gz
chmod +x pocket-ic
export RUST_LOG=debug
./pocket-ic --ip-addr 0.0.0.0 --port 8057 --ttl 2592000
