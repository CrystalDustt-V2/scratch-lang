# scratch-lang Agent Context & Continuity System

## Purpose
This directory exists specifically for AI agents and long-running development continuity across sessions.
It preserves project status, architectural decisions, roadmaps, known issues, test statuses, and session logs in version control so that any agent can immediately orient themselves and continue work without context loss or hallucination.

## Rules for Agents
1. **Never ignore this directory**: `.agent-context/` must remain tracked in git.
2. **Read order on session start**:
   - `.agent-context/CURRENT_STATE.md` (compact snapshot of current state, blockers, and next steps)
   - `.agent-context/PROJECT_STATUS.md` (feature status board)
   - `.agent-context/ROADMAP.md` (milestones and progression)
   - `.agent-context/KNOWN_ISSUES.md` (current bugs and quirks)
3. **Session close protocol**:
   - Update `CURRENT_STATE.md`
   - Update `PROJECT_STATUS.md`
   - Update `ROADMAP.md` if milestones advance
   - Update `CHANGELOG.md` for meaningful changes
   - Update `KNOWN_ISSUES.md` if new issues arise
   - Update `TEST_STATUS.md` with latest test outcomes
   - Add a session entry under `.agent-context/sessions/`
   - Add/update `.agent-context/reports/latest.md` if a major milestone was reached.
4. **Source of truth hierarchy**:
   `Actual implementation > Current documented architecture > Historical session notes > Old plans`

## Authoritative Files
- `CURRENT_STATE.md`: Quick-start snapshot. Must always be up-to-date.
- `PROJECT_STATUS.md`: Granular status of language, compiler, runtime, and tooling features.
- `ARCHITECTURE.md`: Master architectural blueprint and subsystem boundaries.
- `DECISIONS.md`: Architectural Decision Records (ADRs) with rationale and consequences.
- `ROADMAP.md`: High-level phases and sequence of development.
- `TEST_STATUS.md`: Test coverage and verification metrics.
