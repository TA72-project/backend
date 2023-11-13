CREATE TABLE "l_nurses_skills" (
  "id_nurse" bigint NOT NULL REFERENCES "nurses" ("id") ON DELETE CASCADE,
  "id_skill" bigint NOT NULL REFERENCES "skills" ("id") ON DELETE CASCADE,
  PRIMARY KEY ("id_nurse", "id_skill")
);

