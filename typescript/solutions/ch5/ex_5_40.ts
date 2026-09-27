// SPDX-License-Identifier: GPL-3.0-only
import {
  type Cenv,
  type CompilerConfig,
  compileProgram,
  defaultConfig,
  LinkageNext,
  newState,
} from "../../packages/ch5/src/05-compilation.js";

const EXAMPLE =
  "((lambda (x y) (lambda (a b c d e) ((lambda (y z) (* x y z)) (* a b x) (+ c d x)))) 3 4)";

/** Exercise 5.40: the compile-time environment is threaded through
 * every code generator, and the trace reports the frame each variable
 * reference was compiled against. */
export const ex_5_40 = (): readonly string[] => {
  const trace: string[] = [];
  const cfg: CompilerConfig = {
    ...defaultConfig(),
    trace: (frames: Cenv, name) => {
      const rendered = frames.map((frame) => `(${frame.join(" ")})`).join(" ");
      trace.push(`${name} in (${rendered})`);
    },
  };
  compileProgram(cfg, newState(), EXAMPLE, LinkageNext);
  if (!trace.some((line) => line === "x in ((y z) (a b c d e) (x y))"))
    throw new Error(`the trace missed x: ${trace.join(" | ")}`);
  if (!trace.some((line) => line === "z in ((y z) (a b c d e) (x y))"))
    throw new Error("the trace missed z");
  return trace;
};
