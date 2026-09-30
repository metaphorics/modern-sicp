(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.10: a new syntax for the machine language, isolated in
    its own syntax procedures. The simulator is untouched: [syntax]
    translates the new statements into the instructions it already
    runs. *)

(** A term of the new syntax: a register, a number, a label, or an
    operation call. A call is a term only on the right of [Set]. *)
type term =
  | R of string
  | N of int
  | L of string
  | Call of string * term list

(** A statement of the new syntax, one per instruction: [Name] marks a
    label, [Set] assigns a term, [Check] tests, [Jump_if] branches,
    [Jump] goes to a label or through a register, [Push] and [Pop] save
    and restore, and [Do] performs an action. *)
type stmt =
  | Name of string
  | Set of string * term
  | Check of string * term list
  | Jump_if of string
  | Jump of term
  | Push of string
  | Pop of string
  | Do of string * term list

(** [syntax stmts] is the controller [stmts] denotes, one instruction
    per statement, or a [Bad_instruction] for a call in an operand
    position or a jump to a number or a call. *)
val syntax
  :  stmt list
  -> ( Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
       , Sicp_ch5.Sec_5_2.error )
       result

(** The GCD machine of 5.1.1 in the simulator's own syntax: registers
    [a], [b], and [t]. *)
val gcd_controller : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** The same GCD machine written in the new syntax. *)
val gcd_new_syntax : stmt list

(** [ex_5_10 ()] runs the GCD machine written in both syntaxes on
    (12, 8) and shows the third instruction of each: the same typed
    instruction. *)
val ex_5_10 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
