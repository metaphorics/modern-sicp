(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cubic in SICP section 1.3
   exercise 1.40 *)

(** Reference solution of exercise 1.40. *)

val cubic : float -> float -> float -> float -> float
val deriv : (float -> float) -> float -> float
val newton_transform : (float -> float) -> float -> float
val newtons_method : (float -> float) -> float -> float

(** [ex_1_40 ()] is [1.], the root [newtons_method] finds first from a
    guess of [1.]. *)
val ex_1_40 : unit -> float
