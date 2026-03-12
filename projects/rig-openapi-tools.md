---
order: 1
name: rig-openapi-tools
slug: rig-openapi-tools
description: Turn any OpenAPI spec into LLM-callable tools for rig
github_url: https://github.com/skharchikov/rig-openapi-tools
crates_url: https://crates.io/crates/rig-openapi-tools
tags:
  - rust
  - openapi
  - llm
  - ai-agents
---

A Rust library that parses OpenAPI 3.0 specs and generates tool definitions that LLM agents can call directly via [rig](https://github.com/0xPlaygrounds/rig).

## Features

- **Zero Boilerplate**: Point at an OpenAPI spec and get callable tools — no manual wiring
- **Full OpenAPI 3.0 Support**: Path, query, header parameters, request bodies, and `$ref` resolution
- **Builder API**: Configure base URL, auth, custom HTTP client, and hidden context
- **Per-Request Context**: Inject user-specific parameters (user ID, session, etc.) without re-parsing the spec
- **Hidden Parameters**: Auto-inject values the LLM should never see in the tool schema

## Tech Stack

- **rig-core**: Agent framework for tool registration
- **openapiv3**: Spec parsing and validation
- **reqwest**: HTTP client with TLS support
- **serde**: JSON/YAML serialization

## Use Cases

Build AI agents that interact with any REST API by simply providing its OpenAPI spec — no hand-written tool definitions required. Ideal for customer support bots, internal tooling agents, and API orchestration.
