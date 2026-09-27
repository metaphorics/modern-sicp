(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program cont-frac in SICP section 1.3
   exercise 1.38 *)

(** Exercise 1.38: Euler's continued-fraction expansion of [e - 2]. The
    stubs raise [Sicp_common.Pending.Pending_solution] until they are
    solved. *)

(** [euler_d i] is the [i]th denominator of Euler's expansion: 1, 2, 1,
    1, 4, 1, 1, 6, 1, 1, 8, ..., i.e. [2 * (i + 1) / 3] where [i + 1]
    is a multiple of 3, and 1 elsewhere. *)
val euler_d : int -> float

(** [e_approx k] is [2. +. Sec_1_37.cont_frac (fun _ -> 1.) euler_d k]. *)
val e_approx : int -> float

val ex_1_38 : unit -> float
