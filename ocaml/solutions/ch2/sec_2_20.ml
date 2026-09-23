(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.20: same-parity in
   the rest-argument idiom. *)

(* [x - first] is even exactly when the two have the same parity; the
   difference, unlike [mod] on negative operands, needs no case
   analysis over signs. *)
let ex_2_20 first rest = first :: List.filter (fun x -> (x - first) mod 2 = 0) rest
