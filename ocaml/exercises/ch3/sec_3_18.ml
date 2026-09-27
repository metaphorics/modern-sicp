(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.18 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.18: detecting a cycle with a visited-by-identity scan. *)

(** [contains_cycle x] is whether some cdr chain from [x] reaches a pair
    it has already visited. *)

(** [ex_3_18 ()] is [(the answer for the plain list (a b c), for the
    (a b c) ring of exercise 3.13, for the one-pair list whose cdr
    points at itself, and for the empty list)]. *)
let contains_cycle _x = raise Sicp_common.Pending.Pending_solution

let ex_3_18 () = raise Sicp_common.Pending.Pending_solution
