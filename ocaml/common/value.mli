(* SPDX-License-Identifier: GPL-3.0-only *)

(** The dynamically typed values of the Scheme subset: the runtime data of
    the chapter 4 evaluator, the registers of the chapter 5 simulator, and
    the operands the chapter 5 compiler hands to the shared [apply-dispatch].

    [Value.t] and the environment type [Value.env] are mutually recursive
    through compound procedures, so both live in this module; the public
    environment interface is [Env], whose [Env.t] is [Value.env]. *)

(** A Scheme value. *)
type t

(** An environment: mutable frames, newest first. Constructed and mutated
    through [Env]. *)
type env

(** A primitive procedure: takes the evaluated operands and returns the
    result or an error through the one shared error channel. *)
type primitive = t list -> (t, Eval_error.t) result

(** The parts of a compound procedure that evaluation, printing, and
    compilation need. *)
type compound_view =
  { name : string option
    (** [Some name] when the procedure value was produced by [(define
            (name ...))], which the printer prints as
            [#[compound-procedure name]]. *)
  ; parameters : string list
  ; body : Ast.expr list
  ; env : env
  }

type view =
  | Int of int
  | Float of float
  | Bool of bool
  | String of string
  | Symbol of string
  | Nil
  | Pair of t * t
  | Primitive_procedure of string
  | Compound_procedure of compound_view
  (** [view v] exposes the shape of [v] without exposing the representation. *)

(** [view v] exposes the shape of [v]; a compound procedure view never
    compares structurally, because its environment is stateful. *)
val view : t -> view

(** [int n] is the exact integer [n]. *)
val int : int -> t

(** [float f] is the inexact float [f]. *)
val float : float -> t

(** [bool b] is [#t] or [#f]. *)
val bool : bool -> t

(** [string s] is the string [s]. *)
val string : string -> t

(** [symbol name] is the symbol [name]. *)
val symbol : string -> t

(** [nil] is the empty list, printed as [()]. *)
val nil : t

(** [pair car cdr] is the pair [(car . cdr)]. *)
val pair : t -> t -> t

(** [primitive ~name f] is the primitive procedure printed as
    [#[primitive-procedure name]]. *)
val primitive : name:string -> primitive -> t

(** [compound ~name ~parameters ~body ~env] is the procedure value of a
    [lambda] or a function [define]. *)
val compound
  :  name:string option
  -> parameters:string list
  -> body:Ast.expr list
  -> env:env
  -> t

(** [physical_equal a b] is the [eq?] of the subset: exact atoms compare by
    value, pairs, strings, and procedures by identity. *)
val physical_equal : t -> t -> bool

(** [structural_equal a b] is the [equal?] of the subset: recursive over
    pairs, by name over primitives, by identity over compound procedures.
    It never compares function code. *)
val structural_equal : t -> t -> bool

(** [to_string v] is the printed form of [v] under the shared printer
    contract: strings quoted, with escapes for the double quote and the
    backslash. *)
val to_string : t -> string

(** [display v] is the [display] form of [v]: like [to_string], except
    strings print raw, without quotes or escapes. *)
val display : t -> string

(** {2 Environment hooks}

    The environment representation is owned here because compound
    procedures capture it; [Env] publishes these operations under the
    project interface. Do not call them directly. *)

(** See [Env.empty]. *)
val env_empty : unit -> env

(** See [Env.extend]. *)
val env_extend : string list -> t list -> env -> (env, Eval_error.t) result

(** See [Env.find_binding]. *)
val env_find_binding : env -> string -> t option

(** See [Env.define]. *)
val env_define : env -> string -> t -> unit

(** See [Env.set]. *)
val env_set : env -> string -> t -> (unit, Eval_error.t) result
