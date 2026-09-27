(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.47 *)

(** Exercise 3.47: a semaphore of size [n] generalizes a mutex: up to
    [n] processes hold it at once. Part (a) builds one from mutexes,
    part (b) from atomic test-and-set operations. This edition adds
    exercise 3.47a in the same module: a bounded semaphore that blocks
    on [Mutex] and [Condition] instead of spinning. *)

(** The semaphore surface: acquire blocks while no permit is free;
    release returns one. *)
type semaphore =
  { acquire : unit -> unit
  ; release : unit -> unit
  }

(** Part (a): the permits are counted under a mutex; an acquirer that
    finds none free releases the mutex and retries. *)
val make_semaphore_via_mutexes : int -> semaphore

(** Part (b): the same count is guarded by the section's atomic
    test-and-set cell instead of the host mutex. *)
val make_semaphore_via_test_and_set : int -> semaphore

(** [max_concurrent_entries semaphore ~workers ~rounds] sends
    [workers] domains through [rounds] acquire/enter/release rounds
    each and answers the largest number of domains observed inside at
    once. With a correct size-[n] semaphore the answer never exceeds
    [n]. *)
val max_concurrent_entries : semaphore -> workers:int -> rounds:int -> int

(** [ex_3_47 ()] is the observed maximum for the mutex construction
    and for the test-and-set construction, each a size-2 semaphore
    driven by four workers. *)
val ex_3_47 : unit -> int * int

(** The bounded semaphore of exercise 3.47a, this edition's addition:
    acquire and release block on [Mutex] and [Condition], so a waiting
    acquirer burns no cycles. *)
type bounded

(** [make_bounded n] is a semaphore of size [n]; [n] must be positive. *)
val make_bounded : int -> bounded

val acquire_bounded : bounded -> unit
val release_bounded : bounded -> unit

(** [try_acquire_bounded s] takes a permit if one is free and answers
    whether it did; it never blocks. *)
val try_acquire_bounded : bounded -> bool

(** [ex_3_47a ()] is the try-acquire verdict (two permits for a
    size-2 semaphore, the third attempt refused), whether an acquirer
    blocked on the empty semaphore completed after a release, and the
    largest number of domains observed inside at once under a
    concurrent workload. *)
val ex_3_47a : unit -> bool * bool * int
