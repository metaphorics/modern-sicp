(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** The register-machine simulator of section 5.2: the assembler that
    turns controller text into instruction objects with resolved
    labels, the execution-procedure dispatch, and the monitored stack
    of 5.2.4. The instruction ADT and the typed failures are the 5.1
    substrate's, re-exported; the machine type is an abstract mutable
    record left open to 5.3's vector memory and 5.4's evaluator. *)

type value = Sec_5_1.value =
  | Int of int
  | Float of float
  | Bool of bool
  | Symbol of string
  | Label of string

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

type source = Sec_5_1.source =
  | Reg of string
  | Const of value
  | Label_source of string

type instruction = Sec_5_1.instruction =
  | Assign of string * source
  | Assign_op of string * string * source list
  | Test of string * source list
  | Branch of string
  | Goto_label of string
  | Goto_reg of string
  | Perform of string * source list
  | Save of string
  | Restore of string

type op = Sec_5_1.op =
  | Value_op of (value list -> (value, error) result)
  | Action_op of (value list -> (unit, error) result)

(** The substrate's renderers and reader, re-exported under this
    module's names. *)
val value_to_string : value -> string

val equal_value : value -> value -> bool
val error_to_string : error -> string
val instruction_to_string : instruction -> string
val source_to_string : source -> string

(** A parsed controller: the instructions in order and each label with
    the index it names, the assembler's input. *)
type program = Sec_5_1.program =
  { code : instruction array
  ; labels : (string * int) list
  }

val parse_program : string -> (program, error) result
val arith_operations : (string * op) list

(** [instruction_registers inst] names every register [inst] reads or
    writes; the machine's own [flag] is never named. The scan-out the
    register-derivation exercise builds on. *)
val instruction_registers : instruction -> string list

(** One assembled machine. The register table always holds [flag], the
    operations list always begins with [initialize-stack] and
    [print-stack-statistics], and every label resolves to an index into
    the instruction array before [start] may run. *)
type machine

(** [make_machine ~registers ~operations ~controller] allocates the
    registers, installs the operations, and assembles the controller:
    an unknown register, operation, or label, or a register or label
    used twice, fails before the machine can start. *)
val make_machine
  :  registers:string list
  -> operations:(string * op) list
  -> controller:string
  -> (machine, error) result

(** [make_machine_from_program ~registers ~operations program] assembles
    a controller that some other syntax has already parsed into the
    typed program -- the seam the new-syntax exercise plugs into. *)
val make_machine_from_program
  :  registers:string list
  -> operations:(string * op) list
  -> program
  -> (machine, error) result

(** [set_register m r v] is [set-register-contents!]: it stores a value
    in the named register before [start]. *)
val set_register : machine -> string -> value -> (unit, error) result

(** [get_register m r] is [get-register-contents]: it reads a register
    of the stopped machine. *)
val get_register : machine -> string -> (value, error) result

(** [start m] simulates the machine from the first instruction until
    the sequence ends or an instruction fails. *)
val start : machine -> (unit, error) result

(** [print_stack_statistics m] renders the monitored stack's counters,
    [total-pushes] and [maximum-depth]. *)
val print_stack_statistics : machine -> string

(** [transcript m] is what the machine's [print-stack-statistics]
    actions have printed, in order. *)
val transcript : machine -> string list
