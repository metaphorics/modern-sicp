(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.59 *)

val element_of_set : int -> int list -> bool
val adjoin_set : int -> int list -> int list

(** [union_set set1 set2] is the union of [set1] and [set2], each an
    unordered list that may repeat no element. *)
val union_set : int list -> int list -> int list

(** [ex_2_59 ()] is the union of [\{1; 2; 3\}] and [\{3; 4; 5\}]. *)
val ex_2_59 : unit -> int list
