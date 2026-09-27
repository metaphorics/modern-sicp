(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.43: scanning internal definitions out of procedure
    bodies. *)

val source : string
val scan_out : Sicp_ch5.Sec_5_5.config
val body_shape : Sicp_ch5.Sec_5_5.config -> (bool * bool, Sicp_ch5.Sec_5_5.error) result
val ex_5_43 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
