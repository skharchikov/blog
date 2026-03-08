---
order: 1
name: Polymarket Signal Bot
slug: polymarket-bot
description: Automated prediction market bot with multi-agent LLM consensus and multi-strategy Kelly sizing
github_url: https://github.com/skharchikov/polymarket-bot
tags:
  - rust
  - tokio
  - openai
  - postgres
  - docker
---

An automated trading bot that finds alpha on [Polymarket](https://polymarket.com) by matching breaking news to prediction markets that haven't priced it in yet.

## How It Works

The bot runs two async loops. A **news scan** (every 10 min) fetches headlines from Google News RSS, Reddit, CoinDesk, Reuters, and Polymarket trending, matches them to active markets by keyword overlap, then uses Bayesian updating with multiple LLM agents to assess whether the news shifts the true probability away from the current market price.

A **housekeeping loop** (every 30 min) resolves settled bets, updates the calibration curve, and sends daily PnL reports to Telegram.

## Bayesian Multi-Agent Consensus

Instead of trusting a single LLM call, the bot queries 2-3 agents with different roles. Each agent estimates a **likelihood ratio** — how much more likely is this news in worlds where YES happens vs NO:

- **Skeptic** (temp 0.1) — assumes the market is efficient, LRs biased toward 1.0
- **Catalyst** (temp 0.3) — hunts for fresh catalysts, willing to give strong LRs
- **BaseRate** (temp 0.2) — ignores narratives, anchors LRs to historical base rates

Starting from the market price as the Bayesian prior, each agent's confidence-dampened LR updates the posterior sequentially via Bayes' rule: `posterior_odds = prior_odds × LR₁^c₁ × LR₂^c₂ × ...`. When agents disagree on direction (some LR > 1, some < 1), an agreement penalty crushes confidence, preventing bets on ambiguous signals.

## Calibration Tracking

Every LLM estimate is logged to Postgres. Once enough markets have resolved, a 10-bin calibration curve (with Laplace smoothing) corrects systematic biases — if the LLM says "70%" but outcomes only happen 55% of the time, future 70% estimates get shifted down automatically.

## Multi-Strategy Profiles

Three strategies run simultaneously with independent bankrolls:

| Strategy | Kelly Fraction | Min Edge | Min Confidence |
|---|---|---|---|
| Aggressive | 50% | 5% | 40% |
| Balanced | 25% | 8% | 50% |
| Conservative | 10% | 12% | 65% |

Strategies share the expensive LLM scan results but evaluate signals independently — the aggressive profile might take a bet that the conservative one rejects. Each strategy has its own bankroll, daily signal limits, and bet history.

## Tech Stack

- **Rust** + **Tokio**: async runtime with dual-loop architecture
- **rig-core**: LLM agent framework (OpenAI GPT-4o)
- **SQLx** + **PostgreSQL**: persistence, migrations, calibration data
- **reqwest**: HTTP client for Polymarket Gamma/CLOB APIs and news sources
- **Telegram Bot API**: real-time alerts and daily reports ([@Polymarket_rs_bot](https://t.me/Polymarket_rs_bot))
- **Docker**: Alpine musl build, scratch final image (~3.5 MB), UPX compressed
- **mimalloc**: memory allocator replacing musl's slow malloc
- **GitHub Actions**: CI/CD with auto-deploy to Hetzner

## Backtesting

Includes a backtesting engine that runs 4 price-based strategies (SMA crossover, mean reversion, trend following, contrarian extremes) on historical closed markets with slippage/fee simulation. Reports ROI, Sharpe ratio, Brier score, and max drawdown.
