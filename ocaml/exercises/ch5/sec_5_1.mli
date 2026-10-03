(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** The pending exercise's public contract. *)

(** Exercise 5.1: the designed iterative factorial machine. [ex_5_01 ()] runs the designed machine on 0, 1, 5, and
      10. *)
val ex_5_01 : unit -> (string list, Sicp_ch5.Sec_5_1.error) result

(** Exercise 5.1a (this edition's addition): the driver loop over
      repeated inputs. [ex_5_01a ()] is the transcript of two consecutive runs and
      a run whose input dries up. *)
val ex_5_01a : unit -> (string list, Sicp_ch5.Sec_5_1.error) result
