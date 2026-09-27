(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the reference solutions' public contracts for
   section 3.4. Deterministic values the exercises compute are pinned
   exactly; every interleaving-dependent quantity is asserted as an
   invariant (membership in the enumerated outcome set, conservation,
   a capacity bound), never as one specific schedule's outcome. *)

module Sec_3_38 = Sicp_ch3_solutions.Sec_3_38
module Sec_3_39 = Sicp_ch3_solutions.Sec_3_39
module Sec_3_40 = Sicp_ch3_solutions.Sec_3_40
module Sec_3_41 = Sicp_ch3_solutions.Sec_3_41
module Sec_3_42 = Sicp_ch3_solutions.Sec_3_42
module Sec_3_43 = Sicp_ch3_solutions.Sec_3_43
module Sec_3_44 = Sicp_ch3_solutions.Sec_3_44
module Sec_3_45 = Sicp_ch3_solutions.Sec_3_45
module Sec_3_46 = Sicp_ch3_solutions.Sec_3_46
module Sec_3_47 = Sicp_ch3_solutions.Sec_3_47
module Sec_3_48 = Sicp_ch3_solutions.Sec_3_48
module Sec_3_49 = Sicp_ch3_solutions.Sec_3_49
module Account = Sicp_ch3.Sec_3_4.Account

let check_int_list name expected got = Alcotest.(check (list int)) name expected got
let is_subset smaller larger = List.for_all (fun x -> List.mem x larger) smaller

let ex_3_38_sequential_orders () =
  let sequential, interleaved, sampled = Sec_3_38.ex_3_38 () in
  check_int_list
    "the six sequential orders leave four balances"
    [ 35; 40; 45; 50 ]
    sequential;
  check_int_list
    "every interleaving of reads and writes, enumerated"
    [ 30; 35; 40; 45; 50; 55; 60; 80; 90; 110 ]
    interleaved;
  Alcotest.(check bool)
    "the sampled live runs landed inside the enumerated set"
    true
    (is_subset sampled interleaved)
;;

let ex_3_39_serialized_possibilities () =
  check_int_list
    "all five outcomes before serialization"
    [ 101; 121; 110; 11; 100 ]
    Sec_3_39.possibilities_before;
  check_int_list
    "the serializer leaves the write-outside race alive"
    [ 11; 100; 101; 121 ]
    (Sec_3_39.ex_3_39 ());
  Alcotest.(check bool)
    "live runs stay inside the computed set"
    true
    (is_subset (Sec_3_39.sample_runs 100) (Sec_3_39.ex_3_39 ()))
;;

let ex_3_40_powers_of_ten () =
  let unserialized, serialized = Sec_3_40.ex_3_40 () in
  check_int_list
    "unsynchronized squaring and cubing leave five powers"
    [ 100; 1000; 10000; 100000; 1000000 ]
    unserialized;
  check_int_list "serialized, both orders agree on x^6" [ 1000000 ] serialized;
  Alcotest.(check bool)
    "live unsynchronized runs stay inside the enumeration"
    true
    (is_subset (Sec_3_40.sample_unserialized_runs 100) unserialized)
;;

let ex_3_41_reads_never_lie () =
  let rounds = 500 in
  let reads = Sec_3_41.collect_reads rounds in
  Alcotest.(check bool) "the reader saw the account change" true (List.length reads > 0);
  Alcotest.(check int)
    "every reading is a balance some deposit wrote"
    0
    (Sec_3_41.anomalous_reads rounds reads)
;;

let ex_3_42_hoisting_is_safe () =
  let per_message, hoisted = Sec_3_42.ex_3_42 () in
  Alcotest.(check int) "per-message serialization conserves every deposit" 400 per_message;
  Alcotest.(check int) "hoisted serialization conserves every deposit too" 400 hoisted
;;

let ex_3_43_multiset_and_sum () =
  let serialized_ok, sum_ok, violations = Sec_3_43.ex_3_43 () in
  Alcotest.(check bool)
    "serialized exchanges keep the original three balances"
    true
    serialized_ok;
  Alcotest.(check bool) "even interleaved exchanges conserve the sum" true sum_ok;
  ignore violations
;;

(* an observed count, never an asserted outcome *)

let ex_3_44_transfer_conserves () =
  let ok, total = Sec_3_44.ex_3_44 () in
  Alcotest.(check bool) "concurrent transfers conserve the total" true ok;
  Alcotest.(check int) "three thousand dollars stay three thousand" 3000 total;
  let a = Account.make_account 100 in
  let b = Account.make_account 0 in
  Sec_3_44.transfer a b 30;
  let balance_of account =
    match account Account.Balance with
    | Account.New_balance n -> n
    | _ -> invalid_arg "balance_of: not a balance"
  in
  Alcotest.(check (pair int int))
    "one transfer moves exactly the amount"
    (70, 30)
    (balance_of a, balance_of b)
;;

let ex_3_45_louis_deadlocks () =
  let inner_impossible, stalled = Sec_3_45.ex_3_45 () in
  Alcotest.(check bool)
    "a held mutex cannot be acquired again by its holder"
    true
    inner_impossible;
  Alcotest.(check bool) "Louis's doubly serialized exchange cannot complete" true stalled
;;

let ex_3_46_naive_cell_races () =
  let schedule_fails, naive_count = Sec_3_46.ex_3_46 () in
  Alcotest.(check bool)
    "the replayed schedule seats both processes in the mutex"
    true
    schedule_fails;
  ignore naive_count (* a measurement of one stress batch, never an asserted outcome *);
  Alcotest.(check int)
    "the host mutex never double-seats anyone"
    0
    (Sec_3_46.host_mutex_double_acquires 5000)
;;

(* Drives [workers] domains at [semaphore] and answers whether an
   extra acquirer, spawned while the caller holds every permit, was
   kept out until a release. *)
let capacity_keeps_extras_out semaphore size =
  for _ = 1 to size do
    semaphore.Sec_3_47.acquire ()
  done;
  let got_in = Atomic.make false in
  let extra =
    Domain.spawn (fun () ->
      semaphore.Sec_3_47.acquire ();
      Atomic.set got_in true)
  in
  let rec spin budget =
    if Atomic.get got_in || budget = 0
    then ()
    else (
      Domain.cpu_relax ();
      spin (budget - 1))
  in
  spin 2_000_000;
  let held_out = not (Atomic.get got_in) in
  semaphore.Sec_3_47.release ();
  Domain.join extra;
  held_out && Atomic.get got_in
;;

let ex_3_47_semaphore_of_size_two () =
  let via_mutexes, via_tas = Sec_3_47.ex_3_47 () in
  Alcotest.(check bool)
    "the mutex-built semaphore never seated more than two"
    true
    (via_mutexes <= 2);
  Alcotest.(check bool)
    "the test-and-set semaphore never seated more than two"
    true
    (via_tas <= 2);
  Alcotest.(check bool)
    "with both permits out, a third acquirer waits for a release"
    true
    (capacity_keeps_extras_out (Sec_3_47.make_semaphore_via_mutexes 2) 2)
;;

let ex_3_47a_bounded_semaphore () =
  let try_ok, blocks_then_completes, max_entries = Sec_3_47.ex_3_47a () in
  Alcotest.(check bool) "try-acquire honors the permit count" true try_ok;
  Alcotest.(check bool)
    "a blocked acquirer completes when a permit returns"
    true
    blocks_then_completes;
  Alcotest.(check bool)
    "the bounded semaphore never seated more than two"
    true
    (max_entries <= 2);
  let s = Sec_3_47.make_bounded 2 in
  Alcotest.(check bool) "first permit" true (Sec_3_47.try_acquire_bounded s);
  Alcotest.(check bool) "second permit" true (Sec_3_47.try_acquire_bounded s);
  Alcotest.(check bool)
    "an empty semaphore refuses"
    false
    (Sec_3_47.try_acquire_bounded s);
  Alcotest.check_raises
    "a nonpositive capacity is rejected"
    (Invalid_argument "make_bounded: capacity must be positive")
    (fun () -> ignore (Sec_3_47.make_bounded 0))
;;

let ex_3_48_ordered_locks_complete () =
  let survived, balances = Sec_3_48.ex_3_48 () in
  Alcotest.(check bool) "reversed concurrent exchanges all complete" true survived;
  check_int_list "the balances stay the original three amounts" [ 10; 20; 30 ] balances;
  Alcotest.(check bool)
    "the reversed scenario survives many rounds"
    true
    (Sec_3_48.survives_reversed_concurrent_exchanges 300)
;;

let ex_3_49_routing_defeats_numbering () =
  Alcotest.(check bool)
    "each process holds its own account, and neither gets the other's"
    true
    (Sec_3_49.ex_3_49 ());
  let held_own, refused_other, deadlock = Sec_3_49.routing_deadlock_reachable () in
  Alcotest.(check bool) "both first locks taken" true held_own;
  Alcotest.(check bool) "both second locks refused" true refused_other;
  Alcotest.(check bool) "and that is a deadlock" true deadlock
;;

(* The run handle of [Parallel.parallel]: [halt] sets the flag the
   workers' [halted] probes read, so a worker still polling gives up. *)
let parallel_halt_reaches_a_polling_worker () =
  let module P = Sicp_ch3.Sec_3_4.Parallel in
  let started = Atomic.make false in
  let released = Atomic.make false in
  let gave_up, _, _ =
    P.parallel
      (fun handle ->
         Atomic.set started true;
         let rec spin budget =
           if Atomic.get released
           then handle.P.halted ()
           else if budget = 0
           then false
           else (
             Domain.cpu_relax ();
             spin (budget - 1))
         in
         spin 20_000_000)
      (fun handle ->
         while not (Atomic.get started) do
           Domain.cpu_relax ()
         done;
         handle.P.halt ();
         Atomic.set released true)
  in
  Alcotest.(check bool) "the halt flag reaches the worker still polling" true gave_up
;;

let () =
  Alcotest.run
    "sicp_ch3 solutions, section 3.4"
    [ ( "parallel halt handle"
      , [ Alcotest.test_case
            "halt reaches a worker still polling"
            `Quick
            parallel_halt_reaches_a_polling_worker
        ] )
    ; ( "3.38 interleaved balances"
      , [ Alcotest.test_case
            "sequential, interleaved, and live outcomes"
            `Quick
            ex_3_38_sequential_orders
        ] )
    ; ( "3.39 which outcomes survive serialization"
      , [ Alcotest.test_case
            "the computed set holds the live runs"
            `Quick
            ex_3_39_serialized_possibilities
        ] )
    ; ( "3.40 concurrent squaring and cubing"
      , [ Alcotest.test_case
            "enumerated values and the serialized answer"
            `Quick
            ex_3_40_powers_of_ten
        ] )
    ; ( "3.41 serialized balance reads"
      , [ Alcotest.test_case
            "a watcher never reads an unwritten balance"
            `Quick
            ex_3_41_reads_never_lie
        ] )
    ; ( "3.42 serialization hoisted out of dispatch"
      , [ Alcotest.test_case
            "both versions conserve the deposits"
            `Quick
            ex_3_42_hoisting_is_safe
        ] )
    ; ( "3.43 concurrent exchanges"
      , [ Alcotest.test_case "multiset and sum verdicts" `Quick ex_3_43_multiset_and_sum ]
      )
    ; ( "3.44 transfer needs no joint lock"
      , [ Alcotest.test_case
            "concurrent transfers conserve"
            `Quick
            ex_3_44_transfer_conserves
        ] )
    ; ( "3.45 double serialization deadlocks"
      , [ Alcotest.test_case
            "Louis's exchange cannot complete"
            `Quick
            ex_3_45_louis_deadlocks
        ] )
    ; ( "3.46 the test-and-set race window"
      , [ Alcotest.test_case
            "constructed schedule and host comparison"
            `Quick
            ex_3_46_naive_cell_races
        ] )
    ; ( "3.47 semaphores of size n"
      , [ Alcotest.test_case
            "capacity bound and blocking third acquirer"
            `Quick
            ex_3_47_semaphore_of_size_two
        ] )
    ; ( "3.47a bounded semaphore on Condition"
      , [ Alcotest.test_case
            "permits, blocking, and the capacity bound"
            `Quick
            ex_3_47a_bounded_semaphore
        ] )
    ; ( "3.48 deadlock avoidance by ordering"
      , [ Alcotest.test_case
            "ordered exchanges complete"
            `Quick
            ex_3_48_ordered_locks_complete
        ] )
    ; ( "3.49 when ordering cannot help"
      , [ Alcotest.test_case
            "the routing scenario deadlocks"
            `Quick
            ex_3_49_routing_defeats_numbering
        ] )
    ]
;;
