# bob-management

Management UI for Bob

## Commands

To run dev. server:

```sh
cargo fullstack --features=dev
```

This will run dev. server on 21012 port for frontend and backend server on port 9000.

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
