build-riscv:
    cargo build --target riscv64gc-unknown-none-elf

test:
    cargo test -p clam

docker:
    docker run --rm -it --platform linux/riscv64 -v "$(pwd)":/init-shell -w /init-shell riscv64/debian:unstable

emulate:
    docker run --rm --privileged multiarch/qemu-user-static --reset -p yes