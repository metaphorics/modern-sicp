(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.49 *)

(** Exercise 3.49: number-the-resources deadlock avoidance needs the
    full set of resources a process will touch known in advance. Here
    it is not: account [a] routes overdrafts to account [b] and [b]
    routes them to [a], so each process must hold its own account
    before it can even read which account it will need next. Two
    processes that each open their own account first, and only then
    reach for the other's, can deadlock no matter how the accounts are
    numbered, because neither could have ordered its acquisitions
    without already knowing the other's target. *)

let routing_deadlock_reachable () =
  let lock_a = Mutex.create () in
  let lock_b = Mutex.create () in
  let a_holds_own = Atomic.make false in
  let b_holds_own = Atomic.make false in
  let process_a_holds_target = Atomic.make true in
  let process_b_holds_target = Atomic.make true in
  let spin_until flag =
    let rec go budget =
      if Atomic.get flag || budget = 0
      then ()
      else (
        Domain.cpu_relax ();
        go (budget - 1))
    in
    go 5_000_000
  in
  let process_a () =
    Mutex.lock lock_a;
    Atomic.set a_holds_own true;
    (* the routing record inside [a] names [b] as the next account,
       but that is only knowable once [a]'s own lock is held *)
    spin_until b_holds_own;
    Atomic.set process_a_holds_target (Mutex.try_lock lock_b)
  in
  let process_b () =
    Mutex.lock lock_b;
    Atomic.set b_holds_own true;
    spin_until a_holds_own;
    Atomic.set process_b_holds_target (Mutex.try_lock lock_a)
  in
  let da = Domain.spawn process_a in
  let db = Domain.spawn process_b in
  Domain.join da;
  Domain.join db;
  let both_held_first = Atomic.get a_holds_own && Atomic.get b_holds_own in
  let both_failed_second =
    (not (Atomic.get process_a_holds_target)) && not (Atomic.get process_b_holds_target)
  in
  (* Neither process ever let go of its own lock -- that is the
     deadlock -- so the two mutexes stay locked here; a fresh call
     builds fresh locks. *)
  both_held_first, both_failed_second, both_held_first && both_failed_second
;;

let ex_3_49 () =
  let held, refused, deadlock = routing_deadlock_reachable () in
  held && refused && deadlock
;;
