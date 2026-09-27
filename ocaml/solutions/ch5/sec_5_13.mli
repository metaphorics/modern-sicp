(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.13: the register set derived from the controller text. *)

(** [derive_registers program] names every register the parsed
    controller reads or writes, in first-use order, duplicates
    dropped. *)
val derive_registers : Sicp_ch5.Sec_5_2.program -> string list

(** [make_controller_machine ~operations controller] assembles the
    controller into a machine whose registers are the scan of the
    controller, no list supplied. *)
val make_controller_machine
  :  operations:(string * Sicp_ch5.Sec_5_2.op) list
  -> controller:string
  -> (Sicp_ch5.Sec_5_2.machine, Sicp_ch5.Sec_5_2.error) result

(** [ex_5_13 ()] assembles and runs the GCD and Fibonacci machines
    with no register list at all. *)
val ex_5_13 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
