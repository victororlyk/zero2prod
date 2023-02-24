# local development
test app
```
cargo test
```
run app
```
cargo run
```
## db
create new migration
```
sqlx migrate add migration_name
```
run migration against local db
```bash
SKIP_DOCKER=true ./scripts/init_db.sh
```