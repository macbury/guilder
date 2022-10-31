-- DROP MATERIALIZED VIEW IF EXISTS exchange_rates;
-- https://dba.stackexchange.com/questions/72419/filling-in-missing-dates-in-record-set-from-generate-series
-- generate series date, and fill missing using lead?
-- Series will have gaps, because of how trading view is returning data
CREATE MATERIALIZED VIEW exchange_rates AS
WITH
  "values" AS (
    SELECT
      DISTINCT ON("date", "resource_id")
      "resource_id" AS "currency",
      "date",
      "points"."value" AS "price"
    FROM "points"
    LEFT JOIN
      "assets" ON "assets"."id" = "points"."resource_id" AND "points"."resource_type" = 'Asset'
    WHERE ("assets"."kind" = 'forex' OR "assets"."kind" = 'crypto') AND "points"."kind" = 'P'
  ),
  "range" AS (
    SELECT
      "currency",
      MIN("date") AS "start",
      MAX("date") AS "end"
    FROM "values"
    GROUP BY "currency"
  ),
  "series" AS (
    SELECT
      generate_series("range"."start", "range"."end", '1d')::date as "date",
      "start",
      "end",
      "currency"
    FROM "range"
  ),
  "rates" AS (
    SELECT
      COALESCE("values"."currency", "series"."currency") AS "id",
      COALESCE("values"."date", "series"."date") AS "date",
      -- "values"."price" AS "current_price",
      COALESCE(
        "values"."price",
        LAG("values"."price", 1) OVER (
          PARTITION BY "series"."currency"
          ORDER BY "series"."date"
        ),
        LAG("values"."price", 2) OVER (
          PARTITION BY "series"."currency"
          ORDER BY "series"."date"
        ),
        LAG("values"."price", 3) OVER (
          PARTITION BY "series"."currency"
          ORDER BY "series"."date"
        ),
        LAG("values"."price", 4) OVER (
          PARTITION BY "series"."currency"
          ORDER BY "series"."date"
        ),
        LAG("values"."price", 5) OVER (
          PARTITION BY "series"."currency"
          ORDER BY "series"."date"
        ),
        LAG("values"."price", 6) OVER (
          PARTITION BY "series"."currency"
          ORDER BY "series"."date"
        ),
        LAG("values"."price", 7) OVER (
          PARTITION BY "series"."currency"
          ORDER BY "series"."date"
        )
      ) AS "price"
    FROM "values"
    RIGHT JOIN
      "series" ON "series"."currency" = "values"."currency" AND "series"."date" = "values"."date"
    ORDER BY "date"
  )
SELECT
  *
FROM "rates"
WHERE "price" IS NOT NULL
