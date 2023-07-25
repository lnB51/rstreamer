![Version 0.1](https://img.shields.io/badge/Version%200.1-FFC832?style=for-the-badge&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000?style=for-the-badge&logo=rust&logoColor=white)
[![MIT License](https://img.shields.io/badge/MIT%20License-004772?style=for-the-badge&logo=license&logoColor=white)](https://github.com/b1rd-dev/rust_grep/blob/main/LICENSE.md)

# Rust http camera streamer

## Installation

Download source code from github: 

`git clone https://github.com/lnB51/rstreamer`

<hr/>

Build the project:

`cargo buid` or `cargo buid --release`

<hr/>

Build for raspberry pi

```cargo build --release --target aarch64-unknown-linux-gnu```

You have to add `.cargo/config` file with:

```rust
[target.aarch64-unknown-linux-gnu]
linker = "/usr/bin/aarch64-linux-gnu-gcc"
```

<hr/>

Run the project:

`./rstreamer` or with parameters `./rstreamer -d /dev/video0 -h localhost -p 8080 -r 1920x1080`

<br />

### Paremeters:

<hr/>

#### -d | --device

##### Used for set video device to capture from

#### Exaples:
`./rstreamer -d /dev/video0`
`./rstreamer -d /dev/video10`

<hr/>

#### -h | --host

##### Used for set host IP

#### Exaples:
`./rstreamer -h localhost`
`./rstreamer -h 0.0.0.0`
WARNING: To use `0.0.0.0` host you have to be root user

<hr/>

#### -p | --port

##### Used for set port

#### Exaples:
`./rstreamer -p 8080`
`./rstreamer -p 80`
WARNING: To use `80` port you have to be root user

<hr/>

#### -r | --resolution

##### Used for set resolution

#### Exaples:
`./rstreamer -r 1920x1080`
`./rstreamer -r 1280x720`

<hr/>

#### -hlp | --help

##### Used to get help with commands

#### Exaples:
`./rstreamer -hlp`
