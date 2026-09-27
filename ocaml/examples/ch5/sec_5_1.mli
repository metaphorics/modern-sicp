(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** The register-machine substrate of section 5.1: the machine values,
    the instruction language of 5.1.5, and the simulator core the rest
    of chapter 5 builds on. Machine descriptions are data in the book's
    register-machine language, parsed by the shared reader; the machine
    and state types are open to 5.2's simulator elaboration, 5.3's
    vector memory, and 5.4's explicit-control evaluator. *)

(** One machine value: a register or stack entry's contents. [Label] is
    the first-class entry point of 5.1.3; [Symbol] is the constant kind
    5.1.5 announces beyond numbers; 5.3 adds pairs in vector memory. *)
type value =
  | Int of int
  | Float of float
  | Bool of bool
  | Symbol of string
  | Label of string

(** [value_to_string v] renders [v] the way the machines print it. *)
val value_to_string : value -> string

(** [equal_value a b] holds for values the machine's [=] test equates:
    numbers compare across the int/float boundary, everything else by
    kind and content. *)
val equal_value : value -> value -> bool

(** Every failure of the substrate; nothing raises. *)
type error =
  | Parse of string (** The controller text is not well-formed machine language. *)
  | Unknown_register of string
  | Unknown_operation of string
  | Unknown_label of string
  | Bad_instruction of string
  (** Well-read text that the grammar of 5.1.5 does not allow. *)
  | Arity of string (** An operation refused its arguments. *)
  | Op_failed of string
  (** An operation refused the computation itself: a division by
          zero, an exhausted input. *)
  | Stack_underflow of string (** A restore with an empty stack, naming the register. *)
  | Branch_without_test (** A branch with no preceding test. *)

(** [error_to_string e] renders [e] for a transcript or a test. *)
val error_to_string : error -> string

(** One source of a value: a register, a constant, or -- the 5.1.3
    extension -- a label read as a special constant. *)
type source =
  | Reg of string
  | Const of value
  | Label_source of string

(** One controller instruction of 5.1.5. [Assign] is [assign] from a
    single source; [Assign_op] is [assign] from an operation applied to
    its inputs. *)
type instruction =
  | Assign of string * source
  | Assign_op of string * string * source list
  | Test of string * source list
  | Branch of string
  | Goto_label of string
  | Goto_reg of string
  | Perform of string * source list
  | Save of string
  | Restore of string

(** [instruction_to_string i] renders [i] back in the book's notation. *)
val instruction_to_string : instruction -> string

(** [source_to_string s] renders one source in the book's notation. *)
val source_to_string : source -> string

(** A parsed controller: the instructions in order, and each label with
    the position of the instruction it names. 5.2's assembler supersedes
    this view; 5.1's hand simulations step over the same [code]. *)
type program =
  { code : instruction array
  ; labels : (string * int) list
  }

(** [parse_program text] reads one [(controller ...)] form from [text]
    with the shared reader and resolves its labels; no label may be used
    twice. *)
val parse_program : string -> (program, error) result

(** The operations table of a machine. A [Value_op] computes a value for
    an [assign] or a [test]; an [Action_op] is the 5.1.1 notion of an
    action -- [print] -- pushed by [perform] and producing no value. *)
type op =
  | Value_op of (value list -> (value, error) result)
  | Action_op of (value list -> (unit, error) result)

(** One assembled machine: its registers, operations, labels, and code.
    The record is abstract; 5.3 trades the register file for vector
    memory behind this same surface. *)
type machine

(** [make_machine ~registers ~operations controller] assembles the
    controller text and checks it against the declared registers and
    operations before the machine can start. *)
val make_machine
  :  registers:string list
  -> operations:(string * op) list
  -> controller:string
  -> (machine, error) result

(** [set_register m r v] loads an input register before [start]. *)
val set_register : machine -> string -> value -> (unit, error) result

(** [get_register m r] reads a result register after [start]. *)
val get_register : machine -> string -> (value, error) result

(** [start m] runs the controller from its first instruction until the
    sequence ends -- the book's stop condition -- or until an
    instruction fails. *)
val start : machine -> (unit, error) result

(** [arith_operations] is the arithmetic table the 5.1 machines name in
    their [(op ...)]: the remainder device, the comparisons, and the
    arithmetic the elaborated machines and 5.3 expand into. *)
val arith_operations : (string * op) list

(** [read_print ~inputs ~output] is the operation pair of 5.1.1's
    Actions: [read] consumes the queue of machine values, [print]
    appends the rendered value. [read] fails with a typed error when
    the queue is exhausted, which stops a driver-loop machine. *)
val read_print : inputs:value Queue.t -> output:string list ref -> (string * op) list
