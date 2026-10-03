(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.21 *)

(** Exercise 4.21: recursion without a recursive binding.

    A procedure that receives itself as an argument can recurse without
    [let rec].  In typed OCaml the bare self-application [ft ft] has no
    type: [ft] would need a type equal to a function from that same type,
    and the pinned type checker rejects the program.  Wrapping the
    procedure in a constructor of a recursive variant, [Fix], gives the
    self-application a type; [self_apply] unwraps it.  Nothing is bound
    recursively except the type. *)

(** [untyped_factorial] is the book's program transcribed directly, with
    the bare self-application. *)
val untyped_factorial : string

(** [factorial] computes the factorial of 10 through [Fix]. *)
val factorial : string

(** [fibonacci] computes the tenth Fibonacci number the same way (part
    a). *)
val fibonacci : string

(** [parity] is the book's [f] of part (b), [even] and [odd] passing each
    other along, wrapped in the recursive variant [test]. *)
val parity : string

(** [ex_4_21 ()] is the transcript of each program under the direct
    evaluator: the rejected transcription, then the three typed
    programs. *)
val ex_4_21 : unit -> string list
