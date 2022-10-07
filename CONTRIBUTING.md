# How to build


- Enable [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) on your system
- Install [Microsoft Terminal](https://apps.microsoft.com/store/detail/windows-terminal/9N0DX20HK701?hl=nl-nl&gl=nl) (optional)
- Install a subsystem (I prefer [Ubuntu](https://apps.microsoft.com/store/detail/ubuntu-22041-lts/9PN20MSR04DW))
- Set up your WSL enviroment with a username and pw
- Install Docker: https://docs.docker.com/engine/install/
- download the included file docker.sh (start this file before starting each time!)
- Clone the `Core` [repository](https://github.com/XB15/core)
- Navigate to the core in your WSL window: `cd ../../core`
- Build and tag the docker image: `docker build -t xb15_core .`
- Execute command: `docker run -it -v $PWD:/app xb15_core bash`
- `cd /app`
- `./wrap_for_aarch64.sh cargo build -p led_matrix_test --bin gif --release --target aarch64-unknown-linux-gnu`
Now the compiler should start whirring away.
Files will be output to: `/core/target/aarch64-unknown-linux-gnu/release`


*This guide is still being worked on*
