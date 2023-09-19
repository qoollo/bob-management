# bob-management

Management UI for Bob

## Commands

---

Run debug build:

```sh
cargo run -- --default
```

---

Make release build:

```sh
cargo build --profile=release-lto
```

To run release build with default configuration:

```sh
cargo run --profile=release-lto -- --default
```

Or you can specify configuration file:

```sh
cargo run --profile=release-lto -- --config-file config.yaml
```
