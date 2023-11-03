CREATE TABLE "l_missions_skills" (
  "id_mission_type" bigint NOT NULL REFERENCES "mission_types" ("id"),
  "id_skill" bigint NOT NULL REFERENCES "skills" ("id"),
  "preferred" bool NOT NULL DEFAULT false,
  PRIMARY KEY ("id_mission_type", "id_skill")
);

