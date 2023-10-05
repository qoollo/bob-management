# bob-management

Management UI for Bob

## Commands

Build backend server:

```sh
cargo build-backend
```

Run backend server:

```sh
cargo run-backend
```

Build frontend and move it into the backend's executable directory:

```sh
cargo build-frontend
```

---

Run debug build (Backend + Frontend):

```sh
cargo run -- --default
```

---

Make release build (Backend + Frontend):

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
