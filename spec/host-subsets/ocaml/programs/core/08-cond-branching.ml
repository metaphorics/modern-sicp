let grade score =
  if score >= 90
  then "A"
  else if score >= 80 then "B" else if score >= 70 then "C" else "F"

let () = print_endline (grade 95)

let () = print_endline (grade 85)

let () = print_endline (grade 60)
