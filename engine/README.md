## Run Locally

```bash
    ssh -i <key_link> -L <db_port>:localhost:<db_port> <name>@<vps-ip> -p <vps_port>
    cargo run
```

## Add new db migration 

```bash 
    sqlx migrate add <version_name>
```

## Add a new language adapter

### Generate the boilerplate

```bash
./new_adapter.sh <language>
```

This creates `src/adapters/<language>/mod.rs` with a scaffold implementing the `Adapter` trait.

### Register the adapter

In `src/adapters/mod.rs`, add the new module and register it in `default_adapters()`:

```rust
mod <language>;
use <language>::<Language>Adapter;

pub fn default_adapters() -> Vec<Box<dyn Adapter>> {
    vec![
        Box::new(PythonAdapter {}),
        Box::new(<Language>Adapter {}),
    ]
}
```

### Requirements

- **Implement the `Adapter` trait** (`src/analysis/connector.rs`) — `supported_extensions`, `ignore_files`, `extract`
- **Document supported kinds** via `///` on the adapter struct:
  ```rust
  /// Supported SymbolKinds: Class, Function, Method
  /// Supported RelationKinds: Import
  /// Ignored files: `__init__.py`
  pub struct PythonAdapter {}
  ```
- **Tests** — each adapter must have tests covering:
  - `extract`: ignored file, unreadable file, successful extraction
  - Symbol extraction: success and error cases per symbol kind
  - Relation extraction: success and error cases per relation kind

