# 0002: Local blocks with no GitHub tracker and no Kotlin wrapper

Date: 2026-09-23. Status: blocked, recorded locally.

## Question

Two gaps stop the plan's tracking and Kotlin gates, and neither can be
filed as a remote ticket: GitHub issues are disabled on
`metaphorics/modern-sicp`, and the Kotlin Gradle wrapper is absent from
the tree. Where do they go so no turn loses them?

## Findings

1. Issues disabled. `gh issue list --repo metaphorics/modern-sicp`
   answers "the `metaphorics/modern-sicp` repository has disabled
   issues". The plan's tracking (D6, D26: 25 tickets) and the wayfinder
   map's whole procedure (labels, map issue, tickets, `## Blocked by`
   edits) cannot run. Edition section units stay untracked until issues
   are enabled or the plan is amended to a file-based tracker.
2. Kotlin wrapper absent. `kotlin/justfile` fmt, lint, test, and
   scaffold recipes call `./gradlew`, but `gradlew`, `gradlew.bat`,
   and `gradle/wrapper/` are not on disk; the system gradle is 4.4.1,
   outside the D16 pin of 9.7.0. No Kotlin justfile recipe is runnable
   here. The wrapper arrives via `just setup-kotlin` on the Kotlin
   track; the justfile must not be retargeted at the system binary.

## Decision

Both stay here as local blocks, not remote tickets. The first section
unit of each blocked track re-checks its line before starting work and
either clears it or carries it forward in the next decision note.
