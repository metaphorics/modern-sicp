(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.9: operations cannot take labels as operands. *)

(** [ex_5_09 ()] runs the legal operand forms and reports the typed
    refusal of a label in an operand position. *)
val ex_5_09 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
