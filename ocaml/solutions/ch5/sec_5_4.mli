(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.4: the recursive and iterative exponentiation machines. *)

(** The recursive machine: registers [b], [n], [val], and [continue];
    each level saves [continue] and multiplies [b] into [val] on the
    way back. *)
val expt_recursive_controller : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** The iterative machine: registers [b], [counter], and [product]; no
    stack and no [continue]. *)
val expt_iterative_controller : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** [ex_5_04 ()] runs both machines on (2, 10) and (3, 5). *)
val ex_5_04 : unit -> (string list, Sicp_ch5.Sec_5_1.error) result
