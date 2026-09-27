(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.12: the assembler's analysis of a controller. *)

(** [analysis controller] computes the four lists the exercise asks
    for over the controller's parsed instructions: the deduplicated
    instructions sorted by type, the entry-point registers, the saved
    and restored registers, and each register's sources, as report
    lines. *)
val analysis : string -> (string list, Sicp_ch5.Sec_5_2.error) result

(** [ex_5_12 ()] analyzes the Fibonacci machine of Figure 5.12 and the
    recursive factorial machine of Figure 5.11, whose [val] sources the
    book quotes. *)
val ex_5_12 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
