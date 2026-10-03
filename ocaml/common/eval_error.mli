(* SPDX-License-Identifier: GPL-3.0-only *)

(** The evaluator's closed runtime error, shared by the [Ast] smart
    constructors, the [Env] operations, and every [Value] primitive: all of
    them report recoverable failures through [( _, t) result]. *)

type t =
  | Unbound_variable of string
  (** [Unbound_variable name] is a lookup of [name] that no frame
          binds. *)
  | Arity_mismatch of
      { expected : int
      ; given : int
      }
  (** [Arity_mismatch { expected; given }] is a call or binding that
          wanted [expected] operands and got [given]. *)
  | Type_error of string
  (** [Type_error detail] is an operand of the wrong kind; [detail]
          names the operation and the offending value. *)
  | Not_applicable of string
  (** [Not_applicable printed] is an application whose operator is the
      value printed as [printed], not a procedure. *)
  | Division_by_zero (** [Division_by_zero] is an object-language division by zero. *)
  | Unknown_operation of string
  (** [Unknown_operation name] is a machine operation [name] names
      that the machine's operation table does not install. *)
  | Unknown_label of string
  (** [Unknown_label name] is a jump to [name], which labels no
      controller instruction. *)
  | Bad_instruction of string
  (** [Bad_instruction detail] is a controller instruction whose shape
      or operands the machine cannot execute; [detail] says why. *)
  | Branch_without_test
  (** [Branch_without_test] is a [branch] executed before any [test]. *)
  | Unknown_register of string
  (** [Unknown_register name] is a machine instruction or access naming
      a register the machine does not declare. *)
  | Invalid_form of string
  (** [Invalid_form detail] is a syntactically complete form whose
      shape the subset does not allow; [detail] says why. *)
  | Bounds_error of string
  (** [Bounds_error detail] is an array or bounds failure; [detail]
      names the operation, index, and length. *)
  | User_error of string
  (** [User_error message] is the object program calling its [error]
          primitive with [message] already rendered from the message and
          the irritants. *)

(** [pp ppf e] prints [e] as one short sentence, subsystem and position
    included where known. *)
val pp : Format.formatter -> t -> unit

(** [to_string e] is [pp] rendered to a string. *)
val to_string : t -> string
