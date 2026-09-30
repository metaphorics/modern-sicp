(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.35: [an_integer_between] and the bounded Pythagorean
   triples. The guest program runs under the section's search
   experiment, which prints every successful branch in the order its
   depth-first search reaches it: ascending [i], then [j], then [k]. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program =
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

let () = print_endline (a_pythagorean_triple_between 1 20)
|}
;;

let ex_4_35 () = transcript program
