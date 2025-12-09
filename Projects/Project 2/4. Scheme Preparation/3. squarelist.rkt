#lang racket

(define (squarelist l)
  (map (lambda (x) (* x x)) l))

(display "Square '(1 2 3 4 5): ")
(display (squarelist '(1 2 3 4 5)))
(newline)
(newline)