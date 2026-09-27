(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the memory-vector model the section's listings
   build runs, and every result comment the adapted prose displays is
   proved with [expect]. *)

module Machine = Sicp_ch5.Sec_5_3
module Replay = Sicp_ch1.Replay

let ( >>= ) = Result.bind

(* The vector-memory primitives of 5.3.1: a cons allocates at the free
   pointer, the selectors go through the index part of the pair
   pointer, and a mutator stores through it. *)
let vector_demo () =
  let mem = Machine.make_memory ~size:8 ~root_capacity:4 ~free:0 in
  Machine.cons mem (Machine.Num 1) (Machine.Num 2)
  >>= fun x ->
  Machine.car mem x
  >>= fun a ->
  Machine.cdr mem x
  >>= fun d ->
  Machine.cons mem (Machine.Num 7) Machine.Empty
  >>= fun tail ->
  Machine.set_cdr mem x tail
  >>= fun () ->
  Machine.cdr mem x
  >>= fun d2 ->
  Ok
    ( Machine.word_to_string x
    , Machine.word_to_string a
    , Machine.word_to_string d
    , Machine.word_to_string d2 )
;;

(* The stack of 5.3.1: save and restore are the cons onto the-stack and
   the car/cdr off it, so a save/save/restore leaves one saved cell in
   the memory and the monitored counters of 5.2.4. *)
let stack_demo () =
  let mem = Machine.make_memory ~size:8 ~root_capacity:4 ~free:0 in
  Machine.make_machine
    ~registers:[ "x" ]
    ~operations:[]
    ~controller:
      {|(controller
   (perform (op initialize-stack))
   (save x)
   (save x)
   (restore x)
   (perform (op print-stack-statistics)))|}
    ~memory:mem
  >>= fun m ->
  Machine.set_register m "x" (Machine.Num 7)
  >>= fun () ->
  Machine.start m
  >>= fun () ->
  Machine.get_register m "the-stack"
  >>= fun stk ->
  Ok (Machine.transcript m, Machine.word_to_string stk, Machine.print_stack_statistics m)
;;

(* The allocation path and the collector, the reconfiguration of
   Figure 5.15: the machine fills its working memory with a short list
   and dropped junk -- the mixture of useful data and garbage -- then
   collects, and the useful data comes back compacted at the front of
   the other semispace, the junk gone and the vector registers
   swapped. Building continues in the new working memory. *)
let gc_demo () =
  let mem = Machine.make_memory ~size:14 ~root_capacity:4 ~free:0 in
  Machine.make_machine
    ~registers:[ "x"; "t" ]
    ~operations:[]
    ~controller:
      {|(controller
 build
   (perform (op initialize-stack))
   (assign x (op cons) (const 1) (reg x))
   (assign x (op cons) (const 2) (reg x))
   (assign x (op cons) (const 3) (reg x))
   (assign x (op cons) (const 4) (reg x))
   (assign t (op cons) (const 9) (const 9))
   (assign t (reg x))
   (assign t (op cons) (const 9) (const 9))
   (assign t (reg x))
   (assign t (op cons) (const 9) (const 9))
   (assign t (reg x))
   (assign t (op cons) (const 9) (const 9))
   (assign t (reg x))
   (assign t (op cons) (const 9) (const 9))
   (assign t (reg x))
   (assign t (op cons) (const 9) (const 9))
   (assign t (reg x)))|}
    ~memory:mem
  >>= fun m ->
  Machine.set_register m "x" Machine.Empty
  >>= fun () ->
  Machine.attach_collector m
  >>= fun () ->
  Machine.start m
  >>= fun () ->
  Machine.get_register m "x"
  >>= fun x ->
  let before = Machine.dump mem in
  Machine.collect_garbage m
  >>= fun () ->
  Machine.get_register m "x"
  >>= fun gx ->
  let after = Machine.dump mem in
  Machine.make_machine
    ~registers:[ "x" ]
    ~operations:[]
    ~controller:
      {|(controller
 more
   (assign x (op cons) (const 5) (reg x))
   (assign x (op cons) (const 6) (reg x)))|}
    ~memory:mem
  >>= fun m2 ->
  Machine.set_register m2 "x" gx
  >>= fun () ->
  Machine.attach_collector m2
  >>= fun () ->
  Machine.start m2
  >>= fun () ->
  Machine.get_register m2 "x"
  >>= fun x2 ->
  Ok
    ( before
    , after
    , Machine.word_to_string x
    , Machine.word_to_string gx
    , Machine.write mem x2
    , Machine.word_to_string (Machine.free_word mem)
    , Machine.collections mem
    , Machine.working mem )
