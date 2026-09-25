(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** Exercise 5.22: [append] and [append!] as register machines over the
    list-structure memory, with the memory dumps the book asks to be
    drawn. *)

let ( >>= ) = Result.bind

module Memory = Sicp_ch5.Sec_5_3

(** [list_word mem ints] plants a proper list of numbers. *)
let list_word mem ints =
  let rec spine = function
    | [] -> Ok Memory.Empty
    | n :: rest -> spine rest >>= fun d -> Memory.cons mem (Memory.Num n) d
  in
  spine ints
;;

(** The [append] machine: it copies [x], cell by cell, and shares [y];
    the answer lands in [z]. *)
let append_controller =
  {|(controller
   (assign continue (label append-done))
 append-loop
   (test (op null?) (reg x))
   (branch (label base))
   (assign temp (op car) (reg x))
   (save temp)
   (save continue)
   (assign continue (label after-car))
   (assign x (op cdr) (reg x))
   (goto (label append-loop))
 base
   (assign z (reg y))
   (goto (reg continue))
 after-car
   (restore continue)
   (restore temp)
   (assign z (op cons) (reg temp) (reg z))
   (goto (reg continue))
 append-done)|}
;;

(** The [append!] machine: it walks to the last pair of [x] and splices
    [y] in with a [set-cdr!]; no cell is allocated, and there is no
    [z] -- the value of [x] is the answer. *)
let append_bang_controller =
  {|(controller
   (assign temp (reg x))
 last-pair
   (assign cand (op cdr) (reg temp))
   (test (op null?) (reg cand))
   (branch (label splice))
   (assign temp (op cdr) (reg temp))
   (goto (label last-pair))
 splice
   (perform (op set-cdr!) (reg temp) (reg y)))|}
;;

(** [run controller result mem x y] runs one machine over the planted
    lists and reads the result register. *)
let run controller result mem x y =
  Memory.make_machine
    ~registers:[ "x"; "y"; "z"; "temp"; "cand"; "continue" ]
    ~operations:[]
    ~controller
    ~memory:mem
  >>= fun m ->
  Memory.set_register m "x" x
  >>= fun () ->
  Memory.set_register m "y" y
  >>= fun () -> Memory.start m >>= fun () -> Memory.get_register m result
;;

(** [ex_5_22 ()] plants [x = (1 2 3)] and [y = (4 5)] and runs
    [append]: three fresh cells, [z = (1 2 3 4 5)], [x] untouched. It
    then runs [append!] on a fresh copy and dumps the memory before and
    after: the last pair of [x] changes its cdr from [e0] to [y], [z]
    is nowhere, and the free pointer did not move. *)
let ex_5_22 () =
  let mem = Memory.make_memory ~size:32 ~root_capacity:8 ~free:0 in
  list_word mem [ 1; 2; 3 ]
  >>= fun x1 ->
  list_word mem [ 4; 5 ]
  >>= fun y1 ->
  run append_controller "z" mem x1 y1
  >>= fun z1 ->
  let appended = Memory.write mem z1 in
  let free_after_append = Memory.word_to_string (Memory.free_word mem) in
  let x1_written = Memory.write mem x1 in
  let mem2 = Memory.make_memory ~size:8 ~root_capacity:4 ~free:0 in
  list_word mem2 [ 1; 2; 3 ]
  >>= fun x2 ->
  list_word mem2 [ 4; 5 ]
  >>= fun y2 ->
  let before = Memory.dump mem2 in
  run append_bang_controller "x" mem2 x2 y2
  >>= fun x2_spliced ->
  let after = Memory.dump mem2 in
  Ok
    [ "append: z = " ^ Memory.word_to_string z1 ^ " = " ^ appended
    ; "append: x is still "
      ^ x1_written
      ^ " ("
      ^ Memory.word_to_string x1
      ^ "), free moved to "
      ^ free_after_append
      ^ " -- three fresh cells"
    ; "append!: before, the last pair of x points at e0:\n" ^ before
    ; "append!: after, it points at y:\n" ^ after
    ; "append!: the answer is x itself, now "
      ^ Memory.write mem2 x2_spliced
      ^ " -- the same pointer "
      ^ Memory.word_to_string x2_spliced
      ^ " the caller passed, and free is still "
      ^ Memory.word_to_string (Memory.free_word mem2)
    ]
;;
