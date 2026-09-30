(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(* Exercise 4.44 and this edition's 4.44a: the eight-queens puzzle under
   [amb]. [place] chooses a row for each column in turn and requires it
   safe against the queens already placed, so a doomed column is
   abandoned before any later column is chosen. A placement prints as
   its rows from the last column to the first, the order [place] conses
   them. The search experiment finds every placement: 92 on the board
   of eight, 2 on four, 4 on six, and the first placement it reaches on
   each board is the one the book's first answer gives. *)

module Check = Sicp_common.Check

let transcript source =
  Sicp_ch4.Sec_4_1.transcript ~experiment:Check.Search Sicp_ch4.Sec_4_3.run source
  |> String.split_on_char '\n'
  |> List.filter (fun line -> line <> "")
;;

let program board_size =
  {|
let rec an_integer_between low high =
  require (low <= high);
  amb low (an_integer_between (low + 1) high)

let abs n = if n < 0 then - n else n

let attacks row1 col1 row2 col2 = row1 = row2 || abs (row1 - row2) = abs (col1 - col2)

let rec safe_up_to new_row new_col rows col =
  match rows with
  | [] -> true
  | row :: rest ->
    (not (attacks new_row new_col row col)) && safe_up_to new_row new_col rest (col - 1)

let queens board_size =
  let rec place col rows =
    if col > board_size then rows
    else (
      let row = an_integer_between 1 board_size in
      require (safe_up_to row col rows (col - 1));
      place (col + 1) (row :: rows))
  in
  place 1 []

let rec show_rows rows =
  match rows with
  | [] -> ""
  | [ row ] -> string_of_int row
  | row :: rest -> string_of_int row ^ " " ^ show_rows rest

let () = print_endline ("(" ^ show_rows (queens |}
  ^ string_of_int board_size
  ^ {|) ^ ")")
|}
;;

let queens board_size = transcript (program board_size)

let first_and_counts lines =
  match lines with
  | [] -> []
  | first :: _ -> first :: List.filter (fun line -> String.contains line ':') lines
;;

let ex_4_44 () =
  first_and_counts (queens 8) @ first_and_counts (queens 4) @ first_and_counts (queens 6)
;;

let ex_4_44a () =
  List.concat_map
    (fun board_size ->
       ("board " ^ string_of_int board_size)
       :: List.filter (fun line -> String.contains line ':') (queens board_size))
    [ 4; 5; 6 ]
;;
