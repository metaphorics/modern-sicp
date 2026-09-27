(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.16 *)

(* The pending scaffold of the solution with the same name under
   solutions/ch3: every entry raises the pending marker until the
   exercise is solved. *)

(** Exercise 3.16: Ben's path-counting [count_pairs] against the four
    three-pair structures of the statement. *)

(** [count_pairs x] follows the book's definition: the count of the car
    plus the count of the cdr plus one. Shared pairs are counted once
    per path, and a cycle never finishes. *)

(** [count_pairs_bounded fuel x] is [Some n] with Ben's count [n] when
    the walk stayed within [fuel] pairs, and [None] otherwise. *)

(** [ex_3_16 ()] is [(Ben's answer for the unshared structure, for the
    once-shared one, for the doubly-shared one, the bounded answers for
    those three, and [None] for the ring)], the four structures being
    made of exactly three pairs each. *)
let count_pairs _x = raise Sicp_common.Pending.Pending_solution

let count_pairs_bounded _fuel _x = raise Sicp_common.Pending.Pending_solution
let ex_3_16 () = raise Sicp_common.Pending.Pending_solution
