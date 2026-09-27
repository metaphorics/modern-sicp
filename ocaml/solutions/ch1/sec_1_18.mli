(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program * in SICP section 1.2 exercise 1.18 *)

(** Reference solution of exercise 1.18. *)

(** [mult_iter total a b] holds the invariant [total + a * b] and
    reaches [total = a * b] once [b] is 0, a logarithmic number of
    steps. *)
val mult_iter : int -> int -> int -> int

(** [ex_1_18 a b] is [a * b]; [ex_1_18 3 7] is 21. *)
val ex_1_18 : int -> int -> int
