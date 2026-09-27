// SPDX-License-Identifier: GPL-3.0-only
import {
  compileProgram,
  defaultConfig,
  LinkageNext,
  newStateSeeded,
  statementsText,
} from "../../packages/ch5/src/05-compilation.js";

const SOURCE = "(define (f x) (+ x (g (+ x 2))))";

/** The expression behind Figure 5.18, with the label counter seeded at
 * fourteen: the labels the book's session had already generated. The
 * entry operand is the label reference the book's figure shows, which
 * this edition's assembler resolves to the entry address. */
export const ex_5_35 = (): readonly string[] => {
  const listing = statementsText(
    compileProgram(defaultConfig(), newStateSeeded(14), SOURCE, LinkageNext),
  );
  for (const label of ["entry16", "after-lambda15", "after-call23"]) {
    if (!listing.includes(label)) throw new Error(`the figure label ${label} is missing`);
  }
  const count = listing.split("\n").length;
  return [
    `expression: ${SOURCE}`,
    `Figure 5.18 reproduced: ${count} controller statements`,
    listing,
  ];
};
