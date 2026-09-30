(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.9: operations cannot take labels as operands. *)

(** [check_operands controller] is [Ok ()] when every input of every
    [Assign_op], [Test], and [Perform] in [controller] is a register or
    a constant, and a [Bad_instruction] naming the first label input
    otherwise. *)
val check_operands
  :  Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
  -> (unit, Sicp_ch5.Sec_5_2.error) result

(** [make_machine ~registers ~operations ~controller] is
    [Sicp_ch5.Sec_5_2.make_machine] behind [check_operands]: a label
    operand is refused before the machine is assembled. *)
val make_machine
  :  registers:string list
  -> operations:(string * Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.op) list
  -> controller:Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
  -> (Sicp_ch5.Sec_5_2.machine, Sicp_ch5.Sec_5_2.error) result

(** [ex_5_09 ()] runs the legal operand forms and reports the typed
    refusal of a label in an operand position. *)
val ex_5_09 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
