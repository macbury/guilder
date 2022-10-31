-- DROP VIEW IF EXISTS bonds_monthly_performance;
CREATE OR REPLACE VIEW bonds_monthly_performance AS
SELECT
  DISTINCT ON("month", "bond_id") date_trunc('month', "date")::date AS "month",
  "bond_id",
  "date",
  "start_price",
  "currency",
  "period",
  "price",
  "rate",
  "price_change",
  "percent_change",
  "buyout_price"
FROM "interest_rates_history"
ORDER BY "month" DESC, "bond_id", "date" DESC;
