CREATE TABLE "availabilities" (
  "id" bigserial PRIMARY KEY,
  "start" timestamp NOT NULL,
  "end" timestamp NOT NULL,
  "recurrent" bool NOT NULL,
  "id_nurse" bigint NOT NULL REFERENCES "nurses" ("id") ON DELETE CASCADE
);

