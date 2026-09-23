(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.37:
   matrix operations *)

(** The statement's dot product sample, [1 2 3] against [4 5 6]. *)
val ex_2_37 : unit -> int

val dot_product : int list -> int list -> int
val matrix_times_vector : int list list -> int list -> int list
val transpose : int list list -> int list list
val matrix_times_matrix : int list list -> int list list -> int list list
