(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.23: derived expressions -- [cond] and [let] enter the
    evaluator through transformer machine operations. *)

(** [controller] is the exercise's evaluator: the base controller with
    [cond?] and [let?] tests in the dispatch and the [ev-cond]/[ev-let]
    transformer entries appended. *)
val controller : string

(** The exercise's operations: the two syntax tests and the two
    transformers [cond->if] and [let->combination]. *)
val operations : (string * Sicp_ch5.Sec_5_4.op) list

(** [run source] evaluates the object program [source] and answers the
    driver's transcript; the queue-dry end of the driver loop is the
    normal end. *)
val run : string -> (string list, Sicp_ch5.Sec_5_4.error) result

(** [ex_5_23 ()] runs [cond] and [let] sessions through the extended
    evaluator: a three-clause classify with an [else], a bodyless
    clause, and a [let] whose body is a lambda application. *)
val ex_5_23 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
