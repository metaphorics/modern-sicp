(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.17 *)

(** Exercise 4.17: the extra frame of scanned-out definitions.

    When a call runs a body whose internal definitions live in the call
    frame, the procedure's parameters and its internal names share one
    frame.  Scanning out replaces the definitions by a [let], and the
    [let] makes a second frame below the call frame.  The extra frame
    never changes a correct program's behavior: every name is still
    found, one frame further in, and nothing else can see the frame.

    Simultaneous scope needs no extra frame: the call can allocate the
    cells of the parameter and of every internal name in its one frame
    before any right-hand side runs, then fill them in order.  That is
    the [Shared_frame] strategy. *)

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

(** [program] is the measured source: a procedure [f] with two internal
    definitions, called once. *)
val program : string

(** [ex_4_17 ()] answers the value of [program] and its frame count
    under each strategy. *)
val ex_4_17 : unit -> string list
