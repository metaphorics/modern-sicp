(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* The section replay: the list-structure memory of 5.3.1 and the
   stop-and-copy collector of 5.3.2, results proved with [expect]. *)

module Mem = Sicp_ch5.Sec_5_3
module M = Sicp_ch5.Sec_5_1
module Replay = Sicp_ch1.Replay

let ( let* ) = Result.bind

let show = function
  | Ok s -> s
  | Error e -> "error: " ^ Sicp_common.Eval_error.to_string e
;;

(* 5.3.1: cons allocates at the free pointer; set-cdr! stores through
   the pointer's index. *)
let vector_demo =
  let mem = Mem.make_memory ~size:8 ~root_capacity:2 ~free:0 in
  let* x = Mem.cons mem (Mem.Num 1) (Mem.Num 2) in
  let* tail = Mem.cons mem (Mem.Num 7) Mem.Empty in
  let* () = Mem.set_cdr mem x tail in
  Ok
    (Mem.word_to_string x
     ^ " "
     ^ Mem.write mem x
     ^ " free "
     ^ Mem.word_to_string (Mem.free_word mem))
;;

(* Figure 5.15: six list cells interleaved with six garbage cells fill
   the twelve data cells of a 16-cell semispace (4 reserved for the
   root list), so consing a seventh element exhausts the memory.  The
   collector copies the three root-list cells ([x], [t], [the-stack]),
   the six live list cells, and the one garbage cell [t] still holds:
   ten cells, so after the retried cons the free pointer is p11. *)
let gc_demo =
  let mem = Mem.make_memory ~size:16 ~root_capacity:4 ~free:0 in
  let fill =
    List.concat_map
      (fun i ->
         [ M.Assign_op ("t", "cons", [ M.Const (Mem.Num 0); M.Const Mem.Empty ])
         ; M.Assign_op ("x", "cons", [ M.Const (Mem.Num i); M.Reg "x" ])
         ])
      [ 1; 2; 3; 4; 5; 6 ]
  in
  let* m =
    Mem.make_machine
      ~registers:[ "x"; "t" ]
      ~operations:[]
      ~controller:
        ((M.Assign ("x", M.Const Mem.Empty) :: fill)
         @ [ M.Assign_op ("x", "cons", [ M.Const (Mem.Num 7); M.Reg "x" ]) ])
      ~memory:mem
  in
  let* () = Mem.attach_collector m in
  let* () = Mem.start m in
  let* x = Mem.get_register m "x" in
  Ok
    (Printf.sprintf
       "%s collections %d free %s"
       (Mem.write mem x)
       (Mem.collections mem)
       (Mem.word_to_string (Mem.free_word mem)))
;;

let () =
  Replay.expect (show vector_demo) "p0 [1; 7] free p2";
  Replay.expect (show gc_demo) "[7; 6; 5; 4; 3; 2; 1] collections 1 free p11"
;;
