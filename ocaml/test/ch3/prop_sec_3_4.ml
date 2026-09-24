(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Properties over the reference solutions of section 3.4. The unit
   spot checks in [test_sec_3_4.ml] pin the deterministic values;
   these assert the section's invariants over many runs and random
   inputs, because OCaml's scheduler makes particular interleavings
   too rare to schedule directly. *)

module Sec_3_4 = Sicp_ch3.Sec_3_4
module Sec_3_41 = Sicp_ch3_solutions.Sec_3_41
module Sec_3_44 = Sicp_ch3_solutions.Sec_3_44
module Sec_3_46 = Sicp_ch3_solutions.Sec_3_46
module Sec_3_47 = Sicp_ch3_solutions.Sec_3_47
module Sec_3_48 = Sicp_ch3_solutions.Sec_3_48
module Account = Sicp_ch3.Sec_3_4.Account
open QCheck2

let serialized_race_stays_in_the_two_outcome_set =
  Test.make
    ~name:"the serialized increment/square race lands in {101, 121} every run"
    ~count:200
    Gen.unit
    (fun () ->
       List.mem (Sec_3_4.X_race.run_serialized ()) Sec_3_4.X_race.serialized_values)
;;

let unserialized_race_stays_in_the_five_outcome_set =
  Test.make
    ~name:"the unserialized increment/square race lands in the five-outcome set"
    ~count:200
    Gen.unit
    (fun () ->
       List.mem (Sec_3_4.X_race.run_unserialized ()) Sec_3_4.X_race.possible_values)
;;

let serialized_deposits_conserve_every_unit =
  Test.make
    ~name:"concurrent serialized deposits add exactly the amount deposited"
    ~count:100
    (Gen.pair (Gen.int_range 0 500) (Gen.int_range 0 200))
    (fun (initial, count) ->
       let account = Account.make_account initial in
       Account.concurrent_deposits account count = initial + count)
;;

let serialized_reads_never_leave_the_written_range =
  Test.make
    ~name:"every balance read during concurrent deposits was written by one"
    ~count:20
    (Gen.int_range 50 400)
    (fun rounds ->
       let reads = Sec_3_41.collect_reads rounds in
       List.length reads > 0 && Sec_3_41.anomalous_reads rounds reads = 0)
;;

let concurrent_transfers_conserve_the_total =
  Test.make
    ~name:"concurrent transfers move money without losing any"
    ~count:100
    (Gen.int_range 1 100)
    (fun per_transfer ->
       Sec_3_44.total_preserved_under_concurrent_transfers 50 per_transfer)
;;

let host_mutex_never_double_acquires =
  Test.make
    ~name:"the host mutex never seats two domains at once"
    ~count:20
    (Gen.int_range 100 2000)
    (fun trials -> Sec_3_46.host_mutex_double_acquires trials = 0)
;;

let bounded_semaphore_capacity_is_a_hard_bound =
  Test.make
    ~name:"no more than n domains ever sit inside a size-n semaphore"
    ~count:40
    (Gen.int_range 1 4)
    (fun size ->
       let semaphore = Sec_3_47.make_bounded size in
       let wrapped =
         { Sec_3_47.acquire = (fun () -> Sec_3_47.acquire_bounded semaphore)
         ; release = (fun () -> Sec_3_47.release_bounded semaphore)
         }
       in
       Sec_3_47.max_concurrent_entries wrapped ~workers:4 ~rounds:30 <= size)
;;

let ordered_exchanges_preserve_the_total_balance =
  Test.make
    ~name:"ordered two-account exchanges preserve the total balance"
    ~count:200
    (Gen.pair (Gen.int_range 0 1000) (Gen.int_range 0 1000))
    (fun (b1, b2) ->
       let a1 = Sec_3_48.make_numbered_account 1 b1 in
       let a2 = Sec_3_48.make_numbered_account 2 b2 in
       for _ = 1 to 25 do
         Sec_3_48.ordered_serialized_exchange a1 a2
       done;
       Sec_3_48.balance_of a1 + Sec_3_48.balance_of a2 = b1 + b2)
;;

let () =
  QCheck_base_runner.run_tests_main
    [ serialized_race_stays_in_the_two_outcome_set
    ; unserialized_race_stays_in_the_five_outcome_set
    ; serialized_deposits_conserve_every_unit
    ; serialized_reads_never_leave_the_written_range
    ; concurrent_transfers_conserve_the_total
    ; host_mutex_never_double_acquires
    ; bounded_semaphore_capacity_is_a_hard_bound
    ; ordered_exchanges_preserve_the_total_balance
    ]
;;
