CREATE TABLE "l_nurses_skills" (
  "id_nurse" bigint NOT NULL REFERENCES "nurses" ("id"),
  "id_skill" bigint NOT NULL REFERENCES "skills" ("id"),
  PRIMARY KEY ("id_nurse", "id_skill")
);

