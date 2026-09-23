(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program * in SICP section 1.2 exercise 1.17 *)

(** Reference solution of exercise 1.17. *)

(** [times a b] is [a * b] by repeated addition, @math{{\Theta(b)}}
    steps. *)
val times : int -> int -> int

val double : int -> int
val halve : int -> int

(** [fast_mult a b] is [a * b] by doubling and halving,
    @math{{\Theta(\log b)}} steps. *)
val fast_mult : int -> int -> int

(** [ex_1_17 a b] is [fast_mult a b]; [ex_1_17 3 7] is 21. *)
val ex_1_17 : int -> int -> int
