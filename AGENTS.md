# AGENTS: Repository Operating Rules

This repository contains the extracted `myth-matter` material domain crate.

## Standards

- Keep material taxonomy and recipe logic deterministic and explicit.
- Treat public recipe, taxonomy, and packed-field structures as data contracts when consumed across crates.
- Add or update tests for behavior changes.
- Keep repository hygiene clean; do not leave generated or temporary artifacts behind.

## Validation

Run before handoff:

```bash
cargo check
cargo test
```
