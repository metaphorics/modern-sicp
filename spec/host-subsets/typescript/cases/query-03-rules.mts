// SPDX-License-Identifier: GPL-3.0-only
// case query/03-rules: SICP 4.4.1 rules: lives-near with renamed rule variables, dotted-tail patterns, and a same rule under not.
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
    atom("address", t("Bitdiddle Ben"), list(t("Slumerville"), t("Ridge Road"), t(10))),
    atom("address", t("Hacker Alyssa P"), list(t("Cambridge"), t("Mass Ave"), t(78))),
    atom("address", t("Fect Cy D"), list(t("Cambridge"), t("Ames Street"), t(3))),
    atom("address", t("Reasoner Louis"), list(t("Slumerville"), t("Pine Tree Road"), t(80))),
  ],
  rules: [
    rule(atom("same", v("x"), v("x"))),
    rule(
      atom("lives-near", v("person-1"), v("person-2")),
      atom("address", v("person-1"), pair(v("town"), v("rest-1"))),
      atom("address", v("person-2"), pair(v("town"), v("rest-2"))),
      not(atom("same", v("person-1"), v("person-2"))),
    ),
  ],
  queries: [
    atom("lives-near", v("x"), t("Bitdiddle Ben")),
    atom("lives-near", t("Hacker Alyssa P"), v("who")),
    atom("lives-near", t("Bitdiddle Ben"), t("Fect Cy D")),
  ],
};
