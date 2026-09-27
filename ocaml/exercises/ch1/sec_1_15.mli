(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program sine in SICP section 1.2 exercise
   1.15 *)

(** Exercise 1.15: how many times [p] is applied by [sine], and the
    order of growth of the process it generates. The stubs raise
    [Sicp_common.Pending.Pending_solution] until they are solved. *)

val cube : float -> float
val p : float -> float

(** [sine_with_count angle] is [(sine angle, applications of p)]. *)
val sine_with_count : float -> float * int

(** [ex_1_15 ()] is the number of times [p] is applied for
    [sine 12.15]. *)
val ex_1_15 : unit -> int
