<a name="readme-top"></a>

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Issues][issues-shield]][issues-url]

<br />
<div align="center">

  <h3 align="center">apexsky</h3>

  <p align="center">
    Apex Legends hack
    <br />
    <br />
    <br />
    <a href="https://www.unknowncheats.me/forum/apex-legends/406426-kvm-vmread-apex-esp-aimbot.html">UnknownCheats thread</a>
    ·
    <a href="https://github.com/chettoy/apexsky/issues">Report Bug</a>
    ·
    <a href="https://github.com/chettoy/apexsky/issues">Request Feature</a>
  </p>
</div>

<br />

<img src="Pictures/MainGameView.jpg" alt="Main View" width="960" height="540">

<br />

## About The Project


Apex Legends QEMU/KVM/DMA/Linux hack

UnknownCheats thread: <https://www.unknowncheats.me/forum/apex-legends/406426-kvm-vmread-apex-esp-aimbot.html>

Bone IDs reference: <https://www.unknowncheats.me/wiki/Apex_Legends_Bones_and_Hitboxes>

Game version: v3.0.80.27

 **Features**

* Inherits all features from apex_dma_kvm_pub.
* **Aimbot** with Improved target locking.
* Sky Grenade feature
* Advanced targeting options with various bone selection effects.
* **Aim Assist:** Simulates powerful aim assist-like magnetism through Aimbot.
* **Triggerbot:** Triggerbot function with magnetism and hit prediction.
* **Bone/Skeleton ESP:** Optional glow box and Bone/Skeleton ESP and health bar to replace player glow (ideal for screenshot prevention or live broadcasting). (overlay required)
* **Aiming Target Indicator:** White small circle indicating current pre-aim target. (overlay required)
* **Favorite Gamer Highlight:** Highlights favorite gamers for quick identification.
* **FPS Display**: Calculate and display the game's frame rate. (overlay required)
* **Spectator List:** Displays spectator list. (overlay required)
* ~~**Weapon Model Glow:** Indicates spectators through weapon model color change.~~ (Removed, may be supported via DLC in the future)
* Alerts the number of spectators by blinking the keyboard backlight (requires D-Bus support).
* **Teammate Damage Display:** Shows the damage dealt by teammates in real-time.
* **Voice Navigator:** Overlay integrated voice prompts function, providing in-game callouts like teammates. (overlay required; currently only Chinese voice pack available, more contributions welcome)
* **DLC support**: Write JavaScript extensions and package as apexsky DLC to share with friends; download and install DLC to expand functionality.
* Automatically SuperGlide via DLC
* **Safety:** Only modify the memory when specific functions such as glow are enabled.
* Supports controlling the mouse via Kmbox, QEMU QMP, etc., at which point aimbot does not need to modify memory (no evidence suggests this is safer).
* **User-Friendly Customization:** Interactive terminal menu with a more intuitive and user-friendly design for customization.
* **Config:** Fine customization achieved through saving and editing settings in settings.toml file.
* **Offsets Loader:** Supports dynamic loading of offsets from offsets.ini file, making it convenient to keep up with game version updates.
* **Multilingual Support:** Supports multilingual functionality for global accessibility.

Please star if you like it.
Look forward to your testing and feedback.



## Getting Started



### Usage



**Play with DMA:**

```shell
./apexsky_dma.exe --pcileech fpga
```



**Play with VM:**

There are really only two steps:

1. Run the game on a windows guest in a kvm virtual machine.

2. Run the compiled apex_dma program on the Linux host.

   * Using the DMA Library

     Find the virtual machine process PID and QMP address after starting the virtual machine, then run the compiled apexsky_dma program on the Linux host.

     ```shell
     sudo ./apexsky_dma --pcileech qemu://hugepage-pid=<PID>,qmp=<QMP_ADDRESS>
     ```

     For example, if your VM's PID is 5678 and the QMP address is /tmp/qmp-win11.sock, the command would be:

     ```shell
     sudo ./apexsky_dma --pcileech qemu://hugepage-pid=5678,qmp=/tmp/qmp-win11.sock
     ```

     For more details, see https://github.com/ufrisk/LeechCore/wiki/Device_QEMU

   * Using Memflow

     ```bash
     sudo ./apexsky_dma kvm
     ```



### Overlay (optional)

First ensure that the ESP service is enabled, either by enabling it in the menu or by editing the settings file.

ESP is now implemented as a stand-alone program and is currently available for the Linux/Windows/Web platform.

The official `apexsky_overlay` currently supports Linux/Windows/Web platforms, all of which are interoperable and can be connected remotely. You can also choose or write your own preferred overlay.

`apexsky_overlay` supports mini-map radar (supports teammate perspective), health bars (supports teammate perspective), displays player boxes and skeletons, displays nearby loot and death boxes, and more.

* Linux

     ```bash
     ./apexsky_overlay
     ```
     
     Recommended for **non-**single GPU passthrough users, so that both `apexsky_overlay` and `apexsky` run outside the virtual machine.
     
     Suitable for Linux players, but not guaranteed to be undetected.

* Windows

  `apexsky_overlay.exe`

  Recommended for DMA users paired with an Video Synthesizer on a secondary machine.

  Can also be used inside a virtual machine, but not guaranteed to be undetected.

* Web

  Access via browser at https://chettoy.github.io/apexsky/

  Recommended for use on tablets or shared with teammates via LAN.

  Also serves as a backup option.



Additional information:

