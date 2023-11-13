CREATE TABLE "visits" (
  "id" bigserial PRIMARY KEY,
  "start" timestamp NOT NULL,
  "end" timestamp NOT NULL,
  "id_mission" bigint NOT NULL REFERENCES "missions" ("id") ON DELETE CASCADE
);

