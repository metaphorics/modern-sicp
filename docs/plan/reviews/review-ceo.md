# CEO review (advocate agent) — raw output

{
  "confidence": 0.75,
  "findings": [
    {
      "body": "| Principle | Verdict | Reason |\n| --- | --- | --- |\n| clarity | amend | The plan states D12 lockstep and L units spanning sessions but never defines who unblocks a stalled section. |\n| impact | amend | No reader value ships until Phase H, so a stall in chapter 3 leaves 92 units of sunk work with nothing published. |\n| audience | amend | CC BY-SA on code forces share alike on copied functions, which deters the commercial readers the primers target. |\n| risk | amend | Effect rc plus TS 7 plus t4h plus four toolchains are coupled in one books job and one lockstep order. |\n| sequencing | amend | Lockstep by section multiplies rework when a shared Value, table, or corpus decision changes mid chapter. |\n| reversibility | accept | The split is lossless with a parity gate, demolition stays in git history, and the corpus is versioned. |",
      "confidence": 0.8,
      "file_path": "docs/plan/technical-modern-sicp-editions.md",
      "line_end": 60,
      "line_start": 30,
      "priority": 1,
      "title": "Decision register"
    },
    {
      "body": "1. Publication order (plan sections: Work breakdown, Approach, Wayfinder map and tickets). Failure: the programme stalls in Phase E on hard spots 3.3 and 3.5 with zero published pages, then Phase H discovers Pages is still disabled (Repository facts reports a 404) and the cover path is unknown. Add incremental publication. Paste this, replacing the Phase H row:\n```\n| H Publication | Per chapter-edition: HTML on GitHub Pages from the prototype onward; EPUB 3 per chapter-edition; PDF only at programme end | A7 publishes the 1.1 prototype for all four editions to Pages; each chapter close publishes that chapter; front matter lands per edition at its chapter 1 close |\n```\n2. Ordering (plan sections: Decision register D12, Work breakdown order and dependencies). Failure: a shared OpTable key change in 2.4 forces identical rework in four parallel section units, and one broken Effect rc blocks Rust, OCaml, and Kotlin in the same section. Replace D12 with:\n```\n| D12 | Authoring proceeds edition-leading: Rust leads each chapter by one section, the other three editions follow, and a short sync note closes each chapter before the next begins; a broken edition never blocks the other three |\n```\n3. Effect isolation (plan sections: Edition design TypeScript, Assumptions and contingencies). Failure: rc.117 renames one function and every TypeScript unit breaks at once, while the platform-node rc dist-tag may not exist and examples silently fall back to runPromise. Add this text to the TypeScript digest:\n```\nChapters 1 and 2 import no Effect module. Effect enters at 3.1 behind one `EffectOps` adapter module per package. If the pinned rc renames an API, only the adapter and the companion change in the same commit. If no matching platform-node rc exists, units use Effect.runPromise and record that choice in the ticket.\n```\n4. Math path (plan sections: Text pipeline and demolition step 5, Verification V2, Quality gates and CI). Failure: one tex4ht failure on a @math block reds the single books job for all four editions, and the machine carries Texinfo 7.2 while the plan needs 7.3. Add this text to step 5:\n```\nA3 proves HTML_MATH=t4h on the unadapted tree first. The books job is split into books-html-epub (blocking) and books-pdf (non-blocking until Phase H). PDF builds defer to Phase H. A t4h failure rewrites only that @math block in the documented Texinfo math subset.\n```\n5. Toolchain gap (plan sections: Quality gates and CI, Repository facts, Approach step 3). Failure: the local machine has OCaml 5.4.0, Node 26, no Gradle, and Texinfo 7.2, so no contributor can run the pinned gates as written and CI installs five toolchains cold on every run. Add this text to Approach step 3:\n```\nA2 ships tools/bootstrap.sh plus per-language just recipes that install Texinfo 7.3, the 5.5.1 opam switch, Node 24.21.0, and Gradle 9.7.0 with caches. CI records exact action versions in docs/toolchain-pins.md at scaffold time.\n```\n6. Ticket granularity (plan sections: Wayfinder map and tickets, Work breakdown unit of work). Failure: about 92 section-edition tickets with hand-wired Blocked by numbers go stale within one chapter, and L units spanning 2 to 3 sessions reload full context each session under one ticket. Replace the unit rule with:\n```\nUnit of tracking is one chapter of one edition (23 tickets), each with a section checklist. L sections split into named session slices inside the same ticket. Only the prototype, the four Chapter 0 tickets, and the grilling tickets are created at A6.\n```\n7. License posture (plan sections: License and attribution, Decision register D8). Failure: a reader who copies one example function into a proprietary codebase must license that codebase CC BY-SA or GPLv3, so commercial teams read but never reuse, and per-file adaptation headers churn every listing. Replace the D8 scope map with:\n```\ntext, book trees, figures, and statements stay CC BY-SA 4.0. examples, exercises, and solutions move to GPL-3.0-only under the Section 3(b)(1) compatibility rule. tools stays MIT. Headers: code files carry SPDX-License-Identifier: GPL-3.0-only plus one adapted-from line; prose files carry no headers; every edition README points to NOTICE.md.\n```\n8. Scope and bignum conflict (plan sections: Exercise policy Map overrides, Work breakdown). Failure: the chapter 2 map names bigint and Z for 2.5 while D14 and D17 forbid them, so four implementers invent four different bounds, and uncapped tailored additions plus property tests on all 356 exercises double the 92 units. Add this text to Exercise policy:\n```\n2.5 uses u128 in Rust and 63-bit int in OCaml with the overflow bound stated in the statement; TypeScript uses BigInt and Kotlin uses BigInteger. Tailored additions are capped at one per hard section. Deferred without touching settled choices: PDF builds, EPUB cover post-processing, and any second addition per section.\n```",
      "confidence": 0.75,
      "file_path": "docs/plan/technical-modern-sicp-editions.md",
      "line_end": 220,
      "line_start": 100,
      "priority": 0,
      "title": "Amendments"
    },
    {
      "body": "1. Publish the section 1.1 prototype for all four editions to GitHub Pages in Phase A; acceptance: a public URL serves HTML with 8 anchored exercises per edition.\n2. Prove HTML_MATH=t4h on the unadapted tree in Phase A; acceptance: parity check prints sections=22 exercises=356 figures=91 undefined_refs=0 and 1.1.7 HTML holds a math element.\n3. Add the TypeScript EffectOps adapter and keep chapters 1 and 2 Effect free in Phase B; acceptance: pnpm gates pass with no Effect import in chapters 1 and 2.\n4. Move examples, exercises, and solutions to GPL-3.0-only and rewrite headers in Phase A; acceptance: LICENSE scope map, NOTICE.md, and headers match the new rule.\n5. Reticket from section-edition tickets to chapter-edition tickets with checklists in Phase A; acceptance: gh issue lists show 23 chapter tickets plus prototype plus grilling with resolving Blocked by links.\n6. Split the books CI job into blocking html-epub and non-blocking pdf in Phase A; acceptance: a pdf failure leaves html-epub green.\n7. Fix the 2.5 bound override and cap tailored additions at one per hard section in Phase A; acceptance: exercise_map_check passes with stated bounds and no second addition.\n8. Rewrite D12 to edition-leading with a chapter sync note in Phase A; acceptance: Work breakdown names the lead edition and the sync step per chapter.",
      "confidence": 0.7,
      "file_path": "docs/plan/technical-modern-sicp-editions.md",
      "line_end": 260,
      "line_start": 200,
      "priority": 1,
      "title": "Tasks"
    },
    {
      "body": "- What public host and path will serve the four editions: the default github.io project site or a custom domain?\n- Will the EPUB cover be set through documentinfo or through tools post-processing on the EPUB zip?\n- What PDF typography and fonts will the editions use for code and math?",
      "confidence": 0.7,
      "file_path": "docs/plan/technical-modern-sicp-editions.md",
      "line_end": 353,
      "line_start": 320,
      "priority": 2,
      "title": "Open questions"
    }
  ],
  "strengths": [
    "The lossless split plus parity gate (22 sections, 356 anchors, 91 figures, zero undefined refs) makes demolition reversible.",
    "One to one section and exercise numbering with a shared conformance corpus keeps four editions comparable.",
    "The 1.1 prototype through the full pipeline plus per-unit gates tests text, code, and books together before scale."
  ],
  "summary": "The programme ships nothing until the end, couples four editions to one fragile Effect rc and one math path, and prices code reuse out of reach. Edition-leading order with incremental publication, an isolated Effect surface, a split books gate, chapter tickets, and GPL code defuses each failure.",
  "verdict": "block-must-fix"
}

[You have received this identical output 3 times. Re-reading 'agent://CeoReview' will not change it — use a narrower selector (path:A-B), or proceed with the edit.]