(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 5.1 *)

module Eval_error = Sicp_common.Eval_error

(** The register-machine language of sections 5.1 and 5.2 and its
    simulator (grammar sections 10 and 13).

    A controller is an ordinary OCaml list of typed instructions; there
    is no controller text.  The machine is polymorphic in its word: the
    machines of 5.1 and 5.2 hold [value]s, 5.3 holds tagged memory
    words, and the explicit-control evaluator and the compiler of 5.4
    and 5.5 hold evaluator words.  One simulator runs them all: it
    assembles a controller into an instruction array and a label table,
    rejects unknown registers, operations, and labels and duplicate
    labels before execution, and then executes over an array register
    file, a program-counter ref, a test flag, and a save/restore stack
    with the monitors of 5.2.4. *)

type error = Eval_error.t

(** One operand of an instruction: a constant word, a register's
    contents, or a label as a first-class word (5.1.3). *)
type 'w source =
  | Const of 'w
  | Reg of string
  | Label_ref of string

(** One controller entry.  [Label name] marks the next instruction's
    address and executes nothing. *)
type 'w instruction =
  | Label of string
  | Assign of string * 'w source
  | Assign_op of string * string * 'w source list
  (** [Assign_op (target, operation, inputs)]. *)
  | Test of string * 'w source list
  | Branch of string
  | Goto of string
  | Goto_reg of string
  | Save of string
  | Restore of string
  | Perform of string * 'w source list

(** A machine operation.  A [Value_op] computes a word for
    [Assign_op]; a [Test_op] answers the condition a [Test] sets; an
    [Action_op] is performed for its effect. *)
type 'w op =
  | Value_op of ('w list -> ('w, error) result)
  | Test_op of ('w list -> (bool, error) result)
  | Action_op of ('w list -> (unit, error) result)

(** How a machine handles its words: [label] makes the word a label
    source denotes, [to_label] reads the label a [Goto_reg] register
    holds, [show] renders a word for traces, and [unassigned] is the
    contents of a register never assigned. *)
type 'w words =
  { label : string -> 'w
  ; to_label : 'w -> string option
  ; show : 'w -> string
  ; unassigned : 'w
  }

(** An assembled controller: the executable instructions (labels
    removed) and each label with the index of the instruction it names. *)
type 'w program =
  { code : 'w instruction array
  ; labels : (string * int) list
  }

(** [assemble controller] extracts the labels of [controller]; a label
    defined twice is an error (exercise 5.8's resolution). *)
val assemble : 'w instruction list -> ('w program, error) result

(** [instruction_registers i] is every register [i] names, in order. *)
val instruction_registers : 'w instruction -> string list

(** [instruction_to_string show i] renders [i] in constructor notation,
    words through [show]. *)
val instruction_to_string : ('w -> string) -> 'w instruction -> string

(** One assembled machine. *)
type 'w machine

(** [make ~words ~registers ~operations controller] assembles
    [controller] and checks it against the declared registers and
    operations: every named register is declared, every operation is
    installed with the kind its instruction requires, and every label
    referenced is defined. *)
val make
  :  words:'w words
  -> registers:string list
  -> operations:(string * 'w op) list
  -> 'w instruction list
  -> ('w machine, error) result

(** [set_register m r w] loads [r]. *)
val set_register : 'w machine -> string -> 'w -> (unit, error) result

(** [get_register m r] reads [r]. *)
val get_register : 'w machine -> string -> ('w, error) result

(** [registers m] is the declared register names in declaration order. *)
val registers : 'w machine -> string list

(** [program m] is the assembled controller. *)
val program : 'w machine -> 'w program

(** [pc m] is the index of the next instruction; the length of the code
    array means the machine has stopped. *)
val pc : 'w machine -> int

(** [step m] executes one instruction; [Ok false] means the controller
    has run off its end. *)
val step : 'w machine -> (bool, error) result

(** [start m] steps from the current program counter to the end. *)
val start : 'w machine -> (unit, error) result

(** [goto_label m l] sets the program counter to the instruction [l]
    names, so a driver can enter a controller at one of its labels. *)
val goto_label : 'w machine -> string -> (unit, error) result

(** [restart m] resets the program counter, flag, and stack. *)
val restart : 'w machine -> unit

(** {1 The monitors of 5.2.4} *)

(** [initialize_stack m] empties the stack and zeroes its monitors. *)
val initialize_stack : 'w machine -> unit

(** [stack_statistics m] is [(total_pushes, maximum_depth)] since the
    last [initialize_stack]. *)
val stack_statistics : 'w machine -> int * int

(** [stack_depth m] is the number of saved words. *)
val stack_depth : 'w machine -> int

(** [executed m] is the number of instructions executed since the
    machine was made or restarted. *)
val executed : 'w machine -> int

(** {1 The machines of 5.1 and 5.2} *)

(** The word of the 5.1 and 5.2 machines. *)
type value =
  | Int of int
  | Float of float
  | Bool of bool
  | Str of string
  | Addr of string (** A label held as data. *)
  | Unassigned

(** [value_to_string v] renders [v]: numbers in OCaml syntax, strings
    quoted, labels by name. *)
val value_to_string : value -> string

(** [value_words] is how the 5.1 machines handle [value]s. *)
val value_words : value words

(** [arith_operations] is the operation table of the 5.1 machines:
    [+ - * / rem] on integers, [+. -. *. /.] on floats, the tests
    [= < > <= >=] on two integers or two floats, [abs], [sqrt], and
    [float_of_int]. *)
val arith_operations : (string * value op) list

(** [print_operation emit] is the [print] action of 5.1.1: it writes the
    rendered word and a newline through [emit]. *)
val print_operation : (string -> unit) -> string * value op

(** [make_machine ~registers ~operations ~controller] is [make] over
    [value_words]. *)
val make_machine
  :  registers:string list
  -> operations:(string * value op) list
  -> controller:value instruction list
  -> (value machine, error) result

(** [run ~registers ~operations ~inputs ~controller result] makes the
    machine, loads [inputs], runs it, and reads [result]. *)
val run
  :  registers:string list
  -> operations:(string * value op) list
  -> inputs:(string * value) list
  -> controller:value instruction list
  -> string
  -> (value, error) result
