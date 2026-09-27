(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.46 *)

(** Exercise 3.46: [test-and-set!] written as an ordinary, non-atomic
    procedure. The timing diagram: one process tests the cell and
    finds it free; before it sets the cell, a second process runs a
    whole test-and-set of its own, also finds the cell free, and sets
    it; the first process then finishes setting the cell too. Both
    believe they hold the mutex. *)

let naive_test_and_set cell =
  if !cell
  then true
  else (
    cell := true;
    false)
;;

(* The dangerous schedule, replayed step by step instead of hoped for
   from a race: A reads the free cell; B completes a full
   test-and-set and wins; A resumes and sets the cell too. *)
let constructed_schedule_fails () =
  let cell = ref false in
  let a_saw_free = not !cell in
  let b_acquired = not (naive_test_and_set cell) in
  let a_acquired =
    if a_saw_free
    then (
      cell := true;
      true)
    else false
  in
  a_acquired && b_acquired
;;

(* Two domains repeatedly try to hold the mutex given by [acquire] and
   [release] around a tiny critical section, and this counts the
   rounds where more than one domain believed it was inside at once.
   The count is a measurement of one stress batch, never an asserted
   outcome. *)
let stress ~acquire ~release trials =
  let inside = Atomic.make 0 in
  let violations = Atomic.make 0 in
  let worker () =
    for _ = 1 to trials do
      if acquire ()
      then (
        let now = Atomic.fetch_and_add inside 1 + 1 in
        if now > 1 then Atomic.incr violations;
        Domain.cpu_relax ();
        Domain.cpu_relax ();
        Atomic.decr inside;
        release ())
    done
  in
  let d = Domain.spawn worker in
  worker ();
  Domain.join d;
  Atomic.get violations
;;

let stress_double_acquires trials =
  let cell = ref false in
  let acquire () =
    let rec loop budget =
      if budget = 0
      then false
      else if naive_test_and_set cell
      then loop (budget - 1)
      else true
    in
    loop 200_000
  in
  let release () = cell := false in
  stress ~acquire ~release trials
;;

let host_mutex_double_acquires trials =
  let mutex = Mutex.create () in
  let acquire () =
    Mutex.lock mutex;
    true
  in
  let release () = Mutex.unlock mutex in
  stress ~acquire ~release trials
;;

let ex_3_46 () = constructed_schedule_fails (), stress_double_acquires 2000
