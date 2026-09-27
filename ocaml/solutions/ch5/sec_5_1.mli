(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.1 and this edition's 5.1a: the iterative factorial
    machine and the driver loop over repeated inputs. *)

(** [ex_5_01 ()] runs the designed machine on 0, 1, 5, and 10. *)
val ex_5_01 : unit -> (string list, Sicp_ch5.Sec_5_1.error) result

(** [ex_5_01a ()] is the transcript of two consecutive runs closed by
    [end], then a run whose input dries up. *)
val ex_5_01a : unit -> (string list, Sicp_ch5.Sec_5_1.error) result
