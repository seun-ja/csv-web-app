# Simple CSV Creator App & Downloader

The app uses [Actix](https://actix.rs/) as its web server and Postgres database, in this instance, we're utilizing [sqlx](https://crates.io/crates/sqlx).

Few enviromental variables (ie .env) are needed to get started inlcuding:

```
DB_USERNAME=username
DB_PASSWORD=password
DB_HOST=localhost
DB_PORT=5432
DB_NAME=csv
MEMBERS_CSV_PATH="./members.csv"
```

To run the code

```
cargo run
```
