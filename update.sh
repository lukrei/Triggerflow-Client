cd "./src/dma/cs2dumper/"
rm client_mod.rs
rm offsets_mod.rs
rm engine2_mod.rs
wget https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/client.dll.rs
wget https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/offsets.rs
wget https://raw.githubusercontent.com/a2x/cs2-dumper/main/output/engine2.dll.rs
mv client.dll.rs client_mod.rs
mv engine2.dll.rs engine2_mod.rs
mv offsets.rs offsets_mod.rs
