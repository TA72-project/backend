CREATE TABLE "addresses" (
  "id" bigserial PRIMARY KEY,
  "number" int,
  "street_name" text NOT NULL,
  "postcode" text NOT NULL,
  "city_name" text NOT NULL,
  "complement" text
);
