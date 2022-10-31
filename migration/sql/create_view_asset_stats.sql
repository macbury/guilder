CREATE OR REPLACE VIEW asset_performances AS
WITH
  prices AS (
    SELECT
      DISTINCT ON("points"."resource_id", "points"."kind")
      "points"."value",
      "points"."resource_id" AS "asset_id",
      "points"."date"
    FROM "points"
    WHERE
      "points"."date" >= NOW() - INTERVAL '1 week' AND "kind" = 'P' AND "points"."resource_type" = 'Asset'
    ORDER BY
      "asset_id" DESC, "kind" DESC, "date" DESC
  ),
  high_low_52 AS (
    SELECT
      MAX("value") AS high_value,
      MIN("value") AS low_value,
      "points"."resource_id" AS "asset_id"
    FROM "points"
    WHERE
      "points"."date" >= NOW() - INTERVAL '52 weeks' AND "kind" = 'P' AND "points"."resource_type" = 'Asset'
    GROUP BY "points"."resource_id"
  )
SELECT
  "assets"."id" AS asset_id,
  "prices"."value" AS price,
  "prices"."date" AS price_date,
  "high_low_52"."high_value" AS high_value,
  "high_low_52"."low_value" AS low_value
FROM "assets"
LEFT JOIN
  "prices" ON "prices"."asset_id" = "assets"."id"
LEFT JOIN
  "high_low_52" ON "high_low_52"."asset_id" = "assets"."id";
