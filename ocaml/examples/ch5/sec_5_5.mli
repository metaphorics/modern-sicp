(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.5 *)

(** The compiler of section 5.5 and the 5.5.7 evaluator machine.  See
    the module implementation for the design and the two spelling
    deviations from the book's listings. *)

module Ast = Sicp_common.Ast
module Value = Sicp_common.Value

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

(** {1 Instruction sequences} *)

(** One instruction sequence: the registers the code needs, the
    registers it modifies, and the statements -- controller lines in
    the book's notation. *)
type seq =
  { needs : string list
  ; modifies : string list
  ; stmts : string list
  }

val make_instruction_sequence : string list -> string list -> string list -> seq
val empty_instruction_sequence : seq
val append_2_sequences : seq -> seq -> seq
val append_sequences : seq list -> seq
val tack_on_instruction_sequence : seq -> seq -> seq
val parallel_instruction_sequences : seq -> seq -> seq

(** {1 The compiler's state} *)

(** The compiler's state: the book's label counter and the compile-time
    constant table behind the [(const compile-time-constant-N)] spellings. *)
type state

val new_state : unit -> state

(** [new_state_seeded n] starts the label counter at [n]: 5.35's
    reproduction of Figure 5.18 seeds 14, the labels the book's session
    had already generated. *)
val new_state_seeded : int -> state

val make_label : state -> string -> string

(** [register_const state v] is the fresh [(const %kN)] name of the
    compile-time constant [v]. *)
val register_const : state -> Value.t -> string

(** {1 Targets, linkages, compile-time environments} *)

type linkage =
  | Next
  | Return
  | Lab of string

(** The compile-time environment: the frames of parameter names, newest
    first; the empty list is the top level. *)
type cenv = string list list

val extend_cenv : string list -> cenv -> cenv

(** [find_variable var cenv] is the lexical address [(frame,
    displacement)] or [None] past the global frame. *)
val find_variable : string -> cenv -> (int * int) option

(** {1 The configuration and the code generators} *)

type config =
  { lexical : bool (** 5.40 to 5.42: emit lexical-address accesses. *)
  ; scan_out : bool (** 5.43: scan internal definitions out of bodies. *)
  ; open_code : bool (** 5.38 and 5.44: open-code the named primitives. *)
  ; left_to_right : bool (** 5.36: evaluate operands left to right. *)
  ; preserving_on : bool (** 5.37: the preserving mechanism itself. *)
  ; compound_calls : bool (** 5.47: compiled code may call interpreted procedures. *)
  ; trace : (string list list -> string -> unit) option
    (** 5.40: the compile-time environment dump. *)
  }

val default_config : config
val open_coded_primitives : string list
val cond_to_if : Ast.expr -> (Ast.expr, error) result
val let_to_combination : Ast.expr -> (Ast.expr, error) result

(** [compile cfg state cenv exp target linkage] is the expression's
    instruction sequence, the book's [compile]. *)
val compile
  :  config
  -> state
  -> cenv
  -> Ast.expr
  -> string
  -> linkage
  -> (seq, error) result

(** [compile_program cfg state forms] is the whole program: every
    top-level form compiled into [val] with linkage [next]. *)
val compile_program
  :  ?cfg:config
  -> ?linkage:linkage
  -> state
  -> Ast.expr list
  -> (seq, error) result

val statements_text : seq -> string
val registered_constants : state -> (string * Value.t) list

(** {1 The 5.5.7 machine} *)

(** One machine operation: the 5.4 op type, re-exported. *)
type op = Sec_5_4.op =
  | Value_op of (Sec_5_4.word list -> (Sec_5_4.word, error) result)
  | Action_op of (Sec_5_4.word list -> (unit, error) result)

(** One built machine. *)
type machine

val machine_registers : string list

(** The controller fragments of 5.5.7: the compiled apply-dispatch with
    the [compiled-apply] entry, and the external entry block. *)
val apply_dispatch_compiled : string

val external_entry_block : string

(** [eceval_fragments] is the 5.5.7 controller in named fragments; a
    variant machine replaces one and concatenates. *)
val eceval_fragments : (string * string) list

(** [eceval_controller] is the fragments concatenated. *)
val eceval_controller : string

(** [make_compiled_evaluator ~source ~state ()] builds the machine: the
    [eceval_controller] unless a variant is given, the 5.4 base
    operations, the compiled operations, and any extras last; the
    global environment binds [true], [false], the runtime primitives,
    and the [globals] primitives (also visible to
    [apply-primitive-procedure]); the source fills the driver's input
    queue; [flag] starts false. *)
val make_compiled_evaluator
  :  ?controller:string
  -> ?operations:(string * Sec_5_4.op) list
  -> ?globals:(string * Sicp_common.Value.primitive) list
  -> source:string
  -> state:state
  -> unit
  -> (machine, error) result

(** [compile_block state forms] is a fresh entry label and the
    controller block of the compiled forms. *)
val compile_block : ?cfg:config -> state -> string -> (string * string, error) result

(** [compile_and_go ~state ~compiled ~source ()] compiles [compiled],
    appends it to the machine's controller, and arms the external
    entry; the driver's inputs are [source]. *)
val compile_and_go
  :  ?cfg:config
  -> state:state
  -> compiled:string
  -> source:string
  -> unit
  -> (machine, error) result

(** [build_runtime_primitives ()] is the object-language primitive
    table of a compiled machine (the environment procedures behind id
    symbols included); one call per machine. *)
val build_runtime_primitives : unit -> (string * Value.primitive) list

(** [set_flag m b] arms the external entry. *)
val set_flag : machine -> bool -> unit

val set_register : machine -> string -> Sec_5_4.word -> (unit, error) result
val get_register : machine -> string -> (Sec_5_4.word, error) result

(** [start m] runs the controller from the first instruction until a
    failure; the driver's queue-dry stop is the typed [Op_failed] whose
    message is [Sec_5_4.input_exhausted]. *)
val start : machine -> (unit, error) result

val transcript : machine -> string list

(** [step_count m] is the number of instructions the machine executed. *)
val step_count : machine -> int

(** [start_upto m limit] runs at most [limit] instructions -- the
    debugging harness of a controller loop. *)
val start_upto : machine -> int -> (unit, error) result

(** [instruction_text m i] is the [i]th instruction as read. *)
val instruction_text : machine -> int -> string

(** [current_pc m] is the program counter. *)
val current_pc : machine -> int

val print_stack_statistics : machine -> string
