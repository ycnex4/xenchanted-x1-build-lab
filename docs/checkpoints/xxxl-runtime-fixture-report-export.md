# XXXL Runtime Fixture Report Export Checkpoint

Stage XXXL Program v1 now has a deterministic runtime fixture report/export layer.

New files:

- `src/xxxl/runtime-fixture-report-export.ts`
- `tests/xxxl/runtime-fixture-report-export.test.ts`
- `docs/xxxl/xxxl-runtime-fixture-report-export.md`

The export summarizes:

- fixture count
- unique execution vector count
- fixture ids
- vector ids
- CPI committed vectors
- CPI skipped vectors
- supply audit OK vectors
- expected rejection vectors
- route-aware success vectors
- per-fixture report details
- canonical JSON
- markdown summary

Key properties:

- default report covers all runtime dry-run fixtures
- successful Ethereum and Avalanche route-aware vectors are visible
- rejected vectors are reported as CPI skipped
- expected rejections are not treated as report failures
- canonical JSON is deterministic
- markdown summary is deterministic
- report validation detects bad canonical JSON, bad markdown, and not-ok reports

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 82 files / 613 tests passing
- Build: passing
