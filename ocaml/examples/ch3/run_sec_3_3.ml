(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Sec_3_3 = Sicp_ch3.Sec_3_3
open Sec_3_3
module M = Mpairs
module Q = Queue
module T = Table
module C = Circuit
module K = Constraints

let expect_show o shown = Replay.expect (M.show o) shown

let () =
  (* 3.3.1: (set-car! x y) modifies the pair x is bound to; the pair
     (a b) is detached. *)
  let x = M.from_symbols [ "a"; "b"; "c"; "d" ] in
  let y = M.from_symbols [ "e"; "f" ] in
  M.set_car x y;
  expect_show x "((e f) b c d)";
  (* (cons y (cdr x)) builds a new pair; x's list is unchanged. *)
  let x = M.from_symbols [ "a"; "b"; "c"; "d" ] in
  let z = M.mcons y (M.cdr x) in
  expect_show x "(a b c d)";
  expect_show z "((e f) b c d)";
  (* (set-cdr! x y) replaces the cdr pointer. *)
  M.set_cdr x y;
  expect_show x "(a e f)";
  (* cons built from the two mutators. *)
  let p = M.cons_from_mutators (M.mint 1) (M.mint 2) in
  expect_show p "(1 . 2)";
  (* Sharing: z1 shares x between its car and cdr; z2 does not. *)
  let x = M.from_symbols [ "a"; "b" ] in
  let z1 =
    let shared = M.Pair (M.pair_of x) in
    { M.car = shared; M.cdr = shared }
  in
  let z2 = M.mcons (M.from_symbols [ "a"; "b" ]) (M.from_symbols [ "a"; "b" ]) in
  expect_show (M.Pair z1) "((a b) a b)";
  expect_show z2 "((a b) a b)";
  ignore (M.set_to_wow (M.Pair z1));
  ignore (M.set_to_wow z2);
  expect_show (M.Pair z1) "((wow b) wow b)";
  expect_show z2 "((wow b) a b)"
;;

let () =
  (* 3.3.2: the queue of @ref{Figure 3.18}: insert a, insert b, delete
     a, insert c, insert d, delete b. *)
  let q = Q.make_queue () in
  assert (Q.empty_queue q);
  ignore (Q.insert_queue q (M.msym "a"));
  ignore (Q.insert_queue q (M.msym "b"));
  expect_show (Q.front_queue q) "a";
  ignore (Q.delete_queue q);
  ignore (Q.insert_queue q (M.msym "c"));
  ignore (Q.insert_queue q (M.msym "d"));
  ignore (Q.delete_queue q);
  let items = List.map (fun o -> M.show o) (Q.items q) in
  Replay.expect (String.concat " " items) "c d"
;;

let () =
  (* 3.3.3: the one-dimensional table a: 1, b: 2, c: 3 and the
     two-dimensional table of @ref{Figure 3.23}. *)
  let t1 = T.make_table () in
  T.insert (M.msym "a") (M.mint 1) t1;
  T.insert (M.msym "b") (M.mint 2) t1;
  T.insert (M.msym "c") (M.mint 3) t1;
  let shown =
    match T.lookup (M.msym "b") t1 with
    | Some v -> M.show v
    | None -> "false"
  in
  Replay.expect shown "2";
  let t2 = T.make_table () in
  T.insert2 (M.msym "math") (M.msym "+") (M.mint 43) t2;
  T.insert2 (M.msym "math") (M.msym "-") (M.mint 45) t2;
  T.insert2 (M.msym "letters") (M.msym "a") (M.mint 97) t2;
  T.insert2 (M.msym "letters") (M.msym "b") (M.mint 98) t2;
  let plus =
    match T.lookup2 (M.msym "math") (M.msym "+") t2 with
    | Some v -> M.show v
    | None -> "false"
  in
  let b =
    match T.lookup2 (M.msym "letters") (M.msym "b") t2 with
    | Some v -> M.show v
    | None -> "false"
  in
  Replay.expect plus "43";
  Replay.expect b "98"
;;

let () =
  (* 3.3.4: the half-adder simulation. *)
  let sim = C.make_sim ~inverter_delay:2 ~and_gate_delay:3 ~or_gate_delay:5 () in
  let input_1 = C.make_wire () in
  let input_2 = C.make_wire () in
  let sum = C.make_wire () in
  let carry = C.make_wire () in
  C.probe sim "sum" sum;
  C.probe sim "carry" carry;
  C.half_adder sim input_1 input_2 sum carry;
  input_1.set_signal 1;
  C.propagate sim;
  let lines = List.rev !(sim.trace) in
  List.iter2
    Replay.expect
    [ "sum 0  New-value = 0"; "carry 0  New-value = 0"; "sum 8  New-value = 1" ]
    lines;
  input_2.set_signal 1;
  C.propagate sim;
  let lines = List.rev !(sim.trace) in
  List.iter2
    Replay.expect
    [ "sum 0  New-value = 0"
    ; "carry 0  New-value = 0"
    ; "sum 8  New-value = 1"
    ; "carry 11  New-value = 1"
    ; "sum 16  New-value = 0"
    ]
    lines
;;

let () =
  (* 3.3.5: the Celsius-Fahrenheit network, driven from C and later
     from F. *)
  let c = K.make_connector () in
  let f = K.make_connector () in
  K.celsius_fahrenheit_converter c f;
  let log = ref [] in
  K.probe log "Celsius temp" c;
  K.probe log "Fahrenheit temp" f;
  c.set_value 25 K.User;
  List.iter2
    Replay.expect
    [ "Probe: Celsius temp = 25"; "Probe: Fahrenheit temp = 77" ]
    (List.rev !log);
  (* Setting F while C still holds 25 is a contradiction. *)
  (match
     match f.set_value 212 K.User with
     | () -> false
     | exception Invalid_argument _ -> true
   with
   | true -> ()
   | false -> failwith "expected a contradiction");
  List.iter2
    Replay.expect
    [ "Probe: Celsius temp = 25"; "Probe: Fahrenheit temp = 77" ]
    (List.rev !log);
  c.forget_value K.User;
  List.iter2
    Replay.expect
    [ "Probe: Celsius temp = 25"
    ; "Probe: Fahrenheit temp = 77"
    ; "Probe: Celsius temp = ?"
    ; "Probe: Fahrenheit temp = ?"
    ]
    (List.rev !log);
  f.set_value 212 K.User;
  List.iter2
    Replay.expect
    [ "Probe: Celsius temp = 25"
    ; "Probe: Fahrenheit temp = 77"
    ; "Probe: Celsius temp = ?"
    ; "Probe: Fahrenheit temp = ?"
    ; "Probe: Fahrenheit temp = 212"
    ; "Probe: Celsius temp = 100"
    ]
    (List.rev !log)
;;
