(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.62 *)

(** [union_set set1 set2] is the ordered union of the ordered sets
    [set1] and [set2], computed in [Theta(n)]. *)
val union_set : int list -> int list -> int list

(** [ex_2_62 ()] is the union of [\{1; 3; 5; 7\}] and [\{2; 3; 6; 7\}]. *)
val ex_2_62 : unit -> int list
