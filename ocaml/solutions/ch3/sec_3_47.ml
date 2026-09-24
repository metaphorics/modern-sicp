(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 3.4 exercise 3.47 *)

(** Exercise 3.47: a semaphore of size [n] lets up to [n] processes
    hold it at once. Part (a) counts permits under a host mutex, so an
    acquirer that finds none free releases the mutex and retries.
    Part (b) guards the same count with the section's own atomic
    test-and-set spinlock instead. *)

type semaphore =
  { acquire : unit -> unit
  ; release : unit -> unit
  }

let make_semaphore_via_mutexes n =
  let guard = Mutex.create () in
  let permits = ref n in
  let rec acquire () =
    let took =
      Mutex.protect guard (fun () ->
        if !permits > 0
        then (
          permits := !permits - 1;
          true)
        else false)
    in
    if not took
    then (
      Domain.cpu_relax ();
      acquire ())
  in
  let release () = Mutex.protect guard (fun () -> incr permits) in
  { acquire; release }
;;

let make_semaphore_via_test_and_set n =
  let guard = Atomic.make false in
  let take_guard () =
    while Atomic.compare_and_set guard false true |> not do
      Domain.cpu_relax ()
    done
  in
  let permits = ref n in
  let rec acquire () =
    take_guard ();
    let took =
      if !permits > 0
      then (
        permits := !permits - 1;
        true)
      else false
    in
    Atomic.set guard false;
    if not took
    then (
      Domain.cpu_relax ();
      acquire ())
  in
  let release () =
    take_guard ();
    incr permits;
    Atomic.set guard false
  in
  { acquire; release }
;;

let max_concurrent_entries semaphore ~workers ~rounds =
  let inside = Atomic.make 0 in
  let max_seen = Atomic.make 0 in
  let rec bump_max current =
    let seen = Atomic.get max_seen in
    if current > seen && not (Atomic.compare_and_set max_seen seen current)
    then bump_max current
  in
  let worker () =
    for _ = 1 to rounds do
      semaphore.acquire ();
      let now = Atomic.fetch_and_add inside 1 + 1 in
      bump_max now;
      Domain.cpu_relax ();
      Domain.cpu_relax ();
      ignore (Atomic.fetch_and_add inside (-1));
      semaphore.release ()
    done
  in
  let domains = List.init (workers - 1) (fun _ -> Domain.spawn worker) in
  worker ();
  List.iter Domain.join domains;
  Atomic.get max_seen
;;

let ex_3_47 () =
  let via_mutexes =
    max_concurrent_entries (make_semaphore_via_mutexes 2) ~workers:4 ~rounds:50
  in
  let via_tas =
    max_concurrent_entries (make_semaphore_via_test_and_set 2) ~workers:4 ~rounds:50
  in
  via_mutexes, via_tas
;;

(** Exercise 3.47a, this edition's addition: a bounded semaphore that
    blocks on [Mutex] and [Condition] instead of spinning, so a
    waiting acquirer burns no cycles. *)
type bounded =
  { mutex : Mutex.t
  ; condition : Condition.t
  ; mutable available : int
  }

let make_bounded n =
  if n <= 0 then invalid_arg "make_bounded: capacity must be positive";
  { mutex = Mutex.create (); condition = Condition.create (); available = n }
;;

let acquire_bounded s =
  Mutex.lock s.mutex;
  while s.available = 0 do
    Condition.wait s.condition s.mutex
  done;
  s.available <- s.available - 1;
  Mutex.unlock s.mutex
;;

let release_bounded s =
  Mutex.lock s.mutex;
  s.available <- s.available + 1;
  Condition.signal s.condition;
  Mutex.unlock s.mutex
;;

let try_acquire_bounded s =
  Mutex.protect s.mutex (fun () ->
    if s.available > 0
    then (
      s.available <- s.available - 1;
      true)
    else false)
;;

let ex_3_47a () =
  let s = make_bounded 2 in
  let first = try_acquire_bounded s in
  let second = try_acquire_bounded s in
  let third_refused = not (try_acquire_bounded s) in
  release_bounded s;
  release_bounded s;
  (* A drained size-1 semaphore: the acquirer below can only get in
     once the release above hands the permit back. *)
  let blocked_sem = make_bounded 1 in
  acquire_bounded blocked_sem;
  let completed = Atomic.make false in
  let waiter =
    Domain.spawn (fun () ->
      acquire_bounded blocked_sem;
      Atomic.set completed true)
  in
  let rec spin budget =
    if Atomic.get completed || budget = 0
    then ()
    else (
      Domain.cpu_relax ();
      spin (budget - 1))
  in
  spin 5_000_000;
  let blocked_before_release = not (Atomic.get completed) in
  release_bounded blocked_sem;
  Domain.join waiter;
  let completed_after_release = Atomic.get completed in
  let semaphore =
    { acquire = (fun () -> acquire_bounded s); release = (fun () -> release_bounded s) }
  in
  let max_entries = max_concurrent_entries semaphore ~workers:4 ~rounds:50 in
  ( first && second && third_refused
  , blocked_before_release && completed_after_release
  , max_entries )
;;
