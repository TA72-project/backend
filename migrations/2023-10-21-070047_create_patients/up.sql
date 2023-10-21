CREATE TABLE "patients" (
  "id" bigserial PRIMARY KEY,
  "id_user" bigint UNIQUE NOT NULL REFERENCES "users" ("id"),
  "id_address" bigint NOT NULL REFERENCES "addresses" ("id")
);
