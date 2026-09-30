(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.12: the assembler's analysis of a controller. *)

(** [analysis controller] computes the four lists the exercise asks
    for over the assembled instructions of [controller]: the
    deduplicated instructions sorted by type, the entry-point
    registers, the saved and restored registers, and each assigned
    register's sources, as report lines. Sources are rendered by the
    engine's instruction renderer. *)
val analysis
  :  Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
  -> (string list, Sicp_ch5.Sec_5_2.error) result

(** [ex_5_12 ()] analyzes the Fibonacci machine of Figure 5.12 and the
    recursive factorial machine of Figure 5.11, whose [val] sources the
    book quotes. *)
val ex_5_12 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
