# Database

## In memory HashMap

### Specify database file in the code

```rust
    let passkey_state = AppState::with_store_types(
        ChallengeStoreType::Memory,
        CredentialStoreType::Memory,
    )
    .await?;
```


## SQLite

### Prepare database file

```bash
db=sqlite/data/data.db
rm $db && sqlx database create --database-url sqlite:$db
```

```bash
db=sqlite/data/data.db
echo "select credential_id,counter,user_handle,user_name,user_display_name from credentials" | sqlite3 $db 
echo "select challenge_id,user_name,user_display_name,timestamp from challenges" | sqlite3 $db 
```

```bash
db=sqlite/data/data.db
watch -n 1 "echo 'select credential_id,counter,user_handle,user_name,user_display_name from credentials;select challenge_id,user_name,user_display_name,timestamp from challenges' | sqlite3 $db"
```

### Specify database file in the code

```rust
    let passkey_state = AppState::with_store_types(
        ChallengeStoreType::Sqlite {
            url: "sqlite:./db/sqlite/data/data.db".to_string(),
        },
        CredentialStoreType::Sqlite {
            url: "sqlite:./db/sqlite/data/data.db".to_string(),
        },
    )
    .await?;
```

## PostgresQL

### Prepare database file

```bash
docker compose -f postgresql/docker-compose.yaml up -d
docker compose -f postgresql/docker-compose.yaml ps
```

monitor database

```bash
watch -n 1 "echo 'select credential_id,counter,user_handle,user_name,user_display_name from credentials;select challenge_id,user_name,user_display_name,timestamp from challenges'|psql postgresql://passkey:passkey@localhost:5432/passkey"
```

### Specify database in the code

```rust
    let passkey_state = AppState::with_store_types(
        ChallengeStoreType::Postgres {
            url: "postgresql://passkey:passkey@localhost:5432/passkey".to_string(),
        },
        CredentialStoreType::Postgres {
            url: "postgresql://passkey:passkey@localhost:5432/passkey".to_string(),
        },
    )
    .await?;
```

## Redis

### Prepare database file

```bash
docker compose -f redis/docker-compose.yaml up -d
docker compose -f redis/docker-compose.yaml ps
```

monitor database

```bash
watch -n 1 'redis-cli keys "*" | xargs redis-cli mget'
```
