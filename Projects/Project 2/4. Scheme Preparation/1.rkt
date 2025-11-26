#lang racket
(define first (lambda (x) (car x)))
(define second (lambda (x) (cadr x)))
(define rest (lambda (x) (cdr x)))

(define third (lambda (x) (caddr x)))
(define fourth (lambda (x) (cadddr x)))
(define fifth (lambda (x) (car (cddddr x))))

(define family '(josh sara erin sandy jon))
(display "input:") (display family) (newline)
(display "3rd: ") (display (third family)) (newline)
(display "4th: ") (display (fourth family)) (newline)
(display "5th: ") (display (fifth family)) (newline)
(newline)