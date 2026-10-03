(* SPDX-License-Identifier: GPL-3.0-only
   Adapted from SICP section 4.1 exercise 4.21 *)

module S = Sicp_ch4.Sec_4_1

let untyped_factorial =
  "let fact = fun n -> (fun fact -> fact fact n) (fun ft k -> if k = 1 then 1 else k * \
   ft ft (k - 1))\n\
   let () = print_int (fact 10)\n"
;;

let factorial =
  "type 'a fix = Fix of ('a fix -> 'a)\n\
   let self_apply x = match x with Fix f -> f x\n\
   let fact n = self_apply (Fix (fun ft -> fun k -> if k = 1 then 1 else k * self_apply \
   ft (k - 1))) n\n\
   let () = print_int (fact 10)\n"
;;

let fibonacci =
  "type 'a fix = Fix of ('a fix -> 'a)\n\
   let self_apply x = match x with Fix f -> f x\n\
   let fib n = self_apply (Fix (fun fb -> fun k -> if k < 2 then k else self_apply fb (k \
   - 1) + self_apply fb (k - 2))) n\n\
   let () = print_int (fib 10)\n"
;;

let parity =
  "type test = Test of (test -> test -> int -> bool)\n\
   let call p q r n = match p with Test f -> f q r n\n\
   let f x = (fun even odd -> call even even odd x) (Test (fun ev od n -> if n = 0 then \
   true else call od ev od (n - 1))) (Test (fun ev od n -> if n = 0 then false else call \
   ev ev od (n - 1)))\n\
   let () = print_string (if f 10 then \"even\" else \"odd\"); print_string \" \"; \
   print_string (if f 7 then \"even\" else \"odd\")\n"
;;

let ex_4_21 () =
  List.map (S.transcript S.run) [ untyped_factorial; factorial; fibonacci; parity ]
;;
