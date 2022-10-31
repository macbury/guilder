-- DROP VIEW accounts_metadata;
CREATE OR REPLACE VIEW accounts_metadata AS
WITH
  accounts_with_original_balances AS (
    SELECT
      "accounts"."id",
      CONCAT("bonds"."currency", "accounts"."currency") as exchange_rate_id,
      SUM("price") AS original_balance
    FROM accounts
    LEFT JOIN
      "bonds" ON "bonds"."account_id" = "accounts"."id"
    LEFT JOIN
      "bond_performances" ON "bond_performances"."bond_id" = "bonds"."id"
    GROUP BY
      "accounts"."id", "bonds"."currency"
  ),
  accounts_with_balances AS (
    SELECT
      DISTINCT ON ("accounts_with_original_balances"."id")
      "accounts_with_original_balances".*,
      "original_balance" * COALESCE("price", 1) AS "balance"
    FROM "accounts_with_original_balances"
    LEFT JOIN
      "exchange_rates" ON
        "accounts_with_original_balances"."exchange_rate_id" = "exchange_rates"."id" AND
        "exchange_rates"."date" = now()::date
  )
SELECT
  "accounts"."id" AS "account_id",
  "accounts"."currency",
  COALESCE("balance", 0) AS "balance"
FROM "accounts"
LEFT OUTER JOIN
  "accounts_with_balances" ON
    "accounts"."id" = "accounts_with_balances"."id"
