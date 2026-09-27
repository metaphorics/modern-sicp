(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.42: the eight-queens puzzle. *)

val empty_board : (int * int) list
val adjoin_position : int -> int -> (int * int) list -> (int * int) list
val safe : int -> (int * int) list -> bool
val ex_2_42 : int -> (int * int) list list
