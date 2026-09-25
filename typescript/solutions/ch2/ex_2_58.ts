// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

import {
  constant,
  type Datum,
  type Expr,
  makeProduct,
  makeSum,
  qnum,
  variable,
} from "../../packages/ch2/src/03-symbolic-data.js";

/**
 * Exercise 2.58: infix notation. Both parsers turn the section's
 * quoted data into the section's `Expr`, so the untouched `deriv`
 * differentiates ordinary infix. Part (a) assumes full parenthesization;
 * part (b) takes the standard precedence: `*` before `+`.
 */

const itemsOf = (d: Datum): ReadonlyArray<Datum> => {
  if (d._tag !== "Lst") {
    throw new Error("expected an infix expression datum");
  }
  const items: Datum[] = [];
  for (let rest = d.items; rest._tag === "Cons"; rest = rest.tail) {
    items.push(rest.head);
  }
  return items;
};

const atom = (d: Datum): Expr =>
  d._tag === "Num" ? constant(d.n) : d._tag === "Sym" ? variable(d.name) : parseInfix(d);

/** a. The fully parenthesized infix form: every sum or product is a
 * three-item datum `(a op b)`, with bare symbols and numbers as
 * leaves. */
export function parseInfix(d: Datum): Expr {
  if (d._tag !== "Lst") {
    return atom(d);
  }
  const items = itemsOf(d);
  if (items.length === 1) {
    return atom(items[0] ?? qnum(0));
  }
  const [left, op, right] = items;
  if (left === undefined || op === undefined || right === undefined || op._tag !== "Sym") {
    throw new Error("malformed infix expression");
  }
  return op.name === "+"
    ? makeSum(parseInfix(left), parseInfix(right))
    : makeProduct(parseInfix(left), parseInfix(right));
}

/** Splits a flat token list at the top-level occurrences of `op`. */
const splitTop = (
  tokens: ReadonlyArray<Datum>,
  op: string,
): ReadonlyArray<ReadonlyArray<Datum>> => {
  const parts: ReadonlyArray<Datum>[] = [];
  let current: Datum[] = [];
  for (const token of tokens) {
    if (token._tag === "Sym" && token.name === op && current.length > 0) {
      parts.push(current);
      current = [];
    } else {
      current.push(token);
    }
  }
  parts.push(current);
  return parts;
};

/** Flattens a parenthesized group into the token stream it holds. */
const groupTokens = (d: Datum): ReadonlyArray<Datum> => (d._tag === "Lst" ? itemsOf(d) : [d]);

const parseSumStandard = (tokens: ReadonlyArray<Datum>): Expr => {
  const parts = splitTop(tokens, "+");
  const first = parts[0];
  if (first === undefined) {
    throw new Error("empty infix expression");
  }
  return parts
    .slice(1)
    .reduce<Expr>(
      (acc, part) => makeSum(acc, parseProductStandard(part)),
      parseProductStandard(first),
    );
};

const parseProductStandard = (tokens: ReadonlyArray<Datum>): Expr => {
  const parts = splitTop(tokens, "*");
  const first = parts[0];
  if (first === undefined) {
    throw new Error("empty infix expression");
  }
  return parts
    .slice(1)
    .reduce<Expr>((acc, part) => makeProduct(acc, parsePrimary(part)), parsePrimary(first));
};

const parsePrimary = (tokens: ReadonlyArray<Datum>): Expr =>
  tokens.length === 1 && tokens[0] !== undefined && tokens[0]._tag !== "Lst"
    ? atom(tokens[0])
    : parseSumStandard(tokens.flatMap(groupTokens));

/** b. The standard algebraic notation: parentheses dropped where
 * precedence allows, `*` binding tighter than `+`. */
export const parseInfixStandard = (d: Datum): Expr => parseSumStandard(itemsOf(d));
