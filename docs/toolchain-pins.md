# Toolchain pins

Every pin was read at its release channel on 2026-09-22. Rule: latest stable LTS where the project runs an LTS track, else latest stable; a compatibility window or a user decision overrides and is named. The plan (`docs/plan/technical-modern-sicp-editions.md`) holds the decision register; this file is the pin table the build files must match.

## Rust

|Entry|Pin|Released|Note|Source|
|---|---|---|---|---|
|rustc, cargo|1.98.1 stable|2026-09-03|No LTS track; `rust-toolchain.toml` pins the channel with `rustfmt` and `clippy`|https://doc.rust-lang.org/stable/releases.html|
|edition|2024|stable since 1.85.0|`rust-version = "1.98"`|https://doc.rust-lang.org/cargo/reference/workspaces.html|
|cargo-nextest|0.9.146|2026-09-21|Installed with `cargo install cargo-nextest --locked --version 0.9.146`|https://crates.io/crates/cargo-nextest|
|thiserror|2.0.20|2026-08-08|Library error types|https://crates.io/crates/thiserror|
|anyhow|1.0.104|2026-07-18|Binaries only|https://crates.io/crates/anyhow|
|proptest|1.11.0|2026-03-24|Property tests|https://crates.io/crates/proptest|
|insta|1.48.0|2026-06-11|Snapshots of interpreter transcripts and machine traces|https://crates.io/crates/insta|
|num-bigint, num-rational, num-traits|dropped|last releases 2024|Stale beyond the 12-month rule and not needed (D14)|https://crates.io/crates/num-bigint|

## OCaml

|Entry|Pin|Released|Note|Source|
|---|---|---|---|---|
|OCaml|5.5.1|2026-09-05|Current stable; `opam switch create 5.5.1`|https://ocaml.org/releases|
|opam|2.6.0|2026-09-17||https://opam.ocaml.org/|
|dune|3.24.2|2026-08-03|`(lang dune 3.24)`|https://opam.ocaml.org/packages/dune/|
|base, core|v0.17.3, v0.17.2|2025-06-13, 2026-03-26|Appendix only|https://opam.ocaml.org/packages/base/|
|ppx_jane, ppx_expect, ppx_inline_test|v0.17.0, v0.17.3, v0.17.1|2024 to 2025|Appendix tests|https://opam.ocaml.org/packages/ppx_jane/|
|alcotest|1.9.1|2025-10-01|Main-text tests|https://opam.ocaml.org/packages/alcotest/|
|qcheck-core|0.91|2025-12-28|Property tests|https://opam.ocaml.org/packages/qcheck-core/|
|ocamlformat|0.29.0|2026-03-17|`profile = janestreet`|https://opam.ocaml.org/packages/ocamlformat/|
|odoc|3.2.1|2026-05-12|Requires OCaml below 5.6|https://opam.ocaml.org/packages/odoc/|
|ocaml-lsp-server|1.27.0|2026-06-23||https://opam.ocaml.org/packages/ocaml-lsp-server/|
|zarith|dropped|last release 2024-07-15|Stale and not needed (D17)|https://opam.ocaml.org/packages/zarith/|

## TypeScript

|Entry|Pin|Released|Note|Source|
|---|---|---|---|---|
|typescript|7.0.2|2026-07-08|GA|https://registry.npmjs.org/typescript|
|effect|4.0.0-rc.117|2026-09-21|Release candidate pinned by user decision (D13)|https://registry.npmjs.org/effect|
|@effect/vitest|4.0.0-rc.117|2026-09-21|Peers `effect ^4.0.0-rc.117` and `vitest >=5.0.0 <6.0.0`|https://registry.npmjs.org/@effect/vitest|
|vitest|5.0.1|2026-09-15|Engines `^22.12.0\|\|^24.0.0\|\|>=26.0.0`|https://registry.npmjs.org/vitest|
|@effect/platform-node|4.0.0-rc.117|2026-09-21|Pinned exactly like `effect`|https://registry.npmjs.org/@effect/platform-node|
|@biomejs/biome|2.5.14|2026-09-16|Lint and format|https://registry.npmjs.org/@biomejs/biome|
|Node.js|24.21.0 Active LTS|2026-09-08, EOL 2028-04-30|`.node-version` and `engines.node`|https://endoflife.date/api/v1/products/nodejs/|
|pnpm|12.5.1|2026-09-18|`packageManager` field|https://registry.npmjs.org/pnpm|
|@types/node|resolved at A2|resolved at A2|Highest `24.x`|https://registry.npmjs.org/@types/node|
|fast-check|resolved at A2|resolved at A2|Property tests|https://registry.npmjs.org/fast-check|

