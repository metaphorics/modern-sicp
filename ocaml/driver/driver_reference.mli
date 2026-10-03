(* SPDX-License-Identifier: GPL-3.0-only *)

(** Independent finite reference models for the named experiments and
    domain fixtures of grammar section 10.  Each model executes the
    artifact with its own evaluator, matcher, and state; none calls a
    teaching engine.  Lazy: compound arguments and named [let]
    right-hand sides are delayed and memoized, primitives are strict,
    and the counts follow the guest output.  Search: depth-first over
    choice points, left alternative first, failed branches leave no
    output, and the answer, choice, and failure counts follow.  Query:
    assertions before rules, conjuncts in series, disjuncts
    interleaved.  Machine: the controller's instructions over explicit
    registers and stack, then one [name: value] line per register. *)

(** [run ~emit ~capability path] runs the model of [capability]
    ([lazy], [amb], [query], or [machine]) on the artifact at [path],
    writing its transcript through [emit]. *)
val run : emit:(string -> unit) -> capability:string -> string -> (unit, string) result
