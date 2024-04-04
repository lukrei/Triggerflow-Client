Triggerflow-Client

A Triggerbot for CS2 using memflow This is a fork of https://github.com/superyu1337/radarflow2. Please know that this a clone where some parts of the project radarflow2 are still in place which needs to be cleared at later stage of development. For Example parts of the code are still in the projects which tries to start a web socket but the webradar behind wont work anymore in this repo. If you want the web radar pleasse use the original repo from superyu1337.

The Triggerflow-Client is a Triggerbot Application for Counter-Strike 2 which adds the first 4 Identities you select with your Crosshair to a "Whitelist" which are normally your Teammates ingame. It does this by using a Hash function where the first 4 unique Crosshair IDs are added to an array. After the array contains 4 unique values (your m8s) it will shoot automatically at other identities such as enemies or winebottles, window glass etc. This could propably optimized by finding out the identity ids of your current team where you play.

The Triggerflow-Client is using a Offset Dumper to update all the offsets needed for triggerbot. It does this by using offsets-new.sh file. You should run this when a game update is released it fetches automtically the required files and copies it into the repository. Run this file if a game update has happend.

Also be aware that this is only the client part which using memflow from ko1n and h33p. You need to make your own Server which runs on your gaming Windows VM. I have not included this here to prevent uncareful users to get banned of Valve Anti Cheat.

Use this triggerbot only for showcase of how a triggerbot works and use -insecure parameter in counter-strike 2 start parameters as the client part can you get banned by anti cheat.
How can I run this?

There is two ways to run this, the first way is using a KVM/QEMU setup to target a running VM to read memory out of it. The second way is using pcileech hardware, like a PCIe Screamer.
The KVM/QEMU method

First, you need to set up a virtual machine on linux using qemu.
How to set up a VM on linux is way out of scope for this. You can find plenty of information online on how to do it.

Before you begin, install the necessary memflow plugins using memflowup from the stable 0.2.0 channel!
The needed Plugins are memflow-qemu and memflow-win32

Clone the repo on your vm host:
git clone https://github.com/lukrei/Triggerflow-Client.git
./update.sh
./rebuild.sh
./run.sh


Make your own triggerflow-server on your windows client os which receives the udp packet and then does some action with the game like executing shoot command in game. You can do this via the shoot Base Pointer. You can look Cheat Engine tutorials how to find this basepointer for shooting. I have not included this in this Repository since this is the sensitive part which can get you banned. The Server part of the Triggerflow-Client is completely undetected by VAC but it does not help you much in this scenario since you need some sort of windows application which shoots for you.

For an overview of CLI commands, run this:
cargo run --release -- --help
The pcileech method

Install your pcileech hardware in your target pc. On your attacking PC, install the necessary memflow plugins using memflowup from the stable 0.2.0 channel!
The needed Plugins are memflow-pcileech and memflow-win32.

Furthermore, you need to install some libraries, depending on your attacking PC's OS.

On Windows you additionally need to supply the proprietary FTD3XX.dll.
It can be downloaded from the FTDI Website in the Application Library (DLL) column.

On Linux you need to check-out and compile the leechcore_ft601_driver_linux projectfrom the LeechCore-Plugins repository.
On Linux the leechcore_ft601_driver_linux.so filecurrently has to be placed in /usr/ or /usr/lib.
Alternatively LD_LIBRARY_PATH can be set to the containing path.
Check the dlopen documentation for all possible import paths.

Clone the repo on your attacking pc:
git clone https://github.com/lukrei/Triggerflow-Client.git

Run Triggerflow-Client:
cargo run --release -- --connector pcileech

For an overview of CLI commands, run this:
cargo run --release -- --help
Detection Status
VAC: ❓ (Unknown, could be detected because Triggerflow-Client requires some sort of Client on the Guest OS which could be detected by VAC)
FaceIt: ❓ (Unknown, could be detected because Triggerflow-Client requires some sort of Client on the Guest OS which could be detected by VAC)
ESEA: ❓ (Unknown, could be detected because Triggerflow-Client requires some sort of Client on the Guest OS which could be detected by VAC)
