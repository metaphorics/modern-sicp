type 'a tree =
  | Leaf of 'a
  | Node of 'a tree * 'a tree

let rec count_leaves tree =
  match tree with
  | Leaf _ -> 1
  | Node (left, right) -> count_leaves left + count_leaves right

let () =
  print_int (count_leaves (Node (Node (Leaf 1, Leaf 2), Node (Node (Leaf 3, Leaf 4), Leaf 5))))

let () = print_newline ()
