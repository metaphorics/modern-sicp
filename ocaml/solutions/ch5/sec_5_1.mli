(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.1 *)

(** Exercise 5.1 and this edition's 5.1a: the iterative factorial
    machine, designed as a typed controller and pinned by running it,
    then wrapped in the driver loop of 5.1.1's Actions over repeated
    inputs. *)

(** The controller of Exercise 5.1: three registers ([n], [product],
    [counter]), one test, and the two data-path buttons of the
    iteration step. The drawings the statement asks for are in
    solutions/ch5/ex_5_01.md. *)
val factorial_iterative_controller
  : Sicp_ch5.Sec_5_1.value Sicp_ch5.Sec_5_1.instruction list

(** [ex_5_01 ()] runs the designed machine on 0, 1, 5, and 10. *)
val ex_5_01 : unit -> (string list, Sicp_ch5.Sec_5_1.error) result

(** [ex_5_01a ()] is the transcript of two consecutive runs closed by
    the sentinel [Str "end"], then a run whose input dries up. Each run
    contributes its printed lines and a final stop report. *)
val ex_5_01a : unit -> (string list, Sicp_ch5.Sec_5_1.error) result
