(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.16 *)

(** Exercise 3.16: Ben's path-counting [count_pairs] against the four
    three-pair structures of the statement. *)

open Sicp_ch3.Sec_3_3.Mpairs

(** [count_pairs x] follows the book's definition: the count of the car
    plus the count of the cdr plus one. Shared pairs are counted once
    per path, and a cycle never finishes. *)
val count_pairs : mobj -> int

(** [count_pairs_bounded fuel x] is [Some n] with Ben's count [n] when
    the walk stayed within [fuel] pairs, and [None] otherwise. *)
val count_pairs_bounded : int -> mobj -> int option

(** [ex_3_16 ()] is [(Ben's answer for the unshared structure, for the
    once-shared one, for the doubly-shared one, the bounded answers for
    those three, and [None] for the ring)], the four structures being
    made of exactly three pairs each. *)
val ex_3_16 : unit -> int * int * int * int option * int option * int option * int option
