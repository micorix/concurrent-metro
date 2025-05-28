# Concurrent metro - Rust version [work in progress]

Project created for concurrent programming classes.

Stack: Rust, Tauri, TypeScript, React, Tailwind.

## Idea

1. Config file is selected using a dialog [metro_config.yml](./src-tauri/metro_config.yml)
2. According to config a grid of given size is created
3. Threads are spawned for each train
4. Each train emit render command through Tauri IPC
5. Frontend updates the grid reactively

Backend core is located in src-tauri/src dir.
Frontend is located in src dir.

## Status

Currently broken. Rust borrowing rules + RAII defeated me ([src-tauri/src/train_thread.rs:L66](src-tauri/src/train_thread.rs)).
See main branch for original Java version.

## Quick start

### Using nix

Prerequisites:
* nix (flakes enabled)

In progress

### Using docker

```shell
xhost +local:docker # allow forwarding (linux)

docker compose up

xhost -local:docker # reset (linux)
```

### Using local tools

Prerequisites:
* Tauri prerequisites (https://tauri.app/start/prerequisites/#linux)
* pnpm (https://pnpm.io)

```shell
pnpm install
pnpm tauri dev
```
