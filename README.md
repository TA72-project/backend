# Backend TA72

## Group

- Valentin DOREAU
- Kilian GOËTZ
- Osee Brayan TCHAPPI

# Running

## Docker

```sh
docker-compose up -d
```

## Local machine

# Contributing

1. Run `diesel migration generate create_<table>` to create the migrations
2. Complete the newly created `up.sql` and `down.sql`
3. Create a file named after your model in `src/models/`
4. Create a `struct` with the model name and the corresponding fields
5. This `struct` derives from `Debug`, `Clone`, `Serialize` & `Deserialize`
6. The model file should also contains `struct` for a new model and an updating model

