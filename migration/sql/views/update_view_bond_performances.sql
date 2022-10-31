-- DROP MATERIALIZED VIEW IF EXISTS bond_performances;
CREATE MATERIALIZED VIEW bond_performances AS
WITH
  shares AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind")
      "points"."value",
      "points"."resource_id"
    FROM "points"
    WHERE
      "points"."date" >= NOW() - INTERVAL '1 week' AND "kind" = 'V' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id" DESC, "kind" DESC, "date" DESC
  ),
  rates AS (
    SELECT
      array_agg(DISTINCT "points"."value") AS "list",
      "points"."resource_id"
    FROM "points"
    WHERE "kind" = 'R' AND "points"."resource_type" = 'Bond'
    GROUP BY "points"."resource_id"
  ),
  prices AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind")
      "points"."value",
      "points"."resource_id",
      "points"."date",
      coalesce(LEAD("value", 1) OVER (PARTITION BY "resource_id" ORDER BY "date" DESC), "points"."value") AS last_price
    FROM "points"
    WHERE
      "points"."date" >= NOW() - INTERVAL '1 week' AND "kind" = 'P' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id" DESC, "kind" DESC, "date" DESC
  ),
  buyout_prices AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind")
      "points"."value",
      "points"."resource_id",
      "points"."date"
    FROM "points"
    WHERE
      "points"."date" >= NOW() - INTERVAL '1 week' AND "kind" = 'BP' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id" DESC, "kind" DESC, "date" DESC
  ),
  start_prices AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind")
      "points"."value",
      "points"."resource_id",
      "points"."date"
    FROM "points"
    WHERE
      "points"."date" >= NOW() - INTERVAL '1 week' AND "kind" = 'S' AND "points"."resource_type" = 'Bond'
    ORDER BY
      "resource_id" DESC, "kind" DESC, "date" DESC
  )
SELECT
  "bonds"."id"::bigint AS bond_id,
  "shares"."value"::bigint AS "shares",
  "rates"."list"[array_upper("rates"."list", 1)] AS "current_rate",
  ("bonds"."end_date" - NOW()::date)::bigint AS buyout_days_left,
  ("bonds"."interest_date" - NOW()::date)::bigint AS interest_days_left,
  "start_prices"."value" AS start_price,
  "prices"."value" AS price,
  "buyout_prices"."value" AS buyout_price,
  "prices"."last_price" AS last_price,
  "prices"."date" AS price_date,
  array_to_json("rates"."list")::json AS "rates",
  price_difference("prices"."value", "prices"."last_price") AS day_price_change,
  daily_change_percent("prices"."value", "prices"."last_price") AS day_percent_change,
  price_difference("prices"."value", "start_prices"."value") AS price_change,
  daily_change_percent("prices"."value", "start_prices"."value") AS percent_change
FROM "bonds"
LEFT JOIN
  "rates" ON "rates"."resource_id" = "bonds"."id"::varchar(255)
LEFT JOIN
  "shares" ON "shares"."resource_id" = "bonds"."id"::varchar(255)
LEFT JOIN
  "prices" ON "prices"."resource_id" = "bonds"."id"::varchar(255)
LEFT JOIN
  "buyout_prices" ON "buyout_prices"."resource_id" = "bonds"."id"::varchar(255)
LEFT JOIN
  "start_prices" ON "start_prices"."resource_id" = "bonds"."id"::varchar(255);

-- REFRESH MATERIALIZED VIEW bond_performances;
