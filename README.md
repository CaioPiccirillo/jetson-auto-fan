# Jetson Nano's fan automatic control

A simple application written in Rust for controlling Jetson Nano's fan.

## Dependencies

```sh
cargo install cargo-deb
```
## Packaging

```sh
cargo deb
```

## Installation

```sh
apt install ./target/debian/jetson-auto-fan_<version>_arm64.deb
```

## Starting application

```sh
systemctl start jetson-auto-fan
```