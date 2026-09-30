(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.13: the register set derived from the controller, not
    supplied by the caller. *)

(** [derive_registers controller] names every register [controller]
    reads or writes, in first-use order, duplicates dropped. *)
val derive_registers
  :  Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
  -> string list

(** [make_controller_machine ~operations ~controller] assembles
    [controller] into a machine whose registers are
    [derive_registers controller]. *)
val make_controller_machine
  :  operations:(string * Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.op) list
  -> controller:Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list
  -> (Sicp_ch5.Sec_5_2.machine, Sicp_ch5.Sec_5_2.error) result

(** [ex_5_13 ()] assembles and runs the GCD and Fibonacci machines
    with no register list at all. *)
val ex_5_13 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
