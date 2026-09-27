(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.61 *)

(** Exercise 2.61: [adjoin_set] for the ordered-list representation.
    By analogy with [element_of_set?], the search can stop as soon as
    it passes the point where [x] belongs, needing on average about
    half as many steps as scanning the whole unordered list. *)

let rec adjoin_set x = function
  | [] -> [ x ]
  | first :: rest as set ->
    if x = first then set else if x < first then x :: set else first :: adjoin_set x rest
;;

(** [ex_2_61 ()] is [\{1; 3; 6; 10\}] with [4] adjoined. *)
let ex_2_61 () = adjoin_set 4 [ 1; 3; 6; 10 ]
