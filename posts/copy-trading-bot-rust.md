---
title: "A Polymarket Copy Trader in Rust"
date: "2026-05-05"
slug: "copy_trading_bot_rust"
excerpt: "Notes from wiring up a Polymarket copy trader in Rust. Loops, dedup, sizing, and the boring glue that actually matters."
tags: ["rust", "tokio", "polymarket", "trading"]
---

I wanted a bot that watches sharp Polymarket traders and mirrors their entries. The idea was simple: if a top 10 trader buys, I buy a small piece of the same thing. Turns out most of the work is plumbing, not strategy.

Here is roughly how it ended up.

## The shape of it

Three loops on top of `tokio`, all kicked off from `main`:

* a copy trade loop that polls trader activity every few minutes
* a housekeeping loop that resolves bets after markets settle
* a Telegram command loop so I can `/follow <wallet>` from my phone

```rust
tokio::select! {
    _ = shutdown_signal() => { tracing::info!("shutting down"); }
    r = command_loop      => { tracing::error!("command loop exited: {:?}", r); }
    r = copy_trade_loop   => { tracing::error!("copy trade loop exited: {:?}", r); }
    r = housekeeping_loop => { tracing::error!("housekeeping loop exited: {:?}", r); }
}
```

`tokio::select!` over the join handles means if any loop dies, the bot exits and Docker restarts it. Cheap supervision.

## Wiring it up

Boot order is boring but important. Build the shared state once, then hand `Arc` clones to each spawned loop. No globals, no `OnceCell`, no surprises.

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    dotenvy::dotenv().ok();

    let cfg      = Arc::new(CopyTradingConfig::load()?);
    let pool     = PgPool::connect(&cfg.database_url).await?;
    let portfolio = Arc::new(PgPortfolio::new(pool).await?);
    portfolio.run_migrations().await?;

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;
    let monitor  = Arc::new(CopyTraderMonitor::new(http));
    let notifier = Arc::new(TelegramNotifier::new(&cfg.bot_token, &cfg.chat_id));

    let copy_trade_loop = {
        let portfolio = Arc::clone(&portfolio);
        let notifier  = Arc::clone(&notifier);
        let monitor   = Arc::clone(&monitor);
        let cfg       = Arc::clone(&cfg);
        tokio::spawn(async move {
            loop {
                if let Err(e) = copy_trade_cycle(&portfolio, &notifier, &monitor, &cfg).await {
                    tracing::error!(err = %e, "cycle failed");
                }
                tokio::time::sleep(Duration::from_secs(cfg.interval_secs)).await;
            }
        })
    };

    // ... housekeeping_loop and command_loop spawned the same way ...

    tokio::select! {
        _ = shutdown_signal()  => {}
        r = copy_trade_loop    => tracing::error!("copy_trade exited: {:?}", r),
    }
    Ok(())
}
```

Each loop owns its own `Arc` clones, so the borrow checker stays out of the way. The pattern repeats for every loop you bolt on later.

## Finding traders

Polymarket exposes a public leaderboard endpoint. Pull it for `DAY` / `WEEK` / `MONTH` / `ALL`, sort by PnL, format it for Telegram. That is it for discovery. No scraping, no auth.

```
GET https://data-api.polymarket.com/v1/leaderboard?timePeriod=WEEK&limit=10
```

Fields come back as a mix of strings and numbers depending on size, so the parser deserializes to `serde_json::Value` and coerces. Annoying but stable.

## Detecting trades

For each followed wallet, hit `/activity?user=…&type=TRADE&startTs=…` and diff against what you already saw. Two filters earn their keep:

1. **Stale trades.** Anything older than 5 minutes gets dropped. Price has moved, edge is gone.
2. **Dedup against Postgres.** `(wallet, condition_id, side, price)` as the seen key. Same call within a cycle will not double fire.

All wallets get polled in parallel via `futures_util::join_all`, then results processed sequentially so the dedup writes do not race.

```rust
let poll_futures = traders.iter().map(|t| async move {
    (t, self.poll_trader_activity(&t.proxy_wallet, since).await)
});
let results = join_all(poll_futures).await;
```

## Sizing the bet

Once a fresh BUY shows up, sanity checks before placing:

* skip if the current market price drifted more than 5pp from the trader's entry (you missed it)
* skip if I already have an open bet on the same market or event
* skip if the per trader bankroll is below the minimum

Sizing is quarter Kelly off a probability estimate of `entry_price + 0.05`, capped at 0.95. Conservative, but the whole point is the trader has the edge. I am along for a fraction of the ride.

```rust
let estimated_prob = (entry_price + 0.05).min(0.95);
let kelly = fractional_kelly(estimated_prob, entry_price, 0.25);
```

Each trader gets their own bankroll bucket (`copy:<wallet_short>`), so good ones grow and bad ones starve themselves out without me touching anything.

## Mirroring exits

When a followed trader sells, look up whether you have a matching open position on that market under their strategy bucket. If yes, exit at current price and broadcast PnL to Telegram. If no, ignore. They sold something you never copied.

That symmetry, copy entries and mirror exits, is the whole strategy.

## Crates that pulled their weight

Most of this project is glue. These crates kept the glue thin:

* **[`confique`](https://crates.io/crates/confique)** for config. Derive `Config`, annotate fields with `#[config(env = "...", default = ...)]`, and `Self::builder().env().load()` is the whole loader. No `std::env::var` boilerplate, no manual parsing, defaults live next to the field.

```rust
#[derive(Debug, Config)]
pub struct CopyTradingConfig {
    #[config(env = "DATABASE_URL")]
    pub database_url: String,
    #[config(env = "COPY_TRADE_INTERVAL_MINS", default = 1)]
    pub copy_trade_interval_mins: u64,
}
```

* **[`anyhow`](https://crates.io/crates/anyhow)** for error handling. `Result<T>` everywhere, `.context("...")` to add hops, `?` to propagate. No bespoke error enum until you need one.
* **[`dotenvy`](https://crates.io/crates/dotenvy)** to load `.env` in dev. One call (`dotenvy::dotenv().ok()`) at startup, ignored if missing.

## What I would skip if I started over

* **Do not pre cache the markets.** I tried. The data API is fast enough that an on demand `slug → market` lookup per signal is fine, and the cache invalidation was not worth it.
* **Bake dedup into Postgres, not memory.** A `UNIQUE` constraint on the seen trade tuple is one schema change vs an entire in memory layer to reason about.
* **Trace everything with `#[tracing::instrument]`.** When a trade goes weird at 2am, structured logs with `wallet`, `slug`, `price` fields are the difference between five minutes and an hour.

Repo: [skharchikov/polymarket-bot](https://github.com/skharchikov/polymarket-bot). The interesting bits live in [`copy-trading-bot/src/scanner/copy_trader.rs`](https://github.com/skharchikov/polymarket-bot/blob/master/copy-trading-bot/src/scanner/copy_trader.rs) and [`copy-trading-bot/src/cycles/copy_trade.rs`](https://github.com/skharchikov/polymarket-bot/blob/master/copy-trading-bot/src/cycles/copy_trade.rs). Boot wiring lives in [`copy-trading-bot/src/live.rs`](https://github.com/skharchikov/polymarket-bot/blob/master/copy-trading-bot/src/live.rs).

Bot is live on Telegram: [@Polymarket_rs_copy_trading_bot](https://t.me/Polymarket_rs_copy_trading_bot).
