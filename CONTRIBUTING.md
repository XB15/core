# How to build

- Enable [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) on your system
- Install [Microsoft Terminal](https://apps.microsoft.com/store/detail/windows-terminal/9N0DX20HK701?hl=nl-nl&gl=nl) (optional)
- Install a subsystem (I prefer [Ubuntu](https://apps.microsoft.com/store/detail/ubuntu-22041-lts/9PN20MSR04DW))
- Set up your WSL enviroment with a username and pw
- Install [Docker](https://docs.docker.com/engine/install/)
- Download the included file `docker.sh` (start this file before starting each time!)
- Clone the `Core` [repository](https://github.com/XB15/core): `git clone https://github.com/XB15/core`
- Navigate to the core in your WSL window: `cd core`
- Build and tag the Docker image: `docker build -t xb15_core .`
- Execute command: `docker run -it -v $PWD:/app xb15_core bash`
- `cd /app`
- Depending on what Pi you're using, you can choose:
  - `./wrap_for_32bit.sh cargo build -p led_matrix_test --bin gif --release --target arm-unknown-linux-gnueabihf`
  - `./wrap_for_64bit.sh cargo build -p led_matrix_test --bin gif --release --target aarch64-unknown-linux-gnu`

Now the compiler should start whirring away.

Files will be output to either:
- `/core/target/arm-unknown-linux-gnueabihf/release` (32bit)
- `/core/target/aarch64-unknown-linux-gnu/release` (64bit)


*This guide is still being worked on*
