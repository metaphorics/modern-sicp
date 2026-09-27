(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.14: the host map installed as a primitive. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [eval] is the evaluator with the host-flavored [map] primitive
      installed, the state Louis creates. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_14 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_14 : unit -> string list

(** [ex_4_14a ()] is the tailored addition: the host sort installed as
    a primitive sorts data and fails on an object-language procedure,
    while the same sort defined in the object language does both. The
    trace names each step's outcome. *)
val ex_4_14a : unit -> string list
