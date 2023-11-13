CREATE TABLE "patients" (
  "id" bigserial PRIMARY KEY,
  "id_user" bigint UNIQUE NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
  "id_address" bigint NOT NULL REFERENCES "addresses" ("id") ON DELETE CASCADE
);
