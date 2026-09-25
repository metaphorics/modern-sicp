(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.2: the machine language description, assembled and run. *)

(** [ex_5_02 ()] reports the assembly of the 5.1 controller text --
    instruction and label counts with the label table -- and runs it on
    5 and 6. *)
val ex_5_02 : unit -> (string list, Sicp_ch5.Sec_5_1.error) result
