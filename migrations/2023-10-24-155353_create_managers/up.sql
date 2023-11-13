CREATE TABLE "managers" (
  "id" bigserial PRIMARY KEY,
  "id_user" bigint UNIQUE NOT NULL REFERENCES "users" ("id") ON DELETE CASCADE,
  "id_center" bigint NOT NULL REFERENCES "centers" ("id") ON DELETE CASCADE
);

