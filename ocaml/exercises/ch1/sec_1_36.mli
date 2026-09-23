(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program fixed-point in SICP section 1.3
   exercise 1.36 *)

(** Exercise 1.36: [fixed_point_traced] prints every guess it tries,
    and a comparison of solving [x^x = 1000] with and without average
    damping. The stubs raise [Sicp_common.Pending.Pending_solution]
    until they are solved. *)

(** [fixed_point_traced f first_guess] prints every guess [f] produces,
    one per line, and returns [(final_value, step_count)]. *)
val fixed_point_traced : (float -> float) -> float -> float * int

(** [x_to_the_x_eq_1000 damped] solves [x^x = 1000] as a fixed point of
    [x -> log 1000 / log x] from a guess of [2.]; with [damped = true]
    the transform is average-damped first. *)
val x_to_the_x_eq_1000 : bool -> float * int

(** [ex_1_36 ()] is [(x_to_the_x_eq_1000 false, x_to_the_x_eq_1000
    true)]. *)
val ex_1_36 : unit -> (float * int) * (float * int)
