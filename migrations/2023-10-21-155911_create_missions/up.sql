CREATE TABLE "missions" (
  "id" bigserial PRIMARY KEY,
  "desc" text,
  "start" timestamp NOT NULL,
  "end" timestamp NOT NULL,
  "recurrence_days" smallint,
  "people_required" smallint NOT NULL DEFAULT 1,
  "minutes_duration" int NOT NULL,
  "archived" bool NOT NULL DEFAULT false,
  "id_mission_type" bigint NOT NULL REFERENCES "mission_types" ("id") ON DELETE CASCADE,
  "id_patient" bigint NOT NULL REFERENCES "patients" ("id") ON DELETE CASCADE
);
