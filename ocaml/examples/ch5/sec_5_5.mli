(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme programs in SICP section 5.5 *)

module Ast = Sicp_common.Ast
module Check = Sicp_common.Check
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

(** The section 5.5 compiler: checked host-subset programs compiled to
    typed instruction sequences of [Sec_5_1] over the explicit-control
    evaluator's words, so compiled code and the [Sec_5_4] machine share
    one register set and one operation vocabulary (grammar section 13,
    row 5.5).

    An instruction sequence carries the registers it needs and modifies;
    [preserving] reads those sets, never the code.  Operands are
    evaluated left to right; binary operators are open-coded into
    [arg1] and [arg2]; [let] compiles as the application of a [fun]
    (the book's derived form) and [let rec] as cell allocation followed
    by in-order filling (the 5.43 scan-out); a [match] tries its
    patterns in order.  Compiled procedures are [Value.Compiled] values
    whose entry label names their code; calls whose operand count
    equals the procedure's parameter count jump straight to the entry,
    and every other call goes through the shared [compiled-apply]
    routine, which handles partial application, over-application, and
    primitives. *)

(** A compiled instruction sequence. *)
type seq =
  { needs : string list
  ; modifies : string list
  ; statements : Sec_5_4.word Sec_5_1.instruction list
  }

(** Where compiled code continues. *)
type linkage =
  | Next
  | Return
  | Goto_label of string

(** Compiler state: the label counter. *)
type state

val new_state : unit -> state
val make_label : state -> string -> string

val make_instruction_sequence
  :  string list
  -> string list
  -> Sec_5_4.word Sec_5_1.instruction list
  -> seq

val empty_instruction_sequence : seq
val append_sequences : seq list -> seq
val tack_on_instruction_sequence : seq -> seq -> seq
val parallel_instruction_sequences : seq -> seq -> seq

(** [preserving regs first second] runs [first] then [second], saving
    around [first] each register of [regs] that [first] modifies and
    [second] needs. *)
val preserving : string list -> seq -> seq -> seq

(** [compile state e target linkage] compiles [e] to leave its value in
    [target] and continue at [linkage]. *)
val compile : state -> Ast.expr -> string -> linkage -> seq

(** [compile_open ?preserving ~self state e target linkage] is one
    dispatch step of [compile]: every subexpression [e] contains --
    [fun] bodies, [let] and [let rec] right-hand sides, [match]
    scrutinees and case bodies, call operators and operands, the
    operands of open-coded operators -- compiles through [self], and
    every preservation the step makes goes through [preserving]
    (default {!preserving}).  [compile] is the fixed point of
    [compile_open ~self:compile]. *)
val compile_open
  :  ?preserving:(string list -> seq -> seq -> seq)
  -> self:(state -> Ast.expr -> string -> linkage -> seq)
  -> state
  -> Ast.expr
  -> string
  -> linkage
  -> seq

(** [compile_procedure_call state target linkage] is the call of the
    procedure in [proc] on the arguments in [argl]: the
    [primitive-exact?] dispatch between the in-line primitive and the
    [compiled-apply] routine, answering into [target] and continuing at
    [linkage] -- exactly what [compile] emits for a call after
    constructing its argument list. *)
val compile_procedure_call : state -> string -> linkage -> seq

(** [compile_program state items] compiles a whole unit: each top-level
    binding extends the environment of the items after it; the code
    ends at the label [done], after the shared runtime routines. *)
val compile_program : state -> Ast.item list -> seq

(** [compile_program_with ~compile state items] is [compile_program]
    with every right-hand side of [items] compiled by [compile]; the
    runtime routines and the final [done] label are unchanged. *)
val compile_program_with
  :  compile:(state -> Ast.expr -> string -> linkage -> seq)
  -> state
  -> Ast.item list
  -> seq

(** [compiled_registers] is the register set of compiled code. *)
val compiled_registers : string list

(** [runtime_operations ~apply] is the operation table of compiled code:
    the evaluator's table plus the compiled-procedure operations. *)
val runtime_operations : apply:Value.apply_fun -> (string * Sec_5_4.word Sec_5_1.op) list

(** [load ?operations ~emit seq] assembles [seq] over [operations]
    (taking precedence) and [runtime_operations], with the global
    environment in [env] and the program counter at the first
    instruction; guest printing writes through [emit]. *)
val load
  :  ?operations:(string * Sec_5_4.word Sec_5_1.op) list
  -> emit:(string -> unit)
  -> seq
  -> (Sec_5_4.word Sec_5_1.machine, Eval_error.t) result

(** [statements_text seq] renders the statements, one per line. *)
val statements_text : seq -> string

(** [run ~emit program] compiles [program], assembles its code, and runs
    it from the first instruction with the global environment in [env];
    guest printing writes through [emit]. *)
val run : emit:(string -> unit) -> Check.program -> (Value.t, Eval_error.t) result

(** [run_stats ~emit program] is [run] with the number of instructions
    the machine executed. *)
val run_stats
  :  emit:(string -> unit)
  -> Check.program
  -> (Value.t * int, Eval_error.t) result
