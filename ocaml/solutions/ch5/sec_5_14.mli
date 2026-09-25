(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.2 *)

(** Exercise 5.14: the monitored stack measures the factorial machine. *)

(** [measure n] runs the measured Figure 5.11 machine on [n] and
    reports the statistics line it printed. *)
val measure : int -> (string, Sicp_ch5.Sec_5_2.error) result

(** The Figure 5.11 controller augmented with the stack-clearing and
    statistics instructions. *)
val factorial_measured_controller : string

(** [ex_5_14 ()] measures n = 1..7 and states the formulas: pushes and
    maximum depth are both 2n - 2 for n > 1. *)
val ex_5_14 : unit -> (string list, Sicp_ch5.Sec_5_2.error) result
