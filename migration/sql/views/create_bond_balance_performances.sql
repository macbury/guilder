-- DROP VIEW IF EXISTS bond_balance_performances;
CREATE OR REPLACE VIEW bond_balance_performances AS
WITH
  bond_balances AS (
    SELECT
      month,
      SUM("price") AS price,
      SUM("start_price") AS start_price,
      currency
    FROM "bonds_monthly_performance"
    GROUP BY month, currency
    ORDER BY "month" DESC
  ),
  bond_price_changes AS (
    SELECT
      month,
      "bond_balances"."currency",
      price,
      price_difference("price", "start_price") AS total_price_change,
      daily_change_percent("price", "start_price") AS total_percent_change
    FROM "bond_balances"
  ),
  bond_last_prices AS (
    SELECT
      month,
      COALESCE(LEAD("total_price_change", 1) OVER (PARTITION BY "currency" ORDER BY "month" DESC), total_price_change) AS last_total_price_change,
      COALESCE(LEAD("price", 1) OVER (PARTITION BY "currency" ORDER BY "month" DESC), "price") AS last_price
    FROM "bond_price_changes"
  )
SELECT
  "bond_balances"."month",
  "bond_balances"."start_price",
  "bond_balances"."price",
  "bond_balances"."currency",
  total_price_change,
  total_percent_change,
  price_difference("total_price_change", "last_total_price_change") AS month_price_change,
  daily_change_percent("total_price_change", "last_total_price_change") AS month_percent_change
FROM "bond_balances"
LEFT JOIN
  "bond_last_prices" ON "bond_last_prices"."month" = "bond_balances"."month"
LEFT JOIN
  "bond_price_changes" ON "bond_price_changes"."month" = "bond_balances"."month";
