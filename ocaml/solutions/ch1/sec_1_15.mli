(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sine in SICP section 1.2 exercise
   1.15 *)

(** Reference solution of exercise 1.15. *)

val cube : float -> float
val p : float -> float

(** [sine_with_count angle] is [(sine angle, applications of p)]. *)
val sine_with_count : float -> float * int

(** [ex_1_15 ()] is the number of times [p] is applied for
    [sine 12.15]: 5. *)
val ex_1_15 : unit -> int