;;

(* The illusion has an end: a memory filled to its limit refuses the
   next cons with the typed exhausted-memory failure, even after the
   collector has done its work. *)
let exhaustion_demo () =
  let mem = Machine.make_memory ~size:8 ~root_capacity:2 ~free:0 in
  Machine.make_machine
    ~registers:[ "x" ]
    ~operations:[]
    ~controller:
      {|(controller
   (perform (op initialize-stack))
   (assign x (op cons) (const 1) (reg x))
   (assign x (op cons) (const 2) (reg x))
   (assign x (op cons) (const 3) (reg x))
   (assign x (op cons) (const 4) (reg x))
   (assign x (op cons) (const 5) (reg x))
   (assign x (op cons) (const 6) (reg x))
   (assign x (op cons) (const 7) (reg x)))|}
    ~memory:mem
  >>= fun m ->
  Machine.set_register m "x" Machine.Empty
  >>= fun () ->
  Machine.attach_collector m
  >>= fun () ->
  (match Machine.start m with
   | Ok () -> Ok "ran"
   | Error e -> Ok ("Error: " ^ Machine.error_to_string e))
  >>= fun outcome ->
  Ok (outcome, Machine.collections mem, Machine.word_to_string (Machine.free_word mem))
;;

let show show_v = function
  | Ok v -> show_v v
  | Error e -> "Error: " ^ Machine.error_to_string e
;;

let () =
  Replay.expect
    (show (fun (a, b, c, d) -> String.concat " " [ a; b; c; d ]) (vector_demo ()))
    "p0 n1 n2 p1";
  Replay.expect
    (show
       (fun (lines, stack, stats) ->
          String.concat " | " (lines @ [ "the-stack = " ^ stack; stats ]))
       (stack_demo ()))
    "total-pushes = 2 maximum-depth = 2 | the-stack = p0 | total-pushes = 2 \
     maximum-depth = 2";
  Replay.expect
    (show
       (fun (before, after, x, gx, written, free, collections, working) ->
          String.concat
            "\n"
            [ "== before =="
            ; before
            ; "== after =="
            ; after
            ; "x before = " ^ x ^ ", after the collection = " ^ gx
            ; "x after building on = " ^ written
            ; "free = " ^ free
            ; "collections = " ^ string_of_int collections
            ; "working = " ^ string_of_int working
            ])
       (gc_demo ()))
    (String.trim
       {|
== before ==
index    0   1   2   3   4   5   6   7   8   9   10  11  12  13
the-cars n1  n2  n3  n4  n9  n9  n9  n9  n9  n9  e0  e0  e0  e0
the-cdrs e0  p0  p1  p2  n9  n9  n9  n9  n9  n9  e0  e0  e0  e0
== after ==
index    0   1   2   3   4   5   6   7   8   9   10  11  12  13
the-cars p1  n4  p1  n3  e0  n2  n1  e0  e0  e0  e0  e0  e0  e0
the-cdrs p2  p3  p4  p5  e0  p6  e0  e0  e0  e0  e0  e0  e0  e0
x before = p3, after the collection = p1
x after building on = (6 5 4 3 2 1)
free = p9
collections = 1
working = 1|});
  Replay.expect
    (show
       (fun (outcome, collections, free) ->
          outcome ^ " | collections = " ^ string_of_int collections ^ " | free = " ^ free)
       (exhaustion_demo ()))
    "Error: operation failed: the memory is exhausted | collections = 1 | free = p8"
;;
