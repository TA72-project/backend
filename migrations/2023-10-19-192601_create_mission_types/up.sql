CREATE TABLE "mission_types" (
     "id" bigserial PRIMARY KEY,
     "name" text UNIQUE NOT NULL,
     "people_required" smallint NOT NULL DEFAULT 1,
     "minutes_duration" int NOT NULL
);
