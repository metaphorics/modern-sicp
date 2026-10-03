// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";

import { lookupVariableValue, Session } from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { call, exprStmt, ident, num, str } from "../../packages/ch4/src/syntax/ast.js";
import { admitSource } from "../../packages/ch4/src/syntax/check.js";
import {
  bodyClause,
  evalWithRecipientSwitch,
  type RecipientSwitch,
  recipientClause,
  recipientSwitch,
} from "./ex_4_05.js";

const setup = `
const entryValue = (entry: { key: string; value: number }): number => entry.value;
const entries = [
  { key: "a", value: 1 },
  { key: "b", value: 2 },
];
const lookup = (
  key: string,
  table: Array<{ key: string; value: number }>,
): { key: string; value: number } | undefined => table.find((entry) => entry.key === key);
const first = entries[0];
const second = entries[1];
let count = 0;
const bump = (entry: { key: string; value: number } | undefined): { key: string; value: number } | undefined => {
  count = count + 1;
  return entry;
};
`;

/** A session running one admitted setup unit in its global frame. */
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

const lookupSwitch = (key: string, matched: string): RecipientSwitch =>
  recipientSwitch(
    call(ident("lookup"), [str(key), ident("entries")]),
    [recipientClause(ident(matched), ident("entryValue"))],
    [exprStmt(str("missing"))],
  );

describe("exercise 4.5: recipient clauses in the switch case analysis", () => {
  it('a recipient clause is called with the matched value: lookup("b") is 2', () => {
    const { session, env } = envWith(setup);
    expect(shown(evalWithRecipientSwitch(lookupSwitch("b", "second"), env, session))).toBe("2");
  });

  it('the first entry matches the same way: lookup("a") is 1', () => {
    const { session, env } = envWith(setup);
    expect(shown(evalWithRecipientSwitch(lookupSwitch("a", "first"), env, session))).toBe("1");
  });

  it('an unmatched discriminant falls to the default body: lookup("c") is missing', () => {
    const { session, env } = envWith(setup);
    expect(shown(evalWithRecipientSwitch(lookupSwitch("c", "second"), env, session))).toBe(
      JSON.stringify("missing"),
    );
  });

  it("the discriminant is evaluated once and its value reaches the recipient", () => {
    const { session, env } = envWith(setup);
    const node = recipientSwitch(
      call(ident("bump"), [ident("second")]),
      [recipientClause(ident("second"), ident("entryValue"))],
      [exprStmt(str("missing"))],
    );
    expect(shown(evalWithRecipientSwitch(node, env, session))).toBe("2");
    const count = lookupVariableValue("count", env);
    expect(count.tag).toBe("ok");
    if (count.tag === "ok") {
      expect(count.value).toBe(1);
    }
  });

  it("a body clause runs its body", () => {
    const { session, env } = envWith(setup);
    const node = recipientSwitch(
      call(ident("lookup"), [str("a"), ident("entries")]),
      [bodyClause(ident("first"), [exprStmt(num(7))])],
      [exprStmt(str("missing"))],
    );
    expect(shown(evalWithRecipientSwitch(node, env, session))).toBe("7");
  });

  it("no clause and no default answers undefined", () => {
    const { session, env } = envWith(setup);
    const node = recipientSwitch(call(ident("lookup"), [str("c"), ident("entries")]), [
      recipientClause(ident("second"), ident("entryValue")),
    ]);
    expect(shown(evalWithRecipientSwitch(node, env, session))).toBe("undefined");
  });
});
