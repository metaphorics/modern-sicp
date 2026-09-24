(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 2.3 exercise 2.54 *)

(** A datum of the section's symbolic data: a symbol, or a sequence of
    data (the analogue of a Scheme list of symbols and sublists). *)
type datum =
  | Sym of string
  | Seq of datum list

(** [equal_datum a b] holds when [a] and [b] are both symbols with the
    same name, or both sequences of pairwise [equal_datum] data. *)
val equal_datum : datum -> datum -> bool

(** [words names] is the sequence of one symbol per name. *)
val words : string list -> datum

(** [ex_2_54 ()] is the book's first example: whether
    [(this is a list)] equals itself. *)
val ex_2_54 : unit -> bool
