# rust web server project.

## db
### initial setup
```
$ SKIP_DOCKER=true ./scripts/init_db.sh
```
### manual
```
// create database
$ ./scripts/init_db.sh
// migration
$ export DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter
$ sqlx migrate add create_subscriptions_table
$ sqlx migrate run
```
