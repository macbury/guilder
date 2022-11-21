use once_cell::sync::Lazy;
use anyhow::Result;
use rusty_money::{iso::{Currency, PLN}, Money};
use thirtyfour::prelude::*;
use itertools::join;
use tokio::time::{sleep, Duration};
use chrono::NaiveDate;

use crate::{
  PolishPeso,
  PKOBPBond,
  PKOBPBondRates,
  BondKind
};

#[derive(Clone)]
pub struct PKOBPAccount<'a> {
  selenium_hub_url: &'a str,
  login: &'a str,
  password: &'a str,
  pub cash : Money<'a, Currency>,
  pub bonds: Vec<PKOBPBond<'a>>
}

static ENDPOINT_URL : Lazy<&str> = Lazy::new(|| "https://www.zakup.obligacjeskarbowe.pl/login.html");

async fn wait_for_dialog(driver: &WebDriver) -> Result<()> {
  let dialog = driver
    .query(By::ClassName("ui-dialog-title"))
    .with_text("Operacja w toku")
    .desc("Could not find loading dialog")
    .ignore_errors(true)
    //.wait(Duration::from_secs(1), Duration::from_secs(1))
    .first()
    .await;

  if let Ok(dialog) = dialog {
    dialog.wait_until()
      .error("Timed out waiting for loading dialog to disappear")
      .not_displayed()
      .await;
  }

  Ok(())
}

impl<'a> PKOBPAccount<'a> {
  pub fn new(selenium_hub_url: &'a str, login: &'a str, password: &'a str) -> Self {
    Self { selenium_hub_url, login, password, bonds: Vec::new(), cash: Money::from_major(0, PLN) }
  }

  async fn login(&mut self, driver: &WebDriver) -> Result<()> {
    let endpoint = ENDPOINT_URL.clone();
    tracing::debug!("Visiting page: {:?}", endpoint);
    driver.get(endpoint).await?;

    tracing::debug!("Filling login form");
    let username_input = driver.find_element(By::Id("username")).await?;
    let password_input = driver.find_element(By::Id("password")).await?;

    username_input.send_keys(self.login).await?;
    password_input.send_keys(self.password).await?;

    tracing::debug!("Clicking login");
    // Click the login button.
    let login_button = driver.find_element(By::Id("baton")).await?;
    login_button.click().await?;

    tracing::debug!("Waiting for sign in...");
    driver.query(By::Tag("a"))
      .with_text("Rachunek Rejestrowy")
      .first()
      .await?
      .wait_until()
      .error("Invalid login or password...")
      .displayed()
      .await?;

    tracing::debug!("Signed in! Fetching balance...");
    Ok(())
  }

  async fn adjust_pagination(&mut self, driver: &WebDriver) -> Result<()> {
    tracing::debug!("Adjusting pagination 50 rows");

    driver.query(By::ClassName("ui-paginator-rpp-options"))
      .first()
      .await?
      .select_by_value("50")
      .await?;


    wait_for_dialog(driver).await?;
    Ok(())
  }

  async fn next_page(&mut self, driver: &WebDriver) -> Result<bool> {
    let next_page_button = driver
      .query(By::Css(".ui-paginator-next:not(.ui-state-disabled)"))
      .wait(Duration::from_secs(1), Duration::from_secs(1))
      .first()
      .await;

    match next_page_button {
      Ok(button) => {
        tracing::debug!("Going to next page...");
        button.click().await?;
        wait_for_dialog(driver).await?;
        Ok(true)
      },
      Err(_) => {
        tracing::debug!("Finished by visiting last page;");
        Ok(false)
      }
    }
  }

