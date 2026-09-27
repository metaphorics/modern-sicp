(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program plus in SICP section 1.2 exercise 1.9 *)

(** Exercise 1.9: [plus_deferred] defers its increment until the
    recursive call returns, building a chain of deferred [succ]s exactly
    as [Factorial.recursive] builds a chain of deferred multiplications:
    a recursive process. [plus_tail]'s self-call is its entire result, a
    tail position OCaml runs in constant space: an iterative process.
    Both compute the same function; only their processes differ. *)

let rec plus_deferred a b = if a = 0 then b else succ (plus_deferred (a - 1) b)
let rec plus_tail a b = if a = 0 then b else plus_tail (a - 1) (b + 1)
let ex_1_09 () = plus_deferred 4 5, plus_tail 4 5
