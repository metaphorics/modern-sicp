(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 5.5 *)

(** Exercise 5.39: the lexical-address lookup operation.

    The edition's run-time environment is one chain of bindings, newest
    first; each frame the compiled code pushes is a run of adjacent
    bindings in its declared order.  A lexical address [(frame,
    displacement)] therefore names the binding at the position that
    skips the sizes of the frames before [frame], which the compiler
    knows.  The operation reads that binding, checks that it binds the
    expected name, and signals an error when the binding is a [let rec]
    cell not yet filled, the edition's unassigned value.

    The exercise's [lexical-address-set!] has no counterpart: bindings
    are immutable (grammar section 4), mutation goes through references
    whose cells the lookup answers, and the only write to a binding is
    the filling of a [let rec] cell, which the compiled code performs on
    the group's own cells. *)

(** [position frames address] is the run-time position of [address] in
    an environment whose innermost frames are [frames]. *)
val position : string list list -> int * int -> int

(** [lexical_address_lookup ~frame ~displacement ~position ~name env]
    is the value of the binding of [name] at [position] of [env], the
    address [(frame, displacement)]. *)
val lexical_address_lookup
  :  frame:int
  -> displacement:int
  -> position:int
  -> name:string
  -> Sicp_common.Env.t
  -> (Sicp_common.Value.t, Sicp_common.Eval_error.t) result

(** [operation] is the [lexical-address-lookup] machine operation.  Its
    operands are the address as a [(frame, displacement)] constant, the
    position, the variable, and the environment. *)
val operation : string * Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.op

(** [lookup_instruction frames address name target] is the instruction
    that loads [target] from [address] of [name] in [frames]. *)
val lookup_instruction
  :  string list list
  -> int * int
  -> string
  -> string
  -> Sicp_ch5.Sec_5_4.word Sicp_ch5.Sec_5_1.instruction

(** [ex_5_39 ()] looks addresses up in a run-time environment shaped
    like the compile-time environment of exercise 5.41, and reads a
    [let rec] cell before it is filled. *)
val ex_5_39 : unit -> (string list, Sicp_common.Eval_error.t) result
