# About

Simple software for scraping and tracking bonds from `https://www.zakup.obligacjeskarbowe.pl/`

# Development
```
psql --host localhost --port 19371 --user postgres postgres
CREATE DATABASE guilder_production;
CREATE USER guilder WITH ENCRYPTED PASSWORD 'guilder';
GRANT all privileges ON database guilder_production TO guilder;
sea migrate up
cargo run -- register --username=admin --password=admin1234

docker build . -t guilder
docker-compose -f docker-compose.production.yaml up
```
charts:

https://github.com/react-financial/react-financial-charts
https://nivo.rocks/funnel/

https://mysteryfinanse.pl/co-to-jest-xirr-czyli-jak-obliczyc-stope-zwrotu-z-regularnych-inwestycji/
https://github.com/rsuite/rsuite
https://github.com/warp-tech/warpgate/blob/a97d337913713c69b06ac15acdde0978ecfe29b0/warpgate-db-migrations/src/m00001_create_ticket.rs
https://github.com/SeaQL/sea-query/blob/cc928ac014eae6e9be9339402de0be9767ac9f23/examples/sqlx_sqlite/src/main.rs#L182
https://stackoverflow.com/questions/149055/how-to-format-numbers-as-currency-strings
https://corporatefinanceinstitute.com/resources/knowledge/accounting/


# Libs

- https://crates.io/crates/finql
- https://crates.io/crates/gurufocus_api
- https://crates.io/crates/investments

# Dev

cargo install systemfd
cargo install cargo-watch

# Build

docker run --rm -it guilder
docker build . -t guilder
