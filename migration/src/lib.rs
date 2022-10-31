use sea_orm_migration::prelude::*;
pub use sea_orm_migration::migrator::MigratorTrait;
pub mod view;

mod m20220099_000000_create_change_func;
mod m20220101_000002_create_users_table;
mod m20220101_000003_create_assets_table;
mod m20220101_000004_add_metadata_to_assets;
mod m20220101_000006_create_points;
mod m20220101_000007_create_points_upsert_index;
mod m20220101_000007_create_view;
mod m20220100_000001_create_helper_functions;
mod m20220100_000003_create_ytd_function;
mod m20220513_140521_update_asset_stats;
mod m20220100_000002_create_score_function;
mod m20220519_095224_create_comments;
mod m20220519_100706_create_comments_index;
mod m20220523_124816_create_categories;
mod m20220525_101158_add_category_id_to_assets;
mod m20220530_102824_create_accounts;
mod m20220603_140834_create_integrations;
mod m20220606_201044_create_bonds;
mod m20220607_192843_create_fk_assets_points;
mod m20220608_185348_create_fk_bonds_points;
mod m20220608_190440_create_bond_performances;
mod m20220619_104537_drop_comments_foregin;
mod m20220620_130526_create_interest_rate_history_view;
mod m20220620_154636_update_daily_change_procedure;
mod m20220621_185134_add_periods_to_bonds;
mod m20220622_185853_create_bonds_monthly_performance;
mod m20220624_185902_update_bond_month_performance;
mod m20220624_190452_create_bond_balance_performances;
mod m20220630_191253_update_bond_performances;
mod m20220630_194402_update_bond_balance_performances;
mod m20220704_183106_create_wallets;
mod m20220706_202701_add_wallet_to_bonds;
mod m20220712_093742_bond_balances_view_archived;
mod m20220712_194828_add_kind_to_assets;
mod m20220713_191822_create_exchange_rates_view;
mod m20220714_190556_create_accounts_with_balances_view;
mod m20220716_091349_create_wallets_metadata;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20220099_000000_create_change_func::Migration),
      Box::new(m20220100_000001_create_helper_functions::Migration),
      Box::new(m20220100_000002_create_score_function::Migration),
      Box::new(m20220100_000003_create_ytd_function::Migration),
      Box::new(m20220101_000002_create_users_table::Migration),
      Box::new(m20220101_000003_create_assets_table::Migration),
      Box::new(m20220101_000004_add_metadata_to_assets::Migration),
      Box::new(m20220101_000006_create_points::Migration),
      Box::new(m20220101_000007_create_points_upsert_index::Migration),
      Box::new(m20220101_000007_create_view::Migration),
      Box::new(m20220513_140521_update_asset_stats::Migration),
      Box::new(m20220519_095224_create_comments::Migration),
      Box::new(m20220519_100706_create_comments_index::Migration),
      Box::new(m20220523_124816_create_categories::Migration),
      Box::new(m20220525_101158_add_category_id_to_assets::Migration),
      Box::new(m20220530_102824_create_accounts::Migration),
      Box::new(m20220603_140834_create_integrations::Migration),
      Box::new(m20220606_201044_create_bonds::Migration),
      Box::new(m20220607_192843_create_fk_assets_points::Migration),
      Box::new(m20220608_185348_create_fk_bonds_points::Migration),
      Box::new(m20220608_190440_create_bond_performances::Migration),
      Box::new(m20220619_104537_drop_comments_foregin::Migration),
      Box::new(m20220620_130526_create_interest_rate_history_view::Migration),
      Box::new(m20220620_154636_update_daily_change_procedure::Migration),
      Box::new(m20220621_185134_add_periods_to_bonds::Migration),
      Box::new(m20220622_185853_create_bonds_monthly_performance::Migration),
      Box::new(m20220624_185902_update_bond_month_performance::Migration),
      Box::new(m20220624_190452_create_bond_balance_performances::Migration),
      Box::new(m20220630_191253_update_bond_performances::Migration),
      Box::new(m20220630_194402_update_bond_balance_performances::Migration),
      Box::new(m20220704_183106_create_wallets::Migration),
      Box::new(m20220706_202701_add_wallet_to_bonds::Migration),
      Box::new(m20220712_093742_bond_balances_view_archived::Migration),
      Box::new(m20220712_194828_add_kind_to_assets::Migration),
      Box::new(m20220713_191822_create_exchange_rates_view::Migration),
      Box::new(m20220714_190556_create_accounts_with_balances_view::Migration),
      Box::new(m20220716_091349_create_wallets_metadata::Migration)
    ]
  }
}
