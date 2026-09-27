(define (count-down n)
  (if (= n 0)
      'done
      (count-down (- n 1))))
(count-down 100000)
(define (sum-to n acc)
  (if (= n 0)
      acc
      (sum-to (- n 1) (+ acc n))))
(sum-to 1000 0)
