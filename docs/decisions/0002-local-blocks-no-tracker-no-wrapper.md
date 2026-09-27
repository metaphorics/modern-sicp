# 0002: Local blocks with no GitHub tracker and no Kotlin wrapper

Date: 2026-09-23. Status: finding 1 cleared this day (issues enabled, A6 filed); finding 2 blocked.

## Question

Two gaps stop the plan's tracking and Kotlin gates, and neither can be
filed as a remote ticket: GitHub issues are disabled on
`metaphorics/modern-sicp`, and the Kotlin Gradle wrapper is absent from
the tree. Where do they go so no turn loses them?
1. Issues disabled: CLEARED 2026-09-23. The user authorized the tracker
   decision; issues were enabled (`gh repo edit --enable-issues`) and A6 ran:
   five labels, map issue #1, prototype #2, Chapter 0 tickets #3 to #6 with
   `Blocked by` wired to #2. V5 label counts verified (map 1, task 4,
   prototype 1, grilling 0).

2. Kotlin wrapper absent. `kotlin/justfile` fmt, lint, test, and
   scaffold recipes call `./gradlew`, but `gradlew`, `gradlew.bat`,
   and `gradle/wrapper/` are not on disk; the system gradle is 4.4.1,
   outside the D16 pin of 9.7.0. No Kotlin justfile recipe is runnable
   here. The wrapper arrives via `just setup-kotlin` on the Kotlin
   track; the justfile must not be retargeted at the system binary.

Finding 1 is closed on the tracker itself; finding 2 stays here. The
first Kotlin unit re-checks it before starting work and either clears
it or carries it forward in the next decision note.
