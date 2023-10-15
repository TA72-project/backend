# Backend TA72

## Group

- Valentin DOREAU
- Kilian GOËTZ
- Osee Brayan TCHAPPI

# Running

## Docker

Docker is the easiest way to develop, it doesn't require any dependence.  
You can start the project using the following command. It may take some time the first time.

```sh
docker-compose up -d
```

The application is recompiled automatically on each file modification.

## Local machine

Install Rust [here](https://www.rust-lang.org/tools/install).

You'll then need to setup a Postgres database yourself. Add the environment variable
`DATABASE_URL=<postgres://<user>:<password>@<host>/<database>`.

Finally, run `cargo run` to start the server.

# Contributing

1. Run `diesel migration generate create_<table>` to create the migrations
2. Complete the newly created `up.sql` and `down.sql`
3. Create a file named after your model in `src/models/`
4. Create a `struct` with the model name and the corresponding fields
5. This `struct` derives from `Debug`, `Clone`, `Serialize` & `Deserialize`
6. The model file should also contains `struct` for a new model and an updating model

