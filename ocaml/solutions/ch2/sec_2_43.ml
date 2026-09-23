(* SPDX-License-Identifier: GPL-3.0-only
   Reference solution of SICP section 2.2 exercise 2.43: the swapped-mapping queens. *)

let rec enumerate_interval low high =
  if low > high then [] else low :: enumerate_interval (low + 1) high
;;

let flatmap proc seq = List.concat_map proc seq
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

let queens board_size =
  let rec queen_cols k =
    if k = 0
    then [ [] ]
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

(* Louis's version interchanges the nested mappings: the recursive call
   to [queen_cols] is now inside the row map, so it runs once per
   proposed row instead of once per column. *)
let louis_queens board_size =
  let rec queen_cols k =
    if k = 0
    then [ [] ]
    else
      List.filter
        (fun positions -> safe k positions)
        (flatmap
           (fun new_row ->
              List.map
                (fun rest_of_queens -> adjoin_position new_row k rest_of_queens)
                (queen_cols (k - 1)))
           (enumerate_interval 1 board_size))
  in
  queen_cols board_size
;;

(* Louis's nested mappings visit the search tree in a different
   order, so the two versions are compared as solution sets. *)
let ex_2_43 () =
  let a = louis_queens 6 in
  let b = queens 6 in
  List.length a = List.length b && List.for_all (fun s -> List.mem s b) a
;;
