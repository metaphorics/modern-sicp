// SPDX-License-Identifier: GPL-3.0-only
// case query/01-basic-personnel: SICP 4.4.1 simple queries: assertions as typed Query data, pattern variables, ordered answers.
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
  assertions: [
    atom("job", t("Bitdiddle Ben"), list(t("computer"), t("wizard"))),
    atom("job", t("Hacker Alyssa P"), list(t("computer"), t("programmer"))),
    atom("job", t("Fect Cy D"), list(t("computer"), t("programmer"))),
    atom("job", t("Tweakit Lem E"), list(t("computer"), t("technician"))),
    atom("salary", t("Bitdiddle Ben"), t(60000)),
    atom("salary", t("Hacker Alyssa P"), t(40000)),
    atom("address", t("Bitdiddle Ben"), list(t("Slumerville"), t("Ridge Road"), t(10))),
  ],
  rules: [],
  queries: [
    atom("job", v("x"), list(t("computer"), t("programmer"))),
    atom("job", v("x"), pair(t("computer"), v("type"))),
    atom("address", v("x"), v("y")),
    atom("salary", t("Fect Cy D"), v("amount")),
  ],
};
