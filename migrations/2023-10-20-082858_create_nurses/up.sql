CREATE TABLE "nurses" (
  "id" bigserial PRIMARY KEY,
  "minutes_per_week" int NOT NULL,
  "id_user" bigint UNIQUE NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
  "id_address" bigint NOT NULL REFERENCES "addresses" ("id") ON DELETE CASCADE
);

