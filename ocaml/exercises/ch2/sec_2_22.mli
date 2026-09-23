(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.2 exercise 2.22:
   the iterative square-list bug *)

(** [ex_2_22_louis items] runs Louis's iterative process, which conses
    each square onto the accumulated answer and so returns the squares
    in reverse order. *)
val ex_2_22_louis : int list -> int list
