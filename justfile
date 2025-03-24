# Build the RISCV64 target
build-riscv:
    cargo build --target riscv64gc-unknown-none-elf

# Run tests for library only
test:
    cargo test -p clam

# Run a shell in a RISCV64 container
docker: build-riscv
    docker run --rm -it --platform linux/riscv64 -v "$(pwd)":/init-shell -w /init-shell riscv64/debian:unstable ./target/riscv64gc-unknown-none-elf/debug/init-shell

# Allow for running RISCV64 binaries on x86_64
emulate:
    docker run --rm --privileged multiarch/qemu-user-static --reset -p yes