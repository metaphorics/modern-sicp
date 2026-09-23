(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.21:
   square-list two ways *)

(** The direct recursive definition, consing the square of the first
    item onto the square-list of the rest. *)
val ex_2_21_direct : int list -> int list

(** The same computation expressed with [List.map]. *)
val ex_2_21_map : int list -> int list
