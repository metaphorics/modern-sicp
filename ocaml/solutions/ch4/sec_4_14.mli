(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.14 *)

(** Exercise 4.14: a host higher-order function installed as a
    primitive.

    Louis installs the host's own [List.map] as the guest's [List.map].
    A primitive runs host code, and host code can call only host
    functions: a guest closure is a body and an environment that only
    the evaluator can run, and Louis's primitive is never handed the
    evaluator.  So his map works on a primitive such as
    [string_of_int] and fails on any guest function.  Eva's map is a
    guest program, so the evaluator runs every call.  The prelude's
    [List.map] works because it is written to receive the evaluator's
    own [apply].  Exercise 4.14a repeats the diagnosis with [List.sort],
    whose comparator is always a guest function. *)

(** [louis_map] is the host [List.map] as a guest primitive. *)
val louis_map : Sicp_common.Value.t

(** [louis_sort] is the host [List.sort] as a guest primitive. *)
val louis_sort : Sicp_common.Value.t

(** [eval] is the standard evaluator run in a global environment whose
    [List.map] is [louis_map]. *)
val eval : Sicp_ch4.Sec_4_1.eval_t

(** [ex_4_14 ()] maps [string_of_int] and then a guest squaring
    function with Louis's primitive, maps the guest function with Eva's
    guest [map], and maps it with the prelude's [List.map]. *)
val ex_4_14 : unit -> string list

(** [ex_4_14a ()] sorts a list with a guest comparator through Louis's
    [List.sort], through Eva's guest insertion sort, and through the
    prelude's [List.sort]. *)
val ex_4_14a : unit -> string list
