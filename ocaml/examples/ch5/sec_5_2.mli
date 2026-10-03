(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 5.2 *)

(** The section 5.2 machines: the [Sec_5_1] simulator over [value]
    words with the section's own operations -- the stack monitors of
    5.2.4 ([initialize-stack], [print-stack-statistics]) and [print] --
    writing into a transcript, and the reader that runs a register
    machine fixture of [spec/host-subsets/ocaml/programs/machine]. *)

type error = Sec_5_1.error

(** The machine word, [Sec_5_1.value]. *)
type value = Sec_5_1.value =
  | Int of int
  | Float of float
  | Bool of bool
  | Str of string
  | Addr of string
  | Unassigned

(** A 5.2 machine. *)
type machine

(** [make_machine ~registers ~operations ~controller] assembles a
    machine whose table is [operations] plus the section's
    [initialize-stack], [print-stack-statistics], and [print]; those
    write into the machine's transcript. *)
val make_machine
  :  registers:string list
  -> operations:(string * value Sec_5_1.op) list
  -> controller:value Sec_5_1.instruction list
  -> (machine, error) result

(** [simulator m] is the underlying simulator, for drivers that step it. *)
val simulator : machine -> value Sec_5_1.machine

val set_register : machine -> string -> value -> (unit, error) result
val get_register : machine -> string -> (value, error) result
val start : machine -> (unit, error) result

(** [transcript m] is every line the machine's actions wrote, in order. *)
val transcript : machine -> string list

(** [print_stack_statistics m] is the monitor line of 5.2.4:
    [total-pushes = P maximum-depth = D]. *)
val print_stack_statistics : machine -> string

(** An operation's declared operand or result type in a fixture. *)
type op_type =
  | Int_type
  | Float_type
  | Bool_type
  | Unit_type

(** A register machine fixture: its registers, the operations it
    declares with their types, its typed test inputs, and its
    controller. *)
type fixture =
  { registers : string list
  ; operations : (string * op_type list * op_type) list
  ; inputs : (string * value) list
  ; controller : value Sec_5_1.instruction list
  }

(** [read_fixture ~filename text] decodes the constructor-literal
    fixture [text]: [Machine { registers; operations; inputs;
    controller }].  Every declared operation must exist in
    [Sec_5_1.arith_operations] or be [print], with a result type that
    matches its kind (a [Bool_type] result is a test, [Unit_type] an
    action). *)
val read_fixture : filename:string -> string -> (fixture, string) result

(** [run_fixture ~emit fixture] runs [fixture]: [print] writes its
    operand and a newline, each declared operation checks its operand
    types, and after the controller stops each declared register is
    written as [name: value], in declaration order. *)
val run_fixture : emit:(string -> unit) -> fixture -> (unit, error) result
