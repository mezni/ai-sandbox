# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Level 3 — Prompt Versioning:
  - `PromptRepository` that resolves the active version from `prompts/registry.json`
    and loads the corresponding template.
  - Versioned prompt templates under `prompts/v1/` and `prompts/v2/`.
  - `prompts/registry.json` (active version) and `prompts/catalog.json` (metadata).
- `test_active_version_loaded` covering active-version resolution.

### Changed

- `main.py` now loads the active prompt version via `PromptRepository`.

## [0.2.0] - 2026-10-15

### Added

- Level 2 — Prompt Separation:
  - `PromptRenderer` to substitute variables into templates.
  - Prompt template moved out of application code into `prompts/`.
- Tests for prompt loading and rendering (`tests/test_prompt_renderer.py`).

### Changed

- `main.py` loads and renders the prompt instead of hardcoding it.

## [0.1.0] - 2026-10-15

### Added

- Level 1 — Monolithic AI solution designer script.
- `arch-agent` console entry point.
- Project scaffold: `uv`, `pydantic`, `pytest`, `ruff`.
- README with vision, roadmap, and status tracker.
- `docs/followup.md` design notes.

[Unreleased]: https://github.com/mezni/arch-agent/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/mezni/arch-agent/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/mezni/arch-agent/releases/tag/v0.1.0
