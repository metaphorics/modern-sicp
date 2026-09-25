(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** The explicit-control evaluator of section 5.4: the book's register
    machine over machine words, its controller text in the book's
    notation, and the typed syntax, environment, and primitive
    operations of 4.1 it runs on. *)

(** One machine word: what a register or a stack entry holds.  [V]
    wraps an object-language value; [Exp] and [Seq] hold the typed
    expressions of the shared AST; [Args] is [argl]'s accumulated
    operand words; [Env] an environment; [Lab] a return address.  The
    last two are exercise words the base controller never builds:
    [Clause] is 5.24's cond clause, [Thunk] is 5.25's delayed operand. *)
type word =
  | V of Sicp_common.Value.t
  | Exp of Sicp_common.Ast.expr
  | Seq of Sicp_common.Ast.expr list
  | Args of word list
  | Env of Sicp_common.Value.env
  | Lab of string
  | Clause of Sicp_common.Ast.expr option * Sicp_common.Ast.expr list
  | Thunk of Sicp_common.Ast.expr * Sicp_common.Value.env

(** [word_to_string w] renders a word for a transcript or a test:
    values as the printer prints them, the book's labels bare, the
    internal shapes by name. *)
val word_to_string : word -> string

(** Every failure of the substrate travels through [error]; the type is
    the 5.1 substrate's, re-exported. *)
type error = Sec_5_1.error =
  | Parse of string
  | Unknown_register of string
  | Unknown_operation of string
  | Unknown_label of string
  | Bad_instruction of string
  | Arity of string
  | Op_failed of string
  | Stack_underflow of string
  | Branch_without_test

val error_to_string : error -> string

(** The message whose [Op_failed] ends the driver loop when the input
    queue runs dry: the edition's stop for the book's unbounded
    read-eval-print loop. *)
val input_exhausted : string

(** One operation of the evaluator machine: a [Value_op] computes a
    word for an [assign] or a [test]; an [Action_op] is an action under
    [perform]. *)
type op =
  | Value_op of (word list -> (word, error) result)
  | Action_op of (word list -> (unit, error) result)

(** One built evaluator machine. *)
type machine

(** The evaluator's registers: the book's seven -- [exp], [env], [val],
    [continue], [proc], [argl], [unev] -- plus [flag], which every
    [test] sets. *)
val evaluator_registers : string list

(** [base_controller] is the book's evaluator assembled: the driver
    loop, [eval-dispatch] and the [ev-] entries of 5.4.1 to 5.4.3, and
    the error entries of 5.4.4. *)
val base_controller : string

(** The controller fragments of the base evaluator, in printed order,
    each with the book's name for its block.  An exercise composes a
    variant by keeping the fragments it needs, replacing some, and
    appending its own. *)
val controller_fragments : (string * string) list

(** [eval_error e] renders a shared evaluator failure into the
    substrate's error channel. *)
val eval_error : Sicp_common.Eval_error.t -> error

(** [expr_word r] is the word of the expression result [r]: the shared
    smart constructors' failures are rendered into the substrate's
    error channel, the expression wrapped as an [Exp] word. *)
val expr_word
  :  (Sicp_common.Ast.expr, Sicp_common.Eval_error.t) result
  -> (word, error) result

(** [variable_name e] is the name of the variable expression [e]. *)
val variable_name : Sicp_common.Ast.expr -> (string, error) result

(** [apply_object_primitive name args] applies the object-language
    primitive [name] to [args]; the error-signaling exercise wraps its
    failures in condition codes. *)
val apply_object_primitive
  :  string
  -> Sicp_common.Value.t list
  -> (Sicp_common.Value.t, error) result

(** [word_values ws] is the values of every [V] word in [ws]. *)
val word_values : word list -> (Sicp_common.Value.t list, error) result

(** [result_all rs] collects the results [rs]. *)
val result_all : ('a, error) result list -> ('a list, error) result

(** [parameters_of w] is the names of the parameter list [w], the
    [Seq] of variable expressions a lambda's parameter list is. *)
val parameters_of : word -> (string list, error) result

(** [base_operations] is the operations table of 4.1 typed over words:
    the syntax predicates and selectors, the argument-list procedures,
    the environment procedures, [make-procedure],
    [apply-primitive-procedure], and [true?]. *)
val base_operations : (string * op) list

(** [make_evaluator ~controller ~operations ~source ()] builds the
    evaluator machine: [controller] defaults to [base_controller]; the
    extra [operations] install last so they override the base on a
    name collision; [source] is the object program, read into the
    driver's input queue; the global environment binds [true],
    [false], and the object-language primitives.  An unknown operation
    in the controller or an unreadable source is a typed failure
    before the machine can start. *)
val make_evaluator
  :  ?controller:string
  -> ?operations:(string * op) list
  -> source:string
  -> unit
  -> (machine, error) result

(** [set_register m r w] loads a register before [start]; [get_register]
    reads one after the machine stops. *)
val set_register : machine -> string -> word -> (unit, error) result

val get_register : machine -> string -> (word, error) result

(** [start m] runs the controller from the first instruction until a
    failure.  The driver loop's normal end is the typed [Op_failed]
    whose message is [input_exhausted]; [signal-error] stops the
    machine with the object-level message instead. *)
val start : machine -> (unit, error) result

(** [transcript m] is what the driver printed, in order: the prompts,
    the printed values, and any stack statistics. *)
val transcript : machine -> string list

(** [print_stack_statistics m] renders the monitored stack's counters,
    [(total-pushes = N maximum-depth = M)], the numbers 5.26 to 5.29
    measure. *)
val print_stack_statistics : machine -> string
