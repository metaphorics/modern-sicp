// SPDX-License-Identifier: GPL-3.0-only
import {
  type CompilerConfig,
  compileProgram,
  defaultConfig,
  LinkageNext,
  newState,
} from "../../packages/ch5/src/05-compilation.js";

const SHADOWED = "(lambda (+ * a b x y) (+ (* a x) (* b y)))";
const FREE = "(lambda (a b x y) (+ (* a x) (* b y)))";

const openCodeCount = (source: string, cfg: CompilerConfig): number => {
  const text = compileProgram(cfg, newState(), source, LinkageNext)
    .stmts.map((line) => (line.tag === "assign" && line.source.tag === "op" ? line.source.op : ""))
    .join(" ");
  const machineOps = text.split(" ").filter((op) => op === "+" || op === "*").length;
  return machineOps;
};

/** Exercise 5.44: the open-coding dispatch consults the compile-time
 * environment first, so parameters named + and * shadow the primitives
 * and compile as ordinary procedure calls, while free names stay
 * eligible. A top-level rebind is outside the analysis: the top-level
 * compile-time environment is empty either way. */
export const ex_5_44 = (): readonly string[] => {
  const cfg: CompilerConfig = { ...defaultConfig(), openCode: true };
  const shadowed = openCodeCount(SHADOWED, cfg);
  const free = openCodeCount(FREE, cfg);
  if (shadowed !== 0) throw new Error("a shadowed name was open-coded");
  if (free === 0) throw new Error("no free name was open-coded");
  return [
    `shadowed + and *: ${shadowed} open-coded operations`,
    `free + and *: ${free} open-coded operations`,
    "compile-time lambda frames prevent open coding of rebound names; top-level rebinding is outside this analysis",
  ];
};
