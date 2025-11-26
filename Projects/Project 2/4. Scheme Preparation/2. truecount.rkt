#lang racket
(define (truecount lst)
  (cond
    ((null? lst) 0) ;  empty  = 0
    ((eq? (car lst) #t) (+ 1 (truecount (cdr lst)))) ; true = +1
    (else (truecount (cdr lst)))) ); false = continue

(display "Count #t in '(#t #f #t #t #f): ")
(display (truecount '(#t #f #t #t #f)))
(newline)
(newline)