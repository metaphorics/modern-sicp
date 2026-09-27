(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.42: the eight-queens puzzle. *)

let rec enumerate_interval low high =
  if low > high then [] else low :: enumerate_interval (low + 1) high
;;

let flatmap proc seq = List.concat_map proc seq
let empty_board = []
let adjoin_position row column positions = (row, column) :: positions

let safe k positions =
  match List.find_opt (fun (_, column) -> column = k) positions with
  | None -> true
  | Some (row_k, column_k) ->
    List.for_all
      (fun (row, column) ->
         column = column_k
         || (row <> row_k && abs (row - row_k) <> abs (column - column_k)))
      positions
;;

let ex_2_42 board_size =
  let rec queen_cols k =
    if k = 0
    then [ empty_board ]
    else
      List.filter
        (fun positions -> safe k positions)
        (flatmap
           (fun rest_of_queens ->
              List.map
                (fun new_row -> adjoin_position new_row k rest_of_queens)
                (enumerate_interval 1 board_size))
           (queen_cols (k - 1)))
  in
  queen_cols board_size
;;
