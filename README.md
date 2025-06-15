## Ai Horde Manager

This will be a manager for the [Ai Horde](https://aihorde.net/).

The main goal of this project is to simplify starting an image worker, so instead of the somewhat tedious process you currently need to do, this project will automatically detect the optimal settings for your hardware, and configure it as such. 

After that, the other goals are being able to manage the client(s) from one interface, so being able to configure settings, see kudos, etc, all over the network.

> [!NOTE]
> If you are on windows scoop will be installed for git & python if they are not already installed. If you are on linux your distros package manager will be used. 

> Supported package managers are:
> - pacman
> - dnf
> - apt

> More will be added.

> [!NOTE]  
> This project is still in very early alpha. Do not expect things to work. 

To run the dev build, you can clone this repo, then run `cargo build` or `cargo run`. you can also append `-r` to run an optimized build. Note that compilation will take longer.

Eventually, when the project is in a somewhat stable state, you will be able to download a prebuilt binary from the release page. 