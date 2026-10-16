# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Level 2 — Prompt Separation:
  - `PromptLoader` to read prompt templates from `prompts/`.
  - `PromptRenderer` to substitute variables into templates.
  - `prompts/solution_designer.txt` prompt template.
- Tests for `PromptRenderer` (`tests/test_prompt_renderer.py`).

### Changed

- `main.py` now loads and renders the prompt instead of hardcoding it.

## [0.1.0] - 2026-10-15

### Added

- Level 1 — Monolithic AI solution designer script.
- `arch-agent` console entry point.
- Project scaffold: `uv`, `pydantic`, `pytest`, `ruff`.
- README with vision, roadmap, and status tracker.
- `docs/followup.md` design notes.

[Unreleased]: https://github.com/mezni/arch-agent/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mezni/arch-agent/releases/tag/v0.1.0
