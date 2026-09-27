(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program f in SICP section 1.3
   exercise 1.34 *)

(** Exercise 1.34: [f] applied to procedures that type-check against
    its inferred [(int -> 'a) -> 'a]; the [f f] case itself is
    explained, not run, in [ex_1_34.md]. *)

let f g = g 2
let square x = x * x
let ex_1_34 () = f square, f (fun z -> z * (z + 1))
