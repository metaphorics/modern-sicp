(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program tan-cf in SICP section 1.3
   exercise 1.39 *)

(** Reference solution of exercise 1.39. *)

val tan_cf : float -> int -> float

(** [ex_1_39 ()] is [(0.10033467208545055, 1.557407724654902)], each
    matching [Float.tan] at print precision. *)
val ex_1_39 : unit -> float * float
