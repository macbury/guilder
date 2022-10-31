-- DROP VIEW IF EXISTS interest_rates_history CASCADE;
CREATE OR REPLACE VIEW interest_rates_history AS
WITH
  periods AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."date", "points"."kind")
      "points"."value",
      "points"."resource_id",
      "points"."date"
    FROM "points"
    WHERE
      "kind" = 'PD' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id", "kind", "date" ASC
  ),
  start_prices AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."date", "points"."kind")
      "points"."value",
      "points"."resource_id",
      "points"."date"
    FROM "points"
    WHERE
      "kind" = 'S' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id", "kind", "date" ASC
  ),
  prices AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."date", "points"."kind")
      "points"."date",
      "points"."value" AS price,
      coalesce(LEAD("value", 1) OVER (PARTITION BY "resource_id" ORDER BY "date" DESC), "points"."value") AS last_price,
      "points"."resource_id"
    FROM "points"
    WHERE
      "kind" = 'P' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id", "kind", "date" ASC
  ),
  rates AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."date", "points"."kind")
      "points"."date",
      "points"."value" AS rate,
      "points"."resource_id"
    FROM "points"
    WHERE
      "kind" = 'R' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id", "kind", "date" ASC
  ),
  buyout_prices AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind", "points"."date")
      "points"."value",
      "points"."resource_id",
      "points"."date"
    FROM "points"
    WHERE
      "kind" = 'BP' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id", "kind", "date" ASC
  )
SELECT
  "prices"."resource_id"::bigint AS bond_id,
  "prices"."date" AS "date",
  "bonds"."currency",
  "prices"."last_price",
  "periods"."value"::bigint AS "period",
  "prices"."price" AS "price",
  "buyout_prices"."value" AS "buyout_price",
  "rates"."rate" AS "rate",
  "start_prices"."value" AS "start_price",
  price_difference("prices"."price", "prices"."last_price") AS day_price_change,
  daily_change_percent("prices"."price", "prices"."last_price") AS day_percent_change,
  price_difference("prices"."price", "start_prices"."value") AS price_change,
  daily_change_percent("prices"."price", "start_prices"."value") AS percent_change
FROM "prices"
LEFT JOIN
  "rates" ON "rates"."resource_id" = "prices"."resource_id" AND "rates"."date" = "prices"."date"
LEFT JOIN
  "start_prices" ON "start_prices"."resource_id" = "prices"."resource_id" AND "start_prices"."date" = "prices"."date"
LEFT JOIN
  "periods" ON "periods"."resource_id" = "prices"."resource_id" AND "periods"."date" = "prices"."date"
LEFT JOIN
  "buyout_prices" ON "buyout_prices"."resource_id" = "prices"."resource_id" AND "buyout_prices"."date" = "prices"."date"
LEFT JOIN
  "bonds" ON "bonds"."id"::varchar(255) = "prices"."resource_id";
