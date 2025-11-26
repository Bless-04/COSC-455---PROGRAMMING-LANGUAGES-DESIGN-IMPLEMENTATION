#lang racket
(define (truecount lst)
  (cond
    ((null? lst) 0) ;  empty  = 0
    ((eq? (car lst) true) (+ 1 (truecount (cdr lst)))) ; true = +1
    (else (truecount (cdr lst)))) ); false = continue

(display (truecount '(#t #f #t #t #f))) ;should be 3
(newline)
(newline)