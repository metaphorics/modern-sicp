(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.55 *)

(** [car datum] is the first element of the pair datum [datum].
    [invalid_arg] on any other datum. *)
val car : Sicp_common.Ast.datum -> Sicp_common.Ast.datum

(** [name_of datum] is the name of the symbol datum [datum].
    [invalid_arg] on any other datum. *)
val name_of : Sicp_common.Ast.datum -> string

(** [quoted_datum text] is the datum a top-level [Quote] expression
    wraps, after reading [text]. [invalid_arg] when [text] fails to
    read or does not read as a quote. *)
val quoted_datum : string -> Sicp_common.Ast.datum

(** [ex_2_55 ()] is the name of the symbol [''abracadabra] prints back. *)
val ex_2_55 : unit -> string