## Kotlin

|Entry|Pin|Released|Note|Source|
|---|---|---|---|---|
|Kotlin|2.4.20|2026-09-07|K2|https://kotlinlang.org/docs/releases.html|
|JDK|25 LTS (Temurin 25.0.4.1+1)|2026-08-19|`jvmToolchain(25)` with the foojay resolver convention|https://endoflife.date/api/v1/products/eclipse-temurin/|
|Gradle|9.7.0|2026-08|Inside the Kotlin Gradle plugin's tested window (D16)|https://docs.gradle.org/9.7.1/userguide/compatibility.html|
|arrow-core, arrow-fx-coroutines|2.2.3|2026-06-04||https://central.sonatype.com/artifact/io.arrow-kt/arrow-core|
|kotlinx.coroutines|1.11.0|2026-05-07||https://central.sonatype.com/artifact/org.jetbrains.kotlinx/kotlinx-coroutines-core|
|kotlinx.collections.immutable|0.5.2|2026-08-28|Persistent lists and maps|https://central.sonatype.com/artifact/org.jetbrains.kotlinx/kotlinx-collections-immutable|
|Kotest|6.2.5|2026-09-10|`kotest-runner-junit5`, assertions, property testing|https://central.sonatype.com/artifact/io.kotest/kotest-runner-junit5|
|ktlint|1.8.0 with ktlint-gradle 14.2.0|2025-11-14, 2026-03-12|Style gate|https://plugins.gradle.org/plugin/org.jlleitschuh.gradle.ktlint|
|detekt|dropped|1.23.8 from 2025-02-21|Stale beyond the 12-month rule (D15)|https://github.com/detekt/detekt/releases|

## Book build

|Entry|Pin|Released|Note|Source|
|---|---|---|---|---|
|Texinfo|7.3|2026-03-02|Built from `ftp.gnu.org/gnu/texinfo/texinfo-7.3.tar.xz` into `~/.local` (CI: `/opt/texinfo`)|https://lists.gnu.org/archive/html/info-gnu/2026-03/msg00000.html|
|TeX Live|2025 (installed)||pdfTeX for `texi2pdf`; tex4ht for `HTML_MATH=t4h`|https://tug.org/texlive/|
|Pygments|resolved at A2||Lexers `rust`, `ocaml`, `typescript`, `kotlin`, `scheme`|https://pygments.org/|
|librsvg (`rsvg-convert`)|resolved at A2||SVG to PDF for the PDF build's figures|https://gitlab.gnome.org/GNOME/librsvg|
|Python, uv|3.14.7, 0.12.17 (installed)||`tools/` scripts (D22)|https://docs.astral.sh/uv/|
|epubcheck|resolved at A2||Mandatory in `just books` and in CI|https://github.com/w3c/epubcheck|
|GitHub Actions|commit SHAs resolved at A5||Each action pinned to a commit SHA with its tag beside it (D34)|https://github.com/actions|

## Settled at A3

|Variable|Value|Reason|
|---|---|---|
|`T4H_MATH_CONVERSION`|settled at A3|D36|
|`T4H_TEX_CONVERSION`|settled at A3|D36|

## GitHub Actions pins

Resolved at A5.

## CI durations

Recorded at A5 after the first two dispatches.
