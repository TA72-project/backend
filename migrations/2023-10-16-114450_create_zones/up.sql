CREATE TABLE "zones" (
  "id" bigserial PRIMARY KEY,
  "name" text NOT NULL,
  "id_center" bigint NOT NULL REFERENCES "centers" ("id")
);

