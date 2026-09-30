// SPDX-License-Identifier: GPL-3.0-only
// case core/16-symbols-quotation: SICP 2.3 symbolic data: quoted expressions re-cut as explicit tagged-union data, rendered and evaluated.
type Term =
  | { readonly tag: "number"; readonly value: number }
  | { readonly tag: "symbol"; readonly name: string }
  | { readonly tag: "sum"; readonly left: Term; readonly right: Term }
  | { readonly tag: "product"; readonly left: Term; readonly right: Term };
const num = (value: number): Term => ({ tag: "number", value });
const sym = (name: string): Term => ({ tag: "symbol", name });
const sum = (left: Term, right: Term): Term => ({ tag: "sum", left, right });
const product = (left: Term, right: Term): Term => ({ tag: "product", left, right });
function render(term: Term): string {
  switch (term.tag) {
    case "number":
      return `${term.value}`;
    case "symbol":
      return term.name;
    case "sum":
      return "(+ " + render(term.left) + " " + render(term.right) + ")";
    case "product":
      return "(* " + render(term.left) + " " + render(term.right) + ")";
  }
}
function evaluate(term: Term, x: number): number {
  switch (term.tag) {
    case "number":
      return term.value;
    case "symbol":
      return x;
    case "sum":
      return evaluate(term.left, x) + evaluate(term.right, x);
    case "product":
      return evaluate(term.left, x) * evaluate(term.right, x);
  }
}
const expression = sum(num(1), product(num(2), sym("x")));
console.log(render(expression));
console.log(evaluate(expression, 3));
console.log(sym("a").tag === "symbol");
