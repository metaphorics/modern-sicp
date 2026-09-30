(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Check = Sicp_common.Check
module Core = Sicp_ch4.Sec_4_1

(* The lazy list is the program's own [cons] over the subset's lists:
   under the experiment both operands of the compound call stay
   thunks, so the cell holds a delayed element and a delayed tail. *)
let lazy_lists =
  "let cons x y = x :: y\n\
   let head_or default l = match l with x :: _ -> x | [] -> default\n\
   let tail l = match l with _ :: rest -> rest | [] -> []\n"
;;

(* The chapter 3 stream under the strict core: the element is
   evaluated when the cell is built and only the tail is delayed. *)
let streams =
  "type 'a stream = Empty | Cons of 'a * (unit -> 'a stream)\n\
   let stream_car s = match s with Cons (x, _) -> x | Empty -> 0\n\
   let stream_cdr s = match s with Cons (_, rest) -> rest () | Empty -> Empty\n"
;;

let ex_4_32 () =
  [ Core.transcript
      ~experiment:Check.Lazy
      Sicp_ch4.Sec_4_2.run
      (lazy_lists
       ^ "let () = print_int (head_or 0 (tail (cons (1 / 0) (cons 42 [])))); \
          print_newline ()\n")
  ; Core.transcript
      Core.run
      (streams
       ^ "let () =\n\
         \  print_int (stream_car (stream_cdr (Cons (1 / 0, fun _ -> Cons (42, fun _ -> \
          Empty)))))\n")
  ; Core.transcript
      ~experiment:Check.Lazy
      Sicp_ch4.Sec_4_2.run
      (lazy_lists
       ^ "let () = print_int (head_or 0 (cons 7 [ 1 / 0 ])); print_newline ()\n")
  ]
;;
