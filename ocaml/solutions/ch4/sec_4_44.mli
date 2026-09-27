(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.44 with the tailored addition 4.44a: queens under [amb] and the backtrack counts per board size. The statement lives in the section; this
    signature is the exercise's public contract. *)

(** [ex_4_44 ()] runs the demonstration the statement asks for
    and answers its observable outcomes as printed strings, in the
    order the statement raises them. *)
val ex_4_44 : unit -> string list

(** [ex_4_44a ()] asserts the 4.44a backtrack counts per board size as
    printed strings. *)
val ex_4_44a : unit -> string list
