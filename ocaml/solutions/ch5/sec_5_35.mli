(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.35: the expression behind Figure 5.18, reproduced by
    seeding the label counter at 14. *)

val source : string
val figure_statements : string list
val ex_5_35 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
