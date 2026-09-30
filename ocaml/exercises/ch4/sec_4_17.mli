(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.17 *)

(** Exercise 4.17: the extra frame of scanned-out definitions. The statement lives in the section; this
    signature is the pending exercise's public contract. *)

(** How a call runs a body that opens with internal definitions. *)
type strategy =
  | Scanned (** Scan out into a [let], one frame more per call. *)
  | Shared_frame (** Put the definitions' cells in the call frame. *)

(** [measure strategy source] evaluates the admitted expression
    [source] under [strategy] and answers its value with the number of
    frames the run made: one per [let] and one per closure call.  Only
    calls of one-parameter procedures run through the strategy. *)
val measure
  :  strategy
  -> string
  -> (Sicp_common.Value.t * int, Sicp_common.Eval_error.t) result

(** [ex_4_17 ()] runs the demonstration the statement asks for and
    answers its observable outcomes as printed strings, in the order
    the statement raises them. *)
val ex_4_17 : unit -> string list
