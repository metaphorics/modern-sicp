(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.18: the alternative scan-out strategy. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** [eval_text_strategy] evaluates the solve procedure under the
      text's strategy; [eval_alternative_strategy] under this
      exercise's. Each answers the observation trace of one call. *)
val eval_text_strategy : unit -> string list

val eval_alternative_strategy : unit -> string list

(** [ex_4_18 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_18 : unit -> string list
