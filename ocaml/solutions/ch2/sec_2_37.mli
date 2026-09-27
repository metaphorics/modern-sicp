(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.37: matrix operations as sequence operations. *)

(** [dot_product [1; 2; 3] [4; 5; 6]], the statement's sample. *)
val ex_2_37 : unit -> int

val dot_product : int list -> int list -> int
val matrix_times_vector : int list list -> int list -> int list
val transpose : int list list -> int list list
val matrix_times_matrix : int list list -> int list list -> int list list
