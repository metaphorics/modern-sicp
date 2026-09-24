(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.61 *)

(** [adjoin_set x set] inserts [x] into the ordered set [set] at the
    position that keeps it ordered, or returns [set] unchanged when
    [x] is already present. *)
val adjoin_set : int -> int list -> int list

(** [ex_2_61 ()] is [\{1; 3; 6; 10\}] with [4] adjoined. *)
val ex_2_61 : unit -> int list
