build-riscv:
    cargo build --target riscv64gc-unknown-none-elf

docker:
    docker run --rm -it --platform linux/riscv64 -v "$(pwd)":/init-shell riscv64/debian:unstable /bin/bash