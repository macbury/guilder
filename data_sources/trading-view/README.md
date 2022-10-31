#TODO:

- client that manages multiple connections
- Search for assets(use screener? or original search) - requwest
- fetch price data for range
- pull data for observed stocks
- observe manager
  - spawn new connection for each 30 tickers


# Run

```
export RUST_LOG=trading_view=trace
cargo run --example observe
```
