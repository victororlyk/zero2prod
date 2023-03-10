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


## tests

run one test and see the logs
```
TEST_LOG=true cargo test  non_existing_user_is_rejected | bunyan
```
with greping some specific part
```
TEST_LOG=true cargo test  non_existing_user_is_rejected | bunyan
```


### scripts
to make script executable and then run it
```bash
chmod +x ./scripts/init_redis.sh
./script/init_redis.sh
```