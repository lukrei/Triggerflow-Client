Triggerflow-Client<br>

A Triggerbot for CS2 using memflow.<br>
This is a fork of https://github.com/superyu1337/radarflow2. <br>
Clone the repo on your vm:<br>
git clone https://github.com/lukrei/Triggerflow-Client.git <br>
Go to /src/dma/mod.rs at line 239 and change ip 192.168.50.3 to the ip of your Windows VM. let server_address = "192.168.50.3:1337"; Make sure Windows VM is bridged so in the same Network as the Hypervisor.<br>
change directory into Triggerflow-Client <br>
chmod +x *.sh <br>
Then Update Offset:<br>
./update.sh<br>
Compile it again:<br>
./rebuild.sh <br>
Run CS2 inside the VM<br>
Run the program:<br>
./run.sh<br>
Start Wireshark inside your Windows Client VM and put in the displayfilter udp.port==1337. Now go into the game and put the crosshair onto 4 Teammates then on an Enemy. It should display the packets in Wireshark.<br>
Start to write your own client which accepts the udp packets and trigger shoot via shoot offset of cs2.<br>

If you want to manually update your game if cs2-dumper provides old files in the git repository then git clone a2x cs2-dumper and run following command on your hypervisor while cs2 is running inside VM.<br>
sudo ./cs2-dumper -c qemu<br>
After that look at update.sh script and copy the generated files from output directory of cs2-dumper to the directory where the files are copied in update.sh script.<br>

Be aware you need a self coded client to receive the udp packets and this can get you vacced because the client relies on the builtin shoot function in cs2. You can find the offsets for shooting via cheat engine. Use it at your own Risk and only on VAC Insecured Servers.