  async fn pull_bonds(&mut self, driver: &WebDriver) -> Result<()> {
    let cash : PolishPeso = driver
      .query(By::XPath("//*[@id=\"stanRachunku\"]/span[2]"))
      .first()
      .await?
      .text()
      .await?
      .parse()?;
    self.cash = cash.into_inner();
    self.adjust_pagination(driver).await?;
    tracing::debug!("Current cash: {}", self.cash);

    loop {
      tracing::debug!("Visited bonds page, importing...");

      let table = driver.query(By::ClassName("ui-datatable")).first().await?;
      let rows = table.query(By::Css("tbody tr")).all().await?;

      for row in rows {
        let columns = row.query(By::Tag("td")).all().await?;

        let name_col = &columns[0];
        name_col.scroll_into_view().await?;

        tracing::debug!("Moving cursor to trigger tooltip...");
        driver.action_chain()
          .move_to_element_center(name_col)
          .click()
          .perform()
          .await?;
        tracing::debug!("Waiting for tooltip");

        sleep(Duration::from_millis(69)).await;

        let emission = name_col.text().await?;
        let bond_type : BondKind = join(emission.matches(char::is_alphabetic), "").try_into()?;
        let free : i32 = columns[1].text().await?.parse().unwrap();
        let blocked : i32 = columns[2].text().await?.parse().unwrap();
        let shares = free + blocked;

        let start_price : PolishPeso = columns[3].text().await?.parse()?; //  "3 000,00 PLN"
        let start_price = start_price.into_inner();
        let current_price : PolishPeso = columns[4].text().await?.parse()?; //  "3 000,00 PLN"
        let current_price = current_price.into_inner();
        let buyout_date : NaiveDate = columns[5].text().await?.parse()?; // "2024-12-07"

        let span_name = name_col
          .query(By::Tag("span"))
          .first()
          .await?;

        let tooltip_id = span_name.attr("aria-describedby").await?.ok_or(anyhow::anyhow!("Missing hash tooltip"))?;
        tracing::trace!("Selector id is: {:?}", tooltip_id);
        let tooltip = driver.query(By::Id(&tooltip_id)).desc("Tooltip with rates").first().await?;

        let tooltip = tooltip.text().await?;
        tracing::debug!("Tooltip text: {} for {}", tooltip, emission);

        let rates : PKOBPBondRates = tooltip.parse()?;
        let rates = rates.into_inner();

        tracing::trace!("Bond: {}", bond_type);
        tracing::trace!("  |- emission: {:?}", emission);
        tracing::trace!("  |- free: {:?}", free);
        tracing::trace!("  |- blocked: {:?}", blocked);
        tracing::trace!("  |- shares: {:?}", shares);
        tracing::trace!("  |- start_price: {}", start_price);
        tracing::trace!("  |- current_price: {}", current_price);
        tracing::trace!("  |- buyout_date: {:?}", buyout_date);
        tracing::trace!("  |- rates: {:?}", rates);

        let bond = PKOBPBond {
          account_id: self.login,
          kind: bond_type,
          emission,
          shares,
          start_price,
          current_price,
          buyout_date,
          rates
        };

        tracing::debug!("Bond: {:?}", bond);
        self.bonds.push(bond);
      };

      tracing::debug!("Found all bonds on page");

      if self.next_page(&driver).await? {
        continue;
      } else {
        break;
      }
    }

    Ok(())
  }

  async fn pull_balance(&mut self, driver: &WebDriver) -> Result<()> {
    let endpoint = "https://www.zakup.obligacjeskarbowe.pl/historiaDyspozycji.html";
    tracing::debug!("Visiting page: {:?}", endpoint);
    driver.get(endpoint).await?;

    driver.query(By::Tag("h3"))
      .with_text("Historia dyspozycji na Rachunku Rejestrowym")
      .first()
      .await?
      .wait_until()
      .error("Could not load history")
      .displayed()
      .await?;

    tracing::debug!("Filling date range");
    let from_date_input = driver.find_element(By::Id("datyHistorii:dataOd_input")).await?;
    from_date_input.clear().await?;
    from_date_input.send_keys("2000-01-01").await?;
    let button = driver.find_element(By::Id("datyHistorii:ok")).await?;
    button.click().await?;

    self.adjust_pagination(driver).await?;

    loop {
      tracing::debug!("Visited bonds page, importing...");

      let table = driver.query(By::ClassName("ui-datatable")).first().await?;
      let rows = table.query(By::Css("tbody tr")).all().await?;

      for row in rows {
        let columns = row.query(By::Tag("td")).all().await?;

        let date = columns[0].text().await?; // string can be empty
        let description = columns[1].text().await?;
        let emission = columns[2].text().await?;
        let price = &columns[5].text().await?;
        let note = columns[6].text().await?;

        tracing::debug!("-----");
        tracing::debug!(" - {:?}", date);
        tracing::debug!(" - {:?}", description);
        tracing::debug!(" - {:?}", emission);
        tracing::debug!(" - {:?}", price);
        tracing::debug!(" - {:?}", note);
      };

      tracing::debug!("Found all entries on page");

      if self.next_page(&driver).await? {
        continue;
      } else {
        break;
      }
    }

    Ok(())
  }

  pub async fn sync(&mut self) -> Result<()> {
    self.bonds.clear();

    tracing::debug!("Starting browser using selenium");
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(self.selenium_hub_url, caps).await?;

    if let Err(error) = self.login(&driver).await {
      tracing::error!("Could not login: {:?}", error);
      driver.quit().await?;
      return Err(error)
    }

    if let Err(error) = self.pull_bonds(&driver).await {
      tracing::error!("Could not sync bonds: {:?}", error);
      driver.quit().await?;
      return Err(error)
    }

    if let Err(error) = self.pull_balance(&driver).await {
      tracing::error!("Could not sync balance: {:?}", error);
      driver.quit().await?;
      return Err(error)
    }

    driver.quit().await?;
    Ok(())
  }
}
