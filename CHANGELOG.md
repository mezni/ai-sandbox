# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-10-16

### Added

- Level 5 — Rule-Based Evaluation:
  - `ProposalEvaluator` that scores an `ArchitectureProposal` on section
    completeness and returns an `EvaluationReport`.
  - `EvaluationReport` domain model (`score`, `passed`, `findings`).
- Tests for the evaluator (`tests/test_evaluator.py`).
- `docs/followup.md` notes for Step 5 (rule-based evaluation).

### Changed

- Fixed `tests/test_evaluator.py` import path (`ai_designer` → `arch_agent`).

## [0.4.0] - 2026-10-16

### Added

- Level 4 — Structured Outputs:
  - `ProposalParser` that parses raw model responses into an
    `ArchitectureProposal`.
  - `ArchitectureProposal` and `Idea` domain models.
- Tests for the proposal parser (`tests/test_parser.py`).
- `Makefile` with `test`, `lint`, and `format` targets.

### Changed

- Fixed `parser.py` import path (`ai_designer` → `arch_agent`).
- Reformatted existing sources with `ruff format`.

## [0.3.0] - 2026-10-16

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

[Unreleased]: https://github.com/mezni/arch-agent/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/mezni/arch-agent/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/mezni/arch-agent/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/mezni/arch-agent/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/mezni/arch-agent/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/mezni/arch-agent/releases/tag/v0.1.0
