// SPDX-License-Identifier: GPL-3.0-only
// case query/04-append-form: SICP 4.4.1 append-to-form: a recursive rule over pair terms, run forwards and backwards.
type Term =
  | { readonly tag: "var"; readonly name: string }
  | { readonly tag: "text"; readonly value: string | number | boolean }
  | { readonly tag: "cons"; readonly head: Term; readonly tail: Term }
  | { readonly tag: "nil" };
type Query =
  | { readonly tag: "atom"; readonly relation: string; readonly fields: ReadonlyArray<Term> }
  | { readonly tag: "and"; readonly clauses: ReadonlyArray<Query> }
  | { readonly tag: "or"; readonly clauses: ReadonlyArray<Query> }
  | { readonly tag: "not"; readonly clause: Query };
interface Rule {
  readonly head: Query;
  readonly body: ReadonlyArray<Query>;
}
interface QueryProgram {
  readonly assertions: ReadonlyArray<Query>;
  readonly rules: ReadonlyArray<Rule>;
  readonly queries: ReadonlyArray<Query>;
}
const v = (name: string): Term => ({ tag: "var", name });
const t = (value: string | number | boolean): Term => ({ tag: "text", value });
const pair = (head: Term, tail: Term): Term => ({ tag: "cons", head, tail });
const list = (...items: Term[]): Term => items.reduceRight((tail: Term, head: Term): Term => pair(head, tail), { tag: "nil" });
const atom = (relation: string, ...fields: Term[]): Query => ({ tag: "atom", relation, fields });
const and = (...clauses: Query[]): Query => ({ tag: "and", clauses });
const or = (...clauses: Query[]): Query => ({ tag: "or", clauses });
const not = (clause: Query): Query => ({ tag: "not", clause });
const rule = (head: Query, ...body: Query[]): Rule => ({ head, body });
export const program: QueryProgram = {
  assertions: [],
  rules: [
    rule(atom("append-to-form", list(), v("y"), v("y"))),
    rule(
      atom("append-to-form", pair(v("u"), v("v")), v("y"), pair(v("u"), v("z"))),
      atom("append-to-form", v("v"), v("y"), v("z")),
    ),
  ],
  queries: [
    atom("append-to-form", list(t("a"), t("b")), list(t("c"), t("d")), v("z")),
    atom("append-to-form", list(t("a"), t("b")), v("y"), list(t("a"), t("b"), t("c"), t("d"))),
    atom("append-to-form", v("x"), v("y"), list(t("a"), t("b"), t("c"), t("d"))),
  ],
};
