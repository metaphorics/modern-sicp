(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from the Scheme program of SICP section 5.3 *)

(** Exercise 5.20: the box-and-pointer and memory-vector drawings of
    [let x = (1, 2)] and [let y = [x; x]] with the free pointer
    initially at [p1], and Exercise 5.20a's allocator trace. *)

let ( >>= ) = Result.bind

module Memory = Sicp_ch5.Sec_5_3

(** [draw mem x y] renders the memory-vector drawing and the two
    pointer answers. *)
let draw mem x y =
  Ok
    (Memory.dump mem
     :: [ "x = " ^ Memory.word_to_string x
        ; "y = " ^ Memory.word_to_string y
        ; "free = " ^ Memory.word_to_string (Memory.free_word mem)
        ])
;;

(** [ex_5_20 ()] runs the exercise's two definitions through the
    allocation path: the cells come out at [p1], [p2], and [p3] -- the
    innermost cons of [[x; x]], that is [x :: []], allocates first, since a register
    machine needs the cdr argument before it can fill the outer cell --
    so [x] is [p1], [y] is [p3], and the final value of [free] is
    [p4]. Both elements of [y] name the same cell [p1]: the sharing the
    box-and-pointer drawing shows as two arrows into one box. *)
let ex_5_20 () =
  let mem = Memory.make_memory ~size:8 ~root_capacity:4 ~free:1 in
  Memory.cons mem (Memory.Num 1) (Memory.Num 2)
  >>= fun x ->
  Memory.cons mem x Memory.Empty
  >>= fun inner -> Memory.cons mem x inner >>= fun y -> draw mem x y
;;

(** [ex_5_20a ()] is the same three conses with the allocator's trace
    on: one line per cons, the free pointer before and after each, and
    the cell it filled. *)
let ex_5_20a () =
  let mem = Memory.make_memory ~size:8 ~root_capacity:4 ~free:1 in
  Memory.cons mem (Memory.Num 1) (Memory.Num 2)
  >>= fun x ->
  Memory.cons mem x Memory.Empty
  >>= fun inner ->
  Memory.cons mem x inner
  >>= fun y ->
  Ok
    (Memory.allocation_trace mem
     @ [ "x = " ^ Memory.word_to_string x
       ; "y = " ^ Memory.word_to_string y
       ; "free = " ^ Memory.word_to_string (Memory.free_word mem)
       ])
;;
