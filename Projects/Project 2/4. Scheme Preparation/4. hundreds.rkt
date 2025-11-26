#lang racket
(define (hundreds? l)
  (filter (lambda (n) (> n 100)) l))

(display "Filter >100 in '(10 105 99 100 200 50 1000): ")
(display (hundreds? '(10 105 99 100 200 50 1000)))
(newline)
(newline)
