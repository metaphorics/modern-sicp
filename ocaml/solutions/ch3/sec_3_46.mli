(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.46 *)

(** Exercise 3.46: the text's [test-and-set!] written as an ordinary
    procedure, without atomicity. The exercise asks for a timing
    diagram in which two processes acquire the mutex at the same time;
    the edition replays the dangerous interleaving step by step and
    stresses both mutex constructions. *)

val naive_test_and_set : bool ref -> bool

(** [constructed_schedule_fails ()] replays the interleaving of the
    diagram: one process reads the free cell, a second completes a
    whole test-and-set, the first resumes and sets the cell too, and
    both report acquisition. *)
val constructed_schedule_fails : unit -> bool

(** [stress_double_acquires trials] runs [trials] acquisitions of the
    naive cell mutex from two domains and answers how many trials both
    domains believe they hold the mutex. The count is a measurement,
    not an asserted outcome. *)
val stress_double_acquires : int -> int

(** [host_mutex_double_acquires trials] runs the same acquisition
    protocol with the standard library's [Mutex], which is atomic; it
    always answers 0. *)
val host_mutex_double_acquires : int -> int

(** [ex_3_46 ()] is the constructed-schedule verdict and the measured
    double-acquire count of one stress batch. *)
val ex_3_46 : unit -> bool * int
