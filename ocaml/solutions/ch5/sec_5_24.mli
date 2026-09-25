(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.4 *)

(** Exercise 5.24: [cond] as a basic controller form -- not reduced to
    [if]; and Exercise 5.24a, added by this edition: [and] and [or] as
    basic controller forms in the same style. *)

(** [controller] is the exercise's evaluator: the base controller with
    [cond?], [and?], and [or?] tests in the dispatch and the
    [ev-cond]/[ev-and]/[ev-or] basic-form loops appended. *)
val controller : string

(** [clause_operations] are the syntax operations the basic forms
    need: the form tests, the cond clause selectors and predicates, and
    the and/or operand selectors. *)
val clause_operations : (string * Sicp_ch5.Sec_5_4.op) list

(** [run source] evaluates the object program [source] and answers the
    driver's transcript; the queue-dry end of the driver loop is the
    normal end. *)
val run : string -> (string list, Sicp_ch5.Sec_5_4.error) result

(** [ex_5_24 ()] runs the cond sessions: a three-clause classify with
    an [else], a bodyless clause whose value is its test, and a cond
    with no true clause. *)
val ex_5_24 : unit -> (string list, Sicp_ch5.Sec_5_4.error) result

(** [ex_5_24a ()] runs the and/or sessions: values of the last and
    first-true operand, the empty forms, short-circuit both ways --
    a false cut before an unbound variable, a true cut before a false
    -- and nesting. *)
val ex_5_24a : unit -> (string list, Sicp_ch5.Sec_5_4.error) result
