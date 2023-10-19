CREATE TABLE "users" (
  "id" bigserial PRIMARY KEY,
  "fname" text NOT NULL,
  "lname" text NOT NULL,
  "mail" text UNIQUE NOT NULL,
  "phone" text,
  "password" text,
  "token" text,
  "token_gentime" timestamp,
  "id_center" bigint NOT NULL REFERENCES "centers" ("id")
);

