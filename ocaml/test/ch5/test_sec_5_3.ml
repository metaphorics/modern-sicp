(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Alcotest suite over the section 5.3 memory-vector model and the
   reference solutions' public contracts. The allocator's trace, the
   collector's reconfiguration, the pointer-swap, the stack-as-root
   relocation mid-recursion, and the typed exhausted-memory failure are
   all pinned to the exact observable outcomes; a solution that leans
   on a broken clause cannot pass. *)

module Memory = Sicp_ch5.Sec_5_3
module M = Sicp_ch5.Sec_5_1
module Eval_error = Sicp_common.Eval_error
module S20 = Sicp_ch5_solutions.Sec_5_20
module S21 = Sicp_ch5_solutions.Sec_5_21
module S22 = Sicp_ch5_solutions.Sec_5_22

let ( >>= ) = Result.bind
let strings = Alcotest.(check (list string))
let the_string = Alcotest.check Alcotest.string
let the_int = Alcotest.check Alcotest.int

let strings_outcome name expected = function
  | Ok lines -> strings name expected lines
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

let num n = M.Const (Memory.Num n)

let run_or_fail = function
  | Ok () -> ()
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

(* The typed words: the book's letter-prefixed pointers render back in
   the book's notation, and the predicates check only the type field. *)
let words () =
  let mem = Memory.make_memory ~size:8 ~root_capacity:4 ~free:0 in
  Memory.cons mem (Memory.Num 1) (Memory.Num 2)
  >>= fun x ->
  strings_outcome
    "cell and selectors"
    [ "p0"; "n1"; "p1"; "p2"; "true"; "true"; "true"; "true"; "[1; 7]" ]
    (Memory.cons mem (Memory.Num 7) Memory.Empty
     >>= fun tail ->
     Memory.set_cdr mem x tail
     >>= fun () ->
     Memory.car mem x
     >>= fun a ->
     Memory.cdr mem x
     >>= fun d ->
     Ok
       [ Memory.word_to_string x
       ; Memory.word_to_string a
       ; Memory.word_to_string d
       ; Memory.word_to_string (Memory.free_word mem)
       ; string_of_bool (Memory.is_pair x)
       ; string_of_bool (Memory.is_null Memory.Empty)
       ; string_of_bool (Memory.is_number (Memory.Num 4))
       ; string_of_bool (Memory.is_atom (Memory.Atom "junk"))
       ; Memory.write mem x
       ]);
  Ok ()
;;

(* The stack of 5.3.1: save is a cons onto the-stack, restore a car/cdr
   off it, and the monitored counters of 5.2.4 still answer. *)
let stack () =
  let mem = Memory.make_memory ~size:8 ~root_capacity:4 ~free:0 in
  Memory.make_machine
    ~registers:[ "x" ]
    ~operations:[]
    ~controller:
      [ M.Perform ("initialize-stack", [])
      ; M.Save "x"
      ; M.Save "x"
      ; M.Restore "x"
      ; M.Perform ("print-stack-statistics", [])
      ]
    ~memory:mem
  >>= fun m ->
  Memory.set_register m "x" (Memory.Num 7)
  >>= fun () ->
  Memory.start m
  >>= fun () ->
  Memory.get_register m "the-stack"
  >>= fun stk ->
  strings_outcome
    "stack cells and counters"
    [ "total-pushes = 2 maximum-depth = 2"
    ; "the-stack = p0"
    ; "total-pushes = 2 maximum-depth = 2"
    ]
    (Ok
       (Memory.transcript m
        @ [ "the-stack = " ^ Memory.word_to_string stk; Memory.print_stack_statistics m ]
       ));
  Ok ()
;;

(* The collector's reconfiguration, Figure 5.15: the before-memory is a
   mixture of the list and dropped junk; the after-memory is the list
   compacted at the front of the other semispace, the junk gone, x
   forwarded, and the vector registers swapped. *)
let gc () =
  let mem = Memory.make_memory ~size:14 ~root_capacity:4 ~free:0 in
  Memory.make_machine
    ~registers:[ "x"; "t" ]
    ~operations:[]
    ~controller:
      ((M.Label "build"
        :: M.Perform ("initialize-stack", [])
        :: List.map
             (fun n -> M.Assign_op ("x", "cons", [ num n; M.Reg "x" ]))
             [ 1; 2; 3; 4 ])
       @ List.concat
           (List.init 6 (fun _ ->
              [ M.Assign_op ("t", "cons", [ num 9; num 9 ]); M.Assign ("t", M.Reg "x") ]))
      )
    ~memory:mem
  >>= fun m ->
  Memory.set_register m "x" Memory.Empty
  >>= fun () ->
  Memory.attach_collector m
  >>= fun () ->
  Memory.start m
  >>= fun () ->
  let x_before =
    match Memory.get_register m "x" with
    | Ok (Pair 3) -> 3
    | Ok w -> Alcotest.fail ("x before the collection was " ^ Memory.word_to_string w)
    | Error e -> Alcotest.fail (Eval_error.to_string e)
  in
  Memory.collect_garbage m
  >>= fun () ->
  Memory.get_register m "x"
  >>= fun x_after ->
  the_string "x forwarded" "p1" (Memory.word_to_string x_after);
  the_int "x before the collection" 3 x_before;
  the_int "one collection" 1 (Memory.collections mem);
  the_int "semispaces swapped" 1 (Memory.working mem);
  the_string
    "free after the collection"
    "p7"
    (Memory.word_to_string (Memory.free_word mem));
  strings
    "the after-memory is compacted"
    [ "index    0   1   2   3   4   5   6   7   8   9   10  11  12  13"
    ; "the-cars p1  n4  p1  n3  e0  n2  n1  e0  e0  e0  e0  e0  e0  e0"
    ; "the-cdrs p2  p3  p4  p5  e0  p6  e0  e0  e0  e0  e0  e0  e0  e0"
    ]
    (String.split_on_char '\n' (Memory.dump mem));
  Ok ()
;;

(* The allocation path arms the collector: a machine that fills its
   memory is stopped by the typed exhausted-memory failure, one
   collection having already run. *)
let exhaustion () =
  let mem = Memory.make_memory ~size:8 ~root_capacity:2 ~free:0 in
  Memory.make_machine
    ~registers:[ "x" ]
    ~operations:[]
    ~controller:
      (M.Perform ("initialize-stack", [])
       :: List.map
            (fun n -> M.Assign_op ("x", "cons", [ num n; M.Reg "x" ]))
            [ 1; 2; 3; 4; 5; 6; 7 ])
    ~memory:mem
  >>= fun m ->
  Memory.set_register m "x" Memory.Empty
  >>= fun () ->
  Memory.attach_collector m
  >>= fun () ->
  match Memory.start m with
  | Ok () -> Alcotest.fail "the memory should have run out"
  | Error (Eval_error.Bounds_error detail) ->
    the_string "typed exhaustion" "the memory is exhausted" detail;
    the_int "the collector ran once" 1 (Memory.collections mem);
    Ok ()
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

(* The collector without a machine to root it is a typed failure. *)
let unattached () =
  let mem = Memory.make_memory ~size:4 ~root_capacity:2 ~free:0 in
  Memory.make_machine
    ~registers:[ "x" ]
    ~operations:[]
    ~controller:[ M.Assign ("x", num 1) ]
    ~memory:mem
  >>= fun m ->
  match Memory.collect_garbage m with
  | Ok () -> Alcotest.fail "no collector is attached"
  | Error (Eval_error.Invalid_form detail) ->
    the_string
      "typed unattached collector"
      "no collector is attached to the machine"
      detail;
    Ok ()
  | Error e -> Alcotest.fail (Eval_error.to_string e)
;;

(* The deepest pin: the collector fires in the middle of the recursive
   count-leaves run -- the machine's stack is a chain of saved trees in
   the working memory -- and the run completes with the right answer,
   the stack relocated into the flipped semispace. *)
let stack_root () =
  let mem = Memory.make_memory ~size:32 ~root_capacity:6 ~free:0 in
  S21.plant
    mem
    (S21.Node
       [ S21.Leaf 1
       ; S21.Leaf 2
       ; S21.Node [ S21.Leaf 3; S21.Node [ S21.Leaf 4; S21.Leaf 5 ] ]
       ])
  >>= fun root ->
  Memory.make_machine
    ~registers:[ "tree"; "val"; "n"; "continue"; "temp" ]
    ~operations:[]
    ~controller:S21.recursive_controller
    ~memory:mem
  >>= fun m ->
  Memory.set_register m "tree" root
  >>= fun () ->
  Memory.attach_collector m
  >>= fun () ->
  Memory.start m
  >>= fun () ->
  Memory.get_register m "val"
  >>= fun v ->
  the_string "the count survives the collection" "n5" (Memory.word_to_string v);
  the_int "the collector fired" 1 (Memory.collections mem);
  the_int "the stack moved with the flip" 1 (Memory.working mem);
  Ok ()
;;

(* Exercise 5.20: the cells come out at p1, p2, p3; x is p1, y is p3,
   free ends at p4, and both elements of y are p1. *)
let ex_5_20 () =
  strings_outcome
    "the two drawings' answers"
    [ "index    0   1   2   3   4   5   6   7\n\
       the-cars e0  n1  p1  p1  e0  e0  e0  e0\n\
       the-cdrs e0  n2  e0  p2  e0  e0  e0  e0"
    ; "x = p1"
    ; "y = p3"
    ; "free = p4"
    ]
    (S20.ex_5_20 ())
;;

(* Exercise 5.20a: the allocator's trace over the same three conses. *)
let ex_5_20a () =
  strings_outcome
    "the allocator's trace"
    [ "cons -> p1 = (n1, n2); free p1 -> p2"
    ; "cons -> p2 = (p1, e0); free p2 -> p3"
    ; "cons -> p3 = (p1, p2); free p3 -> p4"
    ; "x = p1"
    ; "y = p3"
    ; "free = p4"
    ]
    (S20.ex_5_20a ())
;;

(* Exercise 5.21: both machines answer with the oracle, and the
   recursive machine pays more stack for it. *)
let ex_5_21 () =
  strings_outcome
    "both machines beside the oracle"
    [ "[1; 2; [3; [4; 5]]]: recursive n5, iterative n5, oracle 5;"
      ^ " recursive stack total-pushes = 21 maximum-depth = 14,"
      ^ " iterative stack total-pushes = 14 maximum-depth = 10"
    ; "[[7]]: recursive n1, iterative n1, oracle 1;"
      ^ " recursive stack total-pushes = 6 maximum-depth = 4,"
      ^ " iterative stack total-pushes = 4 maximum-depth = 4"
    ; "[]: recursive n0, iterative n0, oracle 0;"
      ^ " recursive stack total-pushes = 0 maximum-depth = 0,"
      ^ " iterative stack total-pushes = 0 maximum-depth = 0"
    ]
    (S21.ex_5_21 ())
;;

(* Exercise 5.22: append copies, append! splices, and the dumps are the
   before/after drawings the book asks for. *)
let ex_5_22 () =
  strings_outcome
    "append and append!"
    [ "append: z = p13 = [1; 2; 3; 4; 5]"
    ; "append: x is still [1; 2; 3] (p2), free moved to p14 -- three fresh cells for the"
      ^ " copy, six for the saved words"
    ; "append!: before, the last pair of x points at e0:\n"
      ^ "index    0   1   2   3   4   5   6   7\n"
      ^ "the-cars n3  n2  n1  n5  n4  e0  e0  e0\n"
      ^ "the-cdrs e0  p0  p1  e0  p3  e0  e0  e0"
    ; "append!: after, it points at y:\n"
      ^ "index    0   1   2   3   4   5   6   7\n"
      ^ "the-cars n3  n2  n1  n5  n4  e0  e0  e0\n"
      ^ "the-cdrs p4  p0  p1  e0  p3  e0  e0  e0"
    ; "append!: the answer is x itself, now [1; 2; 3; 4; 5] -- the same pointer p2"
      ^ " the caller passed, and free is still p5"
    ]
    (S22.ex_5_22 ())
;;

let () =
  let open Alcotest in
  run
    "test_sec_5_3"
    [ ( "words"
      , [ test_case "typed words and selectors" `Quick (fun () -> run_or_fail (words ()))
        ] )
    ; ( "stack"
      , [ test_case "save and restore as list operations" `Quick (fun () ->
            run_or_fail (stack ()))
        ] )
    ; ( "gc"
      , [ test_case "stop-and-copy reconfiguration" `Quick (fun () -> run_or_fail (gc ()))
        ] )
    ; ( "exhaustion"
      , [ test_case "the typed exhausted-memory failure" `Quick (fun () ->
            run_or_fail (exhaustion ()))
        ] )
    ; ( "unattached"
      , [ test_case "collecting with no collector" `Quick (fun () ->
            run_or_fail (unattached ()))
        ] )
    ; ( "stack-root"
      , [ test_case "the stack relocated mid-recursion" `Quick (fun () ->
            run_or_fail (stack_root ()))
        ] )
    ; "ex_5_20", [ test_case "the two drawings" `Quick ex_5_20 ]
    ; "ex_5_20a", [ test_case "the allocator's trace" `Quick ex_5_20a ]
    ; "ex_5_21", [ test_case "count-leaves machines and oracle" `Quick ex_5_21 ]
    ; "ex_5_22", [ test_case "append and append!" `Quick ex_5_22 ]
    ]
;;
