(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.35: the expression behind Figure 5.18, and the listing
    notation the section's solutions print compiled code in.

    A listing shows each statement in the constructor notation of
    [Sicp_ch5.Sec_5_1].  An expression constant shows only what its
    operation reads from it: a variable its name, a [fun] its
    parameters, an operator node its operator, a construction its
    constructor, a [let] group its names.  The figure therefore does not
    spell out the source it came from. *)

(** [word_to_string w] is [w] in listing notation. *)
val word_to_string : Sicp_ch5.Sec_5_4.word -> string

(** [statement_to_string i] is [i] in listing notation. *)
val statement_to_string : Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction -> string

(** [listing seq] is every statement of [seq] in listing notation. *)
val listing : Sicp_ch5.Sec_5_5.seq -> string list

(** [figure] is Figure 5.18: the compiled code of one procedure
    definition, as the edition's compiler prints it. *)
val figure : string list

(** [answer] is the definition whose compilation [figure] shows. *)
val answer : string

(** [ex_5_35 ()] is [answer], its compilation, and whether that
    compilation equals [figure] statement for statement. *)
val ex_5_35 : unit -> (string list, Sicp_common.Eval_error.t) result
