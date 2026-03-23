## Run Locally

```bash
    ssh -i <key_link> -L <db_port>:localhost:<db_port> <name>@<vps-ip> -p <vps_port>
    cargo run
```

## Add new db migration 

```bash 
    sqlx migrate add <version_name>
```

