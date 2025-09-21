# Concurrent metro - Java version

Project created for concurrent programming class.

Stack: Gradle, Java, JavaFX

## Idea

1. Two config files are specified
   * [basic_map_routes.yml](app/src/main/resources/org/example/routes/basic_map_routes.yml)
     * routes definition
   * [basic_map.fxml](app/src/main/resources/org/example/maps/basic_map.fxml)
     * board definition
2. According to config a grid of given size is created
3. Threads are spawned for each train
4. Each train thread updates the board separately
5. Trains must not crash!

## Quick start

### Using nix

Prerequisites:
* [nix](https://nixos.org) (flakes enabled)

```shell
nix develop
gradle run # within nix shell
```

### Using local tools

Prerequisites:
* Java 21
* gradle

```shell
gradle run
```

---

For Rust [WIP] version please see `rust-version` branch