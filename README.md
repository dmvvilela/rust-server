# Rust Server

This is a Postgres and Rust CRUD with Diesel ORM.

I followed this but with updated dependencies, used a simpler model and added docker:

https://blog.logrocket.com/create-a-backend-api-with-rust-and-postgres/

Create a .env file like so:

```
RUST_LOG=rest_api=info,actix=info,diesel_migrations=info
DATABASE_URL=postgres://user:password@localhost:5432/rust-server
HOST=127.0.0.1
PORT=5000
```

To use watch.sh install systemfd and cargo-watch with cargo install.

Install diesel_cli with postgres features also.

Docker commands:

```
docker network create --driver bridge postgres-network
```

```
docker run --name rust-postgres --network=postgres-network -e "POSTGRES_PASSWORD=root" -p 5432:5432 -v /home/dmvvilela/Code/PostgreSQL:/var/lib/postgresql/data -d postgres
```

```
docker run --name rust-pgadmin --network=postgres-network -p 15432:80 -e "PGADMIN_DEFAULT_EMAIL=dmvvilela@gmail.com" -e "PGADMIN_DEFAULT_PASSWORD=root" -d dpage/pgadmin4
```
