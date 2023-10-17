# Backend TA72

## Group

- Valentin DOREAU
- Kilian GOËTZ
- Osee Brayan TCHAPPI

# Documentation

The documentation is accessible at the [`/doc`](localhost:8000/doc) route when running the API.  
You can get the `openapi.json` file from this page.

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

It is highly encouraged to take example on other pieces of code and understand the different parts and how they interact together.

This is written in Rust, as a general rule of thumbs, if it compiles, it mostly works.

## Model

1. Run `diesel migration generate create_<table>` to create the migrations
2. Complete the newly created `up.sql` and `down.sql`
3. Create a file named after your model in `src/models/`
4. This file should contain a normal, updating and new `struct` (eg. `Skill`, `UpdatingSkill`, `NewSkill`). The fields should be in the same order as in the sql definition.

Here are the usual `derive` for each `struct`, use your common sense to determine if they need others.

| `struct` | `derive` |
|-|-|
| normal | `Serialize`, `Queryable`, `ToSchema` |
| updating | `Deserialize`, `AsChangeset`, `ToSchema` |
| new | `Deserialize`, `Insertable`, `ToSchema`, `IntoParams` |

## Route

Once the model is created, create a file in `src/routes/` with the same name as the model.

1. Create a `routes` function that returns a `Scope`, this will be to register the routes in this file.
2. Add one function per route, add the correct macro from actix. Also add the `path` macro from utoipa to generate documentation
3. Create a `<type>Doc` `struct` that references the different routes and response `struct`, this is to generate the OpenApi documentation
4. Add this `struct` to the `doc` function in `src/documentation`

## Documentation

We use [utoipa](https://github.com/juhaku/utoipa) to generate [OpenApi](https://www.openapis.org/) documentation.
