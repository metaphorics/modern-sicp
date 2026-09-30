(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.8: the assembler's duplicate-label failure. *)

(** [ex_5_08 ()] reports the typed failure the assembler returns for a
    controller that defines one label twice; the machine never
    starts. *)
val ex_5_08 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
