# Test Coverage Gap Report (bd-tests-foundation)

## Current coverage snapshot (Dec 2, 2025)
- Connectors: `tests/connector_{codex,cline,gemini,claude,opencode,amp}.rs`; since_ts/idx resequencing/dedupe partially covered; Claude still uses temp path helper but now exercised in watch_multi E2E; aider connector tests added; external_id collisions noted (Claude filename reuse).
- Search/Query: `tests/ranking.rs`, `tests/search_caching.rs`, `tests/search_wildcard_fallback.rs`, new FTS coverage (`tests/search_query.rs` via PinkPond) — wildcard, boolean, cache invalidation. Detail-find highlight still untested.
- UI/TUI: `tests/ui_{footer,help,hotkeys,snap}.rs`, `ui_components.rs`; pane count/ranking persistence and reset-state covered via recent work; still missing automated coverage for detail find (/ n/N), breadcrumbs, bulk queue/open, density controls.
- Storage: `tests/storage.rs` happy-path; migrations/append-only rollback still not covered.
- CLI/Robot: `tests/cli_robot.rs` expanded (robot-help/docs contract, reset-state flag), capabilities/introspect fixtures current; limited negative/error-path assertions.
- E2E: `e2e_index_tui.rs` smoke; `e2e_filters.rs` (agent/time/workspace filters) added; `watch_e2e.rs` now covers multi-connector, rapid changes, corrupt inputs; install e2e still narrow.
- Install scripts: checksum happy path only; no bad-checksum/DEST override coverage.
- Logging: `tests/logging.rs` basic; no span/key-event assertions yet.
- Benchmarks: present (index/search/cache/runtime), not enforced in CI.

## High-priority gaps (mapped to beads)
1) TST.1 Coverage inventory (in progress)
   - Deliver: updated module→test map (this doc), mock usage list, gap/fixture table (below).
2) TST.2 Unit: search/query + detail find (real fixtures)
   - Add coverage for wildcard fallback, cache shard eviction, agent/workspace filters, detail-find match counting and scroll targeting; assert cache/log stats.
3) TST.3 Unit: UI interactions
   - Headless ratatui tests for detail find (/ n/N), pane filter coexistence, breadcrumbs, bulk actions, focus toggles, tab cycling; verify status strings/title badges.
4) TST.4 Unit: connectors + storage (real edge fixtures)
   - since_ts routing, external_id dedupe, idx resequencing, timestamp parsing; append-only and migration guards; no mocks.
5) TST.5 E2E: CLI/TUI flows with rich logging
   - Robot/headless scripts covering search→detail find→bulk actions→filters; structured logs/traces; assert outputs.
6) TST.6 E2E: install/index/watch pipeline logging
   - install.sh/ps1 checksum good+bad, DEST override; index --full, watch-once targeted reindex; watch_state bump; detailed logs.
7) Logging assertions
   - Cross-cutting: span/key-event checks for connectors/indexer/search/watch; reusable util in tests/util.
8) Docs/help alignment
   - README/env knobs/help text kept in sync with new tests; add testing matrix section.

## Proposed test tasks (beads)
- bd-unit-connectors: fixtures + per-connector tests (see below).
- bd-unit-storage: Sqlite schema/version/transaction tests.
- bd-unit-indexer: full vs incremental vs append-only coverage.
- bd-unit-search: filter/highlight/pagination tests (detail-find, cache eviction, match_type aggregation).
- bd-unit-tui-components: snapshot tests for bar/pills/detail tabs.
- bd-e2e-index-tui-smoke: seed fixtures, run index --full, launch tui --once, assert logs.
- bd-e2e-watch-incremental: watch run + file touch, assert targeted reindex + watch_state bump (extended scenarios now partly covered; add delete/removal).
- bd-e2e-install-scripts: checksum pass/fail, DEST install.
- bd-logging-coverage: tracing span assertions.
- bd-ci-e2e-job: wire above into CI with timeouts.
- bd-docs-testing: README testing matrix + env knobs.

## Fixture plan
- Extend existing fixtures instead of mocks:
  - Add since_ts/append-only variants for each connector (Codex, Cline, Gemini, Claude, OpenCode, Amp).
  - Replace `mock-claude` temp paths with real fixture dir naming (partial via watch_e2e multi-connector).
- Add installer tar/zip + matching `.sha256` pairs for positive/negative checksum tests (local `file://`, <50KB).
- Provide mini watch playground under tests/fixtures for targeted reindex checks with watch_state.json expectations.
- Shared conversation fixtures for UI/detail-find tests (messages + snippets + raw metadata).

## Next immediate steps (TST.1 → downstream)
1) Draft connector fixture matrix (since_ts + dedupe + malformed) and UI detail-find conversation set; align with yln.4/yln.2.
2) Add tracing/log capture helper in `tests/util` (building on existing TestTracing) for span/key assertions (for TST.5/TST.6).
3) Add detail-find/search unit coverage (yln.2) and UI interactions (yln.3) using shared fixtures.
4) Expand install/watch e2e with checksum-fail and delete-detection cases (yln.6 / bd-e2e-install-scripts).
5) Add CLI negative/limit tests (66o/fwr) once coverage matrix finalized.
