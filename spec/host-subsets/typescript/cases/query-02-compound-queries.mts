// SPDX-License-Identifier: GPL-3.0-only
// case query/02-compound-queries: SICP 4.4.1 compound queries: and, or, and not (negation as failure) over typed Query data.
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
    atom("supervisor", t("Hacker Alyssa P"), t("Bitdiddle Ben")),
    atom("supervisor", t("Fect Cy D"), t("Bitdiddle Ben")),
    atom("supervisor", t("Tweakit Lem E"), t("Bitdiddle Ben")),
  ],
  rules: [],
  queries: [
    and(atom("job", v("person"), list(t("computer"), t("programmer"))), atom("supervisor", v("person"), v("boss"))),
    or(atom("supervisor", v("x"), t("Bitdiddle Ben")), atom("job", v("x"), list(t("computer"), t("wizard")))),
    and(atom("supervisor", v("x"), t("Bitdiddle Ben")), not(atom("job", v("x"), list(t("computer"), t("programmer"))))),
    and(atom("job", v("x"), list(t("computer"), t("wizard"))), not(atom("supervisor", v("x"), v("anyone")))),
  ],
};
