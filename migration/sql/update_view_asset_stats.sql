-- DROP VIEW asset_performances;
CREATE OR REPLACE VIEW asset_performances AS
WITH
  prices AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind")
      COALESCE("points"."value", 0) AS "price",
      "points"."resource_id" AS "asset_id",
      "points"."date",
      COALESCE(LEAD("value", 1) OVER (PARTITION BY "resource_id" ORDER BY "date" DESC), "points"."value", 0) AS last_price
    FROM "points"
    WHERE
      "points"."date" >= NOW() - INTERVAL '1 week' AND "kind" = 'P' AND "resource_type" = 'Asset'
    ORDER BY
      "resource_id" DESC, "kind" DESC, "date" DESC
  ),
  high_low_52 AS (
    SELECT
      MAX("value") AS high_value,
      MIN("value") AS low_value,
      "points"."resource_id" AS "asset_id"
    FROM "points"
    WHERE
      "points"."date" >= NOW() - INTERVAL '52 weeks' AND "kind" = 'P' AND "resource_type" = 'Asset'
    GROUP BY "points"."resource_id"
  ),
  yearly AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind")
      "points"."value" AS "price",
      "points"."resource_id" AS "asset_id",
      "points"."date"
    FROM "points"
    WHERE
      "points"."date" <= NOW() - INTERVAL '1 year' AND "kind" = 'P' AND "resource_type" = 'Asset'
    ORDER BY
      "resource_id" DESC, "kind" DESC, "date" DESC
  ),
  year_to_date AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind")
      "points"."value" AS "price",
      "points"."resource_id" AS "asset_id",
      "points"."date"
    FROM "points"
    WHERE
      "points"."date" >= date_trunc('YEAR', NOW())::DATE AND "kind" = 'P' AND "resource_type" = 'Asset'
    ORDER BY
      "resource_id" DESC, "kind" DESC, "date" ASC
  )
SELECT
  "assets"."id" AS asset_id,
  COALESCE("prices"."price", 0) AS price,
  COALESCE("prices"."last_price", 0) AS last_price,
  price_difference("prices"."price"::float, "prices"."last_price"::float) AS price_change,
  daily_change_percent("prices"."price","prices"."last_price") AS percent_change,
  "prices"."date" AS price_date,
  COALESCE("yearly"."price", 0) AS yearly_price,
  price_difference("prices"."price", "yearly"."price") AS yearly_change,
  daily_change_percent("prices"."price","yearly"."price") AS yearly_percent_change,
  "yearly"."date" AS yearly_date,
  "high_low_52"."high_value" AS high_value,
  "high_low_52"."low_value" AS low_value,
  high_low_score("high_low_52"."low_value", "prices"."price", "high_low_52"."high_value") AS low_high_score,
  "year_to_date"."price" AS ytd_price,
  price_difference("prices"."price", "year_to_date"."price") AS ytd_change,
  ytd("prices"."price", "year_to_date"."price") AS ytd_percent_change,
  "year_to_date"."date" AS ytd_date
FROM "assets"
LEFT JOIN
  "prices" ON "prices"."asset_id" = "assets"."id"
LEFT JOIN
  "high_low_52" ON "high_low_52"."asset_id" = "assets"."id"
LEFT JOIN
  "yearly" ON "yearly"."asset_id" = "assets"."id"
LEFT JOIN
  "year_to_date" ON "year_to_date"."asset_id" = "assets"."id";