1. Please put the overlay window on the top of the VM screen after start. For example, on top of the looking-glass window.
2. For a better experience, please passthrough your keyboard, mouse or controller into the VM.
3. ~~Press Insert to open the Overlay menu.~~ Press and hold the Insert key to temporarily interact with the overlay.
4. If you are using a resolution other than 1080p, save the configuration and then modify the `screen_width` and `screen_height` in *settings.toml* and reload the configuration.



### DLC (optional)

To install DLCs:

1. Place the `.spk` DLC package into the `dlc` folder within the same directory.
2. Navigate to the DLC menu and install DLCs.



### Download

1. **Download libraries for accessing memory**

   - If you're using the *memflow* connector, ensure you download the corresponding files and place them in the same directory:

      * [libmemflow_kvm.so](https://github.com/memflow/memflow-kvm/releases/download/bin-stable/libmemflow_kvm.x86_64.so)
      * [libmemflow_qemu.so](https://github.com/memflow/memflow-qemu/releases/download/bin-0.2.1/libmemflow_qemu.x86_64.so)
      * [libmemflow_win32.so](https://github.com/memflow/memflow-win32/releases/download/bin-stable/libmemflow_win32.x86_64.so)

   - If you're using MemProcFS or LeechCore

      - Download and extract the files from [MemProcFS Releases](https://github.com/ufrisk/MemProcFS/releases).

      - FTDI drivers have to be installed if FPGA is used on Windows.

         Download the 64-bit [`FTD3XX.dll`](https://ftdichip.com/wp-content/uploads/2023/11/FTD3XXLibrary_v1.3.0.8.zip) from FTDI and place it alongside `leechcore.dll`.

         If using the FT2232H instead of the FT601 please download [D2XX drivers from ftdichip](https://ftdichip.com/drivers/d2xx-drivers/).

2. **Download apexsky**

   Click on *[Actions](https://github.com/chettoy/apexsky/actions)* to download the auto-built artifacts.

   Or compile it yourself.



## Build from source

**Requirements:**

* ~~C++ toolchain~~
* Rust toolchain
* ~~CMake~~
* Git
* Protoc (protobuf)

**Install Rust nightly:**

Run the following command to install `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Set nightly as the default toolchain:

```bash
rustup default nightly
```

**Install Build Dependencies (Ubuntu):**

```bash
sudo apt install clang protobuf-compiler libusb-1.0-0-dev libzstd-dev pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 libwayland-dev libxkbcommon-dev lld
```

**Install Build Dependencies (Arch):**

```bash
sudo pacman -S clang protobuf libusb zstd libx11 pkgconf alsa-lib wayland-protocols wayland lld
```

**Build:**

```shell
git clone --recurse https://github.com/chettoy/apexsky
cd apexsky
git checkout next
git submodule update --init --recursive
cd apexsky
cargo build --release
cd apexsky_overlay
cargo build --release
```



## FAQ

1. Many people are using a single GPU

    > If there is only one GPU and the linux host can't be displayed after  starting the VM, you can still use features other than the overlay.
    > In this case you will not be able to use only the overlay-dependent features such as the mini-map radar, health shield bars, spectator list, etc. But all other features such as player glow, terminal menu and aimbot will still work.
    
2. It seems that the client is still reading the values required for the esp stuff. If AC is looking for access on those specific memory locations, then IDK if just removing the implementation of the overlay will work in  terms of preventing detection. Or is AC simply detecting the presence of the overlay/client itself, and banning due to that?

    > First of all, everything related to game state is realized by *access on those specific memory locations*. So we need to use DMA or VM techniques to access memory covertly.
    > AC detects the overlay client, so we re-implement the overlay outside the VM and remove the client.

3. How to load new offsets after a game update

    > Place a updated offsets.ini in the same directory to automatically load the new offsets instead of the built-in offsets.
    > This may not always be enough to keep up with changes in game updates, but for the most part this will allow play to continue.



## Acknowledgments

* [memflow](https://github.com/memflow/memflow)
* [MemProcFS](https://github.com/ufrisk/MemProcFS)
* [ratatui](https://ratatui.rs)
* [tracel-ai/burn](https://github.com/tracel-ai/burn)
* [TheCruz's Apex Aimbot+ESP](https://www.unknowncheats.me/forum/apex-legends/369786-apex-directx-wallhack-smooth-aimbot-source.html)
* [h33p/vmread](https://github.com/h33p/vmread)
* [Y33Tcoder/EzApexDMAAimbot](https://github.com/Y33Tcoder/EzApexDMAAimbot)
* [MisterY52/apex_dma_kvm_pub](https://github.com/MisterY52/apex_dma_kvm_pub)
* [KrackerCo/apex_dma_kvm_pub](https://github.com/KrackerCo/apex_dma_kvm_pub)
* [CasualX/apexdream](https://github.com/CasualX/apexdream)
* [Nexilist/xap-client](https://github.com/Nexilist/xap-client)
* [Xnieno/ApexDreamForYou](https://github.com/Xnieno/ApexDreamForYou)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
[contributors-shield]: https://img.shields.io/github/contributors/chettoy/apex_dma_kvm_pub.svg?style=for-the-badge
[contributors-url]: https://github.com/chettoy/apex_dma_kvm_pub/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/chettoy/apex_dma_kvm_pub.svg?style=for-the-badge
[forks-url]: https://github.com/chettoy/apex_dma_kvm_pub/network/members
[issues-shield]: https://img.shields.io/github/issues/chettoy/apex_dma_kvm_pub.svg?style=for-the-badge
[issues-url]: https://github.com/chettoy/apex_dma_kvm_pub/issues
