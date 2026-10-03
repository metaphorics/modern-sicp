// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import { describe, expect, it } from "vitest";
import {
  defineVariableValue,
  lookupVariableValue,
  Session,
} from "../../packages/ch4/src/01-metacircular.js";
import { format } from "../../packages/ch4/src/read.js";
import type { Env } from "../../packages/ch4/src/runtime/env.js";
import { child } from "../../packages/ch4/src/runtime/env.js";
import type { Outcome } from "../../packages/ch4/src/runtime/errors.js";
import { str } from "../../packages/ch4/src/syntax/ast.js";
import { evalWithUnbind, removeBinding, unbindNode } from "./ex_4_13.js";

const envWith = (): { session: Session; env: Env } => {
  const session = new Session("core");
  const env = session.globalEnv();
  defineVariableValue("a", 1, env);
  return { session, env };
};

const shown = (outcome: Outcome): string =>
  outcome.tag === "ok" ? format(outcome.value) : `error:${outcome.error.tag}`;

describe("exercise 4.13: unbind removes a binding", () => {
  it("drops the inner binding and exposes the outer one", () => {
    const { session, env } = envWith();
    const inner = child(env);
    defineVariableValue("a", 2, inner);
    expect(shown(lookupVariableValue("a", inner))).toBe("2");
    expect(shown(evalWithUnbind(unbindNode(str("a")), inner, session))).toBe("undefined");
    expect(shown(lookupVariableValue("a", inner))).toBe("1");
  });

  it("a write after unbinding updates the outer frame", () => {
    const { session, env } = envWith();
    const inner = child(env);
    defineVariableValue("a", 2, inner);
    evalWithUnbind(unbindNode(str("a")), inner, session);
    expect(session.setVariableValue("a", 50, inner).tag).toBe("ok");
    expect(shown(lookupVariableValue("a", env))).toBe("50");
  });

  it("unbinding a name no frame has fails with unbound-name", () => {
    const { session, env } = envWith();
    expect(shown(evalWithUnbind(unbindNode(str("zz")), env, session))).toBe("error:unbound-name");
  });

  it("after unbinding the global name, later reads and unbinds fail", () => {
    const { session, env } = envWith();
    expect(shown(removeBinding("a", env))).toBe("undefined");
    expect(shown(lookupVariableValue("a", env))).toBe("error:unbound-name");
    expect(shown(removeBinding("a", env))).toBe("error:unbound-name");
    expect(session.transcript).toStrictEqual([]);
  });
});
