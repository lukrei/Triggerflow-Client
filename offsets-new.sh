cd "./src/dma/cs2dumper/"
rm client.rs
rm offsets.rs
rm engine2.rs
wget https://raw.githubusercontent.com/a2x/cs2-dumper/main/generated/client.dll.rs
wget https://raw.githubusercontent.com/a2x/cs2-dumper/main/generated/offsets.rs
wget https://raw.githubusercontent.com/a2x/cs2-dumper/main/generated/engine2.dll.rs
mv client.dll.rs client.rs
mv engine2.dll.rs engine2.rs
