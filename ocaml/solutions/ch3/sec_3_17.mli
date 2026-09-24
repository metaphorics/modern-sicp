(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.3 exercise 3.17 *)

(** Exercise 3.17: counting distinct pairs with a visited-by-identity
    auxiliary structure. *)

open Sicp_ch3.Sec_3_3.Mpairs

(** [count_pairs_distinct x] is the number of distinct pairs in [x];
    each pair is counted once, and cycles terminate the walk. *)
val count_pairs_distinct : mobj -> int

(** [ex_3_17 ()] is [(the distinct count of the unshared three-pair
    structure, of the once-shared one, of the doubly-shared one, and of
    the three-pair ring)], every answer being 3. *)
val ex_3_17 : unit -> int * int * int * int
