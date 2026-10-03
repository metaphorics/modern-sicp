(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.37: Ben's generator. Ben chooses only [i] and [j] and
   computes the hypotenuse, pruning with [hsq] before it does, so the
   search expands two nested choices where 4.35 expands three. The
   search experiment's own counters measure both complete searches over
   the same bounds: the two procedures answer the same triples in the
   same order, and Ben's expands far fewer choice points and fails far
   fewer branches. The subset has no float-to-integer conversion, so
   Ben's [sqrt] and [integer?] test become an integer square root and
   the check that it squares back to [ksq]; the root is computed, not
   searched, which is the point of Ben's version. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program procedure =
  {|
let rec an_integer_between low high =
  require (low <= high);
  amb low (an_integer_between (low + 1) high)

let show_triple i j k =
  "(" ^ string_of_int i ^ " " ^ string_of_int j ^ " " ^ string_of_int k ^ ")"

let a_pythagorean_triple_between low high =
  let i = an_integer_between low high in
  let j = an_integer_between i high in
  let k = an_integer_between j high in
  require (i * i + j * j = k * k);
  show_triple i j k

let rec integer_sqrt n root =
  if (root + 1) * (root + 1) > n then root else integer_sqrt n (root + 1)

let a_pythagorean_triple_between_ben low high =
  let i = an_integer_between low high in
  let hsq = high * high in
  let j = an_integer_between i high in
  let ksq = i * i + j * j in
  require (hsq >= ksq);
  let k = integer_sqrt ksq 0 in
  require (k * k = ksq);
  show_triple i j k

let () = print_endline (|}
  ^ procedure
  ^ {| 1 20)
|}
;;

let ex_4_37 () =
  transcript (program "a_pythagorean_triple_between")
  @ transcript (program "a_pythagorean_triple_between_ben")
;;
