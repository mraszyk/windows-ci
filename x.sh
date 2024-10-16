sudo apt update && sudo apt install -y wget curl
wget https://github.com/dfinity/pocketic/releases/download/6.0.0/pocket-ic-x86_64-linux.gz
wget https://download.dfinity.systems/ic/4bd8e1d8430ee17c7e3be47732d1786074ecb592/binaries/x86_64-linux/pocket-ic.gz
gzip -d pocket-ic.gz
chmod +x pocket-ic
export RUST_LOG=debug
sudo ./pocket-ic --port 8057 --ttl 2592000 &> log.txt &
sleep 10
cat log.txt
curl -X GET -H "Content-Type: application/json" http://localhost:8057/status
cat log.txt
