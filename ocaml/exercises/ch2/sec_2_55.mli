(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 2.3 exercise 2.55 *)

(** A symbolic datum: a symbol or a list of data. *)
type datum =
  | Symbol of string
  | List of datum list

(** [quote d] is the datum the quotation of [d] denotes: the list of
    the symbol [quote] and [d]. *)
val quote : datum -> datum

(** [car d] is the first element of the nonempty list [d].
    [invalid_arg] on any other datum. *)
val car : datum -> datum

(** [name_of d] is the name of the symbol [d]. [invalid_arg] on a list. *)
val name_of : datum -> string

(** [ex_2_55 ()] is the name of the symbol printed for the [car] of
    the quotation of the quotation of [abracadabra]. *)
val ex_2_55 : unit -> string
