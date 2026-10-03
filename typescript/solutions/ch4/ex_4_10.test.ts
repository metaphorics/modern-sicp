// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";
import { evalFormIn, evalProgramIn, makeReversedSyntax, makeStandardSyntax } from "./ex_4_10.js";

const envWith = (source: string): { session: Session; env: Env } => {
  const admission = admitSource(source);
  if (!admission.ok) {
    throw new Error(`setup source must admit: ${admission.diagnostics[0]?.construct ?? "reject"}`);
  }
  const session = new Session("core");
  const env = session.globalEnv();
  session.execSequence(admission.program, env);
  return { session, env };
};

/** The observable result: the rendered value, or the fault category. */
const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

describe("exercise 4.10: one evaluator, two syntaxes", () => {
  it("the square program answers 49 under both tables", () => {
    const standard = envWith("let seen = 0;");
    const reversed = envWith("let seen = 0;");
    expect(
      shown(
        evalProgramIn(
          makeStandardSyntax(),
          'const square = fn("x", x * x); square(7);',
          standard.env,
          standard.session,
        ),
      ),
    ).toBe("49");
    expect(
      shown(
        evalProgramIn(
          makeReversedSyntax(),
          'const square = nf("x", x * x); square(7);',
          reversed.env,
          reversed.session,
        ),
      ),
    ).toBe("49");
  });

  it("the branch form answers 1 under both spellings", () => {
    const standard = envWith("let seen = 0;");
    const reversed = envWith("let seen = 0;");
    expect(
      shown(
        evalFormIn(makeStandardSyntax(), "branch(3 > 2, 1, 0)", standard.env, standard.session),
      ),
    ).toBe("1");
    expect(
      shown(
        evalFormIn(makeReversedSyntax(), "hcnarb(3 > 2, 1, 0)", reversed.env, reversed.session),
      ),
    ).toBe("1");
  });

  it("a tag is just a name: the wrong table leaves the form an unbound application", () => {
    const standard = envWith("let seen = 0;");
    const reversed = envWith("let seen = 0;");
    const wrongTable = evalFormIn(
      makeReversedSyntax(),
      "branch(3 > 2, 1, 0)",
      reversed.env,
      reversed.session,
    );
    expect(shown(wrongTable)).toBe("error:unbound-name");
    if (wrongTable.tag === "error" && wrongTable.error.tag === "unbound-name") {
      expect(wrongTable.error.name).toBe("branch");
    }
    const wrongSpelling = evalFormIn(
      makeStandardSyntax(),
      "hcnarb(3 > 2, 1, 0)",
      standard.env,
      standard.session,
    );
    expect(shown(wrongSpelling)).toBe("error:unbound-name");
    if (wrongSpelling.tag === "error" && wrongSpelling.error.tag === "unbound-name") {
      expect(wrongSpelling.error.name).toBe("hcnarb");
    }
  });

  it("procedure bodies follow the table of the evaluator running them", () => {
    const standard = envWith("let seen = 0;");
    expect(
      shown(
        evalFormIn(
          makeStandardSyntax(),
          'fn("x", branch(x > 0, 1, -1))(5)',
          standard.env,
          standard.session,
        ),
      ),
    ).toBe("1");
    expect(
      shown(
        evalFormIn(
          makeStandardSyntax(),
          'fn("x", branch(x > 0, 1, -1))(-5)',
          standard.env,
          standard.session,
        ),
      ),
    ).toBe("-1");
    expect(
      shown(
        evalFormIn(makeStandardSyntax(), 'seq(set("seen", 1), 7)', standard.env, standard.session),
      ),
    ).toBe("7");
  });
});
