(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.60 *)

val element_of_set : int -> int list -> bool

(** [adjoin_set x set] conses [x] onto [set] unconditionally, so the
    element list may repeat [x]. *)
val adjoin_set : int -> int list -> int list

(** [union_set set1 set2] is [set1] followed by [set2], with every
    element's duplicates from both kept. *)
val union_set : int list -> int list -> int list

val intersection_set : int list -> int list -> int list

(** The book's example: [\{1; 2; 3\}] as [(2 3 2 1 3 2 2)]. *)
val sample_set : int list

(** [ex_2_60 ()] is [sample_set] with [1] adjoined, paired with
    [sample_set] unioned with itself. *)
val ex_2_60 : unit -> int list * int list
