CREATE TABLE "centers" (
  "id" bigserial PRIMARY KEY,
  "name" text UNIQUE NOT NULL,
  "desc" text,
  "workday_start" time NOT NULL,
  "workday_end" time NOT NULL,
  "range_km" smallint NOT NULL,
  "id_address" bigint NOT NULL REFERENCES "addresses" ("id")
);

