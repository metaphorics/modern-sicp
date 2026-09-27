(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** Exercise 5.33: the alternative factorial's compilation. *)

val factorial_source : string
val factorial_alt_source : string

val statements_of
  :  Sicp_ch5.Sec_5_5.state
  -> string
  -> (string list, Sicp_ch5.Sec_5_5.error) result

val saves_of : string list -> string list
val run : string -> (string list, Sicp_ch5.Sec_5_5.error) result
val ex_5_33 : unit -> (string list, Sicp_ch5.Sec_5_5.error) result

(** Exercise 5.33a: the measured win of the hand-optimized compilation. *)

val alt_body_source : string

(** The body of the alternative factorial as the compiler emits it,
    entered with the compiled calling convention. *)
val naive_statements : string list

(** The hand-optimized body: constants folded, the re-derivable [proc]
    save dropped, unreferenced labels gone. *)
val optimized_statements : string list

val naive_block : string
val optimized_block : string

(** [measure block n] runs [block] as [factorial-alt] on argument [n]
    and answers the transcript and the machine's executed-instruction
    count; the harness instructions are identical for every block. *)
val measure : string -> int -> (string list * int, Sicp_ch5.Sec_5_5.error) result

val ex_5_33a : unit -> (string list, Sicp_ch5.Sec_5_5.error) result
