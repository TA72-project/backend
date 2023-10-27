CREATE TABLE "managers" (
  "id" bigserial PRIMARY KEY,
  "id_user" bigint UNIQUE NOT NULL REFERENCES "users" ("id")
);

