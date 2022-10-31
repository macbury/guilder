-- DROP VIEW wallets_metadata;
CREATE OR REPLACE VIEW wallets_metadata AS
WITH
  wallets_with_original_balances AS (
    SELECT
      "wallets"."id",
      CONCAT("bonds"."currency", "wallets"."currency") as exchange_rate_id,
      SUM("price") AS original_balance
    FROM wallets
    LEFT JOIN
      "bonds" ON "bonds"."wallet_id" = "wallets"."id"
    LEFT JOIN
      "bond_performances" ON "bond_performances"."bond_id" = "bonds"."id"
    GROUP BY
      "wallets"."id", "bonds"."currency"
  ),
  wallets_with_balances AS (
    SELECT
      DISTINCT ON ("wallets_with_original_balances"."id")
      "wallets_with_original_balances".*,
      "original_balance" * COALESCE("price", 1) AS "balance"
    FROM "wallets_with_original_balances"
    LEFT JOIN
      "exchange_rates" ON
        "wallets_with_original_balances"."exchange_rate_id" = "exchange_rates"."id" AND
        "exchange_rates"."date" = now()::date
  )
SELECT
  "wallets"."id" AS "wallet_id",
  "wallets"."currency",
  COALESCE("balance", 0) AS "balance"
FROM "wallets"
LEFT OUTER JOIN
  "wallets_with_balances" ON
    "wallets"."id" = "wallets_with_balances"."id"
