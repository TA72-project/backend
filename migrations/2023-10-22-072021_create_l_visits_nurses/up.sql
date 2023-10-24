CREATE TABLE "l_visits_nurses" (
  "id_visit" bigint NOT NULL REFERENCES "visits" ("id"),
  "id_nurse" bigint NOT NULL REFERENCES "nurses" ("id"),
  "report" text,
  PRIMARY KEY ("id_visit", "id_nurse")
);

