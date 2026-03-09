---
order: 1
name: Polymarket Signal Bot
slug: polymarket-bot
description: Automated prediction market bot with XGBoost ML model, Bayesian anchoring, continuous self-learning, and multi-strategy Kelly sizing
github_url: https://github.com/skharchikov/polymarket-bot
tags:
  - rust
  - tokio
  - xgboost
  - openai
  - postgres
  - docker
---

An automated trading bot that finds alpha on [Polymarket](https://polymarket.com) by combining an XGBoost ensemble model with Bayesian probability anchoring. The model continuously retrains on its own resolved bets — both wins and losses — getting smarter over time.

## ML-First Pipeline

The bot scores all eligible markets using a stacking ensemble (XGBoost + logistic regression + random forest) trained on 15+ features: price momentum, volatility, volume trends, order book depth, and time to expiry.

Raw model predictions are **Bayesian-anchored** to the market price to prevent overconfidence. The model's likelihood ratio is dampened by its own confidence score (`LR^confidence`), so low-confidence predictions stay close to market price while high-confidence ones can diverge more.

A separate WebSocket connection monitors real-time price movements and triggers instant reassessment through the model, reacting faster than the 10-minute scan cycle.

When the ML model isn't available, the bot falls back to a multi-agent LLM consensus system where 2-3 agents (Skeptic, Catalyst, BaseRate) each estimate likelihood ratios that update the posterior sequentially via Bayes' rule.

## Continuous Self-Learning

The model retrains every 24 hours on ~1000 resolved Polymarket markets plus its own resolved bets (weighted 3x for higher fidelity). Both wins and losses enter training — losses are arguably more valuable, teaching the model where its edge estimates were wrong.

Every prediction is logged with a Brier score tracking system that measures model accuracy vs market baseline. A daily report shows whether the model adds value over simply trusting market prices.

## Risk Management

- **Stop-loss** (30%): automatically exits positions when unrealized loss exceeds threshold
- **Expiry exit**: exits underwater positions 2 days before market resolution
- **Terminal risk scaling**: reduces position size as markets approach expiry
- **Per-strategy bankroll isolation**: losses in one strategy don't affect others

## Multi-Strategy Profiles

Three strategies run simultaneously with independent bankrolls, sharing ML results but evaluating signals against different thresholds:

| Strategy | Kelly Fraction | Min Edge | Min Confidence |
|---|---|---|---|
| Aggressive 🔥 | 50% | 5% | 40% |
| Balanced ⚖️ | 25% | 8% | 50% |
| Conservative 🛡️ | 15% | 8% | 50% |

## Telegram Interface

Real-time notifications for every bet placement (with outcome side, stake, strategy), resolution, stop-loss exit, and daily performance reports. Interactive commands: `/stats` (per-strategy breakdown), `/open` (current positions), `/brier` (model accuracy), `/health` (uptime and scan metrics).

## Tech Stack

- **Rust** + **Tokio**: async runtime with multi-loop architecture (news scan, housekeeping, WebSocket, Telegram commands, heartbeat)
- **Python sidecar**: XGBoost/sklearn stacking ensemble, served over HTTP, auto-reloaded on retrain
- **SQLx** + **PostgreSQL**: persistence, migrations, prediction logging, calibration data
- **reqwest**: Polymarket Gamma/CLOB APIs, news sources (Google News RSS, Reddit, Reuters, CoinDesk)
- **Telegram Bot API**: alerts, commands, daily reports
- **Docker Compose**: 4-service stack (postgres, trainer, model-server, bot), ~3.5 MB bot binary
- **GitHub Actions**: CI/CD with auto-deploy to Hetzner
