let rec sqrt_iter guess x =
  let delta = guess *. guess -. x in
  let close = if delta < 0.0 then -. delta else delta in
  if close < 0.001 then guess else sqrt_iter ((guess +. x /. guess) /. 2.0) x

let () = print_endline (string_of_float (sqrt_iter 1.0 2.0))
