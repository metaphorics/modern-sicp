// SPDX-License-Identifier: GPL-3.0-only
// case core/12-count-leaves: SICP 2.2.2 count-leaves over a recursive tree union.
type Tree = { readonly tag: "leaf"; readonly value: number } | { readonly tag: "node"; readonly left: Tree; readonly right: Tree };
const leaf = (value: number): Tree => ({ tag: "leaf", value });
const node = (left: Tree, right: Tree): Tree => ({ tag: "node", left, right });
function countLeaves(tree: Tree): number {
  return tree.tag === "leaf" ? 1 : countLeaves(tree.left) + countLeaves(tree.right);
}
function sumLeaves(tree: Tree): number {
  return tree.tag === "leaf" ? tree.value : sumLeaves(tree.left) + sumLeaves(tree.right);
}
const tree = node(node(leaf(1), leaf(2)), node(node(leaf(3), leaf(4)), leaf(5)));
console.log(countLeaves(tree));
console.log(sumLeaves(tree));
