(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Properties over the reference solutions of section 3.3. The unit
   spot checks in [test_sec_3_3.ml] cover the statements' own runs;
   these generalize them over arbitrary inputs. *)

module M = Sicp_ch3.Sec_3_3.Mpairs
module Q = Sicp_ch3.Sec_3_3.Queue
module Circuit = Sicp_ch3.Sec_3_3.Circuit
module Constraints = Sicp_ch3.Sec_3_3.Constraints
module Sec_3_12 = Sicp_ch3_solutions.Sec_3_12
module Sec_3_18 = Sicp_ch3_solutions.Sec_3_18
module Sec_3_19 = Sicp_ch3_solutions.Sec_3_19
module Sec_3_26 = Sicp_ch3_solutions.Sec_3_26
module Sec_3_27 = Sicp_ch3_solutions.Sec_3_27
module Sec_3_30 = Sicp_ch3_solutions.Sec_3_30
module Sec_3_37 = Sicp_ch3_solutions.Sec_3_37
open QCheck2

let to_int_list o =
  let rec go acc = function
    | M.Nil -> List.rev acc
    | M.Pair p ->
      (match p.car with
       | M.Int n -> go (n :: acc) p.cdr
       | _ -> assert false)
    | _ -> assert false
  in
  go [] o
;;

let append_is_pure_and_append_bang_splices =
  Test.make
    ~name:"append concatenates without disturbing x, append! splices onto it"
    ~count:200
    (Gen.pair
       (Gen.list_size (Gen.int_range 1 20) (Gen.int_range 0 100))
       (Gen.list_size (Gen.int_range 0 20) (Gen.int_range 0 100)))
    (fun (xs, ys) ->
       let x = M.from_objs (List.map M.mint xs) in
       let y = M.from_objs (List.map M.mint ys) in
       let z = M.append x y in
       let x_untouched = to_int_list x = xs in
       let z_is_concat = to_int_list z = xs @ ys in
       let x2 = M.from_objs (List.map M.mint xs) in
       let y2 = M.from_objs (List.map M.mint ys) in
       let w = Sec_3_12.append_bang x2 y2 in
       let w_is_concat = to_int_list w = xs @ ys in
       let x2_now_shows_concat = to_int_list x2 = xs @ ys in
       x_untouched && z_is_concat && w_is_concat && x2_now_shows_concat)
;;

let cycle_detectors_agree_with_construction =
  Test.make
    ~name:"both cycle detectors agree with how the structure was built"
    ~count:200
    (Gen.pair (Gen.int_range 1 20) Gen.bool)
    (fun (n, close_ring) ->
       let syms = List.init n (fun i -> Printf.sprintf "s%d" i) in
       let x = M.from_symbols syms in
       if close_ring then M.set_cdr (M.last_pair x) x;
       Bool.equal (Sec_3_18.contains_cycle x) close_ring
       && Bool.equal (Sec_3_19.contains_cycle_constant x) close_ring)
;;

let queue_matches_a_fifo_reference =
  Test.make
    ~name:"the pair queue matches a FIFO reference over random operations"
    ~count:200
    (Gen.list_size
       (Gen.int_range 0 60)
       (Gen.oneof
          [ Gen.map (fun n -> `Insert n) (Gen.int_range 0 1000); Gen.return `Delete ]))
    (fun ops ->
       let q = Q.make_queue () in
       let model = Stdlib.Queue.create () in
       List.iter
         (function
           | `Insert n ->
             ignore (Q.insert_queue q (M.mint n));
             Stdlib.Queue.push n model
           | `Delete ->
             if not (Q.empty_queue q)
             then (
               ignore (Q.delete_queue q);
               ignore (Stdlib.Queue.pop model)))
         ops;
       let items =
         List.map
           (function
             | M.Int n -> n
             | _ -> assert false)
           (Q.items q)
       in
       items = List.of_seq (Stdlib.Queue.to_seq model))
;;

let binary_tree_table_matches_an_assoc_model =
  Test.make
    ~name:"the binary-tree table agrees with a last-write-wins assoc model"
    ~count:200
    (Gen.list_size
       (Gen.int_range 1 40)
       (Gen.pair (Gen.int_range 0 20) (Gen.int_range 0 1000)))
    (fun kvs ->
       let t = Sec_3_26.make_table ~key_compare:Sec_3_26.key_compare in
       let model = Hashtbl.create 16 in
       List.iter
         (fun (k, v) ->
            Sec_3_26.insert (M.mint k) (M.mint v) t;
            Hashtbl.replace model k v)
         kvs;
       Hashtbl.fold
         (fun k v ok -> ok && Sec_3_26.lookup (M.mint k) t = Some (M.mint v))
         model
         true)
;;

let ripple_carry_adder_matches_integer_addition =
  Test.make
    ~name:"the ripple-carry adder matches integer addition, wrapped to width"
    ~count:200
    (Gen.triple (Gen.int_range 1 8) (Gen.int_range 0 255) (Gen.int_range 0 255))
    (fun (width, raw_a, raw_b) ->
       let mask = (1 lsl width) - 1 in
       let a = raw_a land mask in
       let b = raw_b land mask in
       let sim =
         Circuit.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:5 ()
       in
       let sum, carry = Sec_3_30.add sim a b width in
       let expect_sum = (a + b) land mask in
       let expect_carry = if a + b > mask then 1 else 0 in
       sum = expect_sum && carry = expect_carry)
;;

let memo_fib_matches_plain_fib =
  Test.make
    ~name:"memoized fib agrees with the plain recursive fib"
    ~count:30
    (Gen.int_range 0 22)
    (fun n ->
       let value, _hits, _computes = Sec_3_27.memo_fib_steps n in
       value = Sec_3_27.fib n)
;;

let celsius_fahrenheit_roundtrips =
  Test.make
    ~name:"the constraint network converts celsius to fahrenheit and back exactly"
    ~count:100
    (Gen.int_range (-20) 60)
    (fun steps_of_five ->
       let c_value = steps_of_five * 5 in
       let c = Constraints.make_connector () in
       let f = Sec_3_37.celsius_fahrenheit_converter c in
       c.set_value c_value Constraints.User;
       let f_value = f.get_value () in
       let forward_ok = f_value = (9 * c_value / 5) + 32 in
       c.forget_value Constraints.User;
       f.set_value f_value Constraints.User;
       let backward_ok = c.get_value () = c_value in
       forward_ok && backward_ok)
;;

let () =
  QCheck_base_runner.run_tests_main
    [ append_is_pure_and_append_bang_splices
    ; cycle_detectors_agree_with_construction
    ; queue_matches_a_fifo_reference
    ; binary_tree_table_matches_an_assoc_model
    ; ripple_carry_adder_matches_integer_addition
    ; memo_fib_matches_plain_fib
    ; celsius_fahrenheit_roundtrips
    ]
;;
