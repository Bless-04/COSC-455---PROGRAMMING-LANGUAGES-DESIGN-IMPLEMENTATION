#lang racket
(define (collatz n)
  (display n) (newline)
  (cond
    [(eq? n 1) 'end ]                     ; end when 1
    [(even? n) (collatz (/ n 2))]         ; n even 
    [else       (collatz (+ (* 3 n) 1))])) ; n odd


