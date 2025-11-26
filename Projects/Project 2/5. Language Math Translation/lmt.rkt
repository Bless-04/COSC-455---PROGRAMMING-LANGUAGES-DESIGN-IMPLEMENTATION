#lang racket

;;; scheme comment
;;; matches chinese or english word to number or #f
(define (match-word input)
  (cond
    ((or (string-ci=? input "ling") (string-ci=? input "zero")) 0)
    ((or (string-ci=? input "yi") (string-ci=? input "one")) 1)
    ((or (string-ci=? input "er") (string-ci=? input "two")) 2)
    ((or (string-ci=? input "san") (string-ci=? input "three")) 3)
    ((or (string-ci=? input "si") (string-ci=? input "four")) 4)
    ((or (string-ci=? input "wu") (string-ci=? input "five")) 5)
    ((or (string-ci=? input "liu") (string-ci=? input "six")) 6)
    ((or (string-ci=? input "qi") (string-ci=? input "seven")) 7)
    ((or (string-ci=? input "ba") (string-ci=? input "eight")) 8)
    ((or (string-ci=? input "jiu") (string-ci=? input "nine")) 9)
    ((or (string-ci=? input "shi") (string-ci=? input "ten")) 10)
    (else #f)))

;;; n+...n = sum
(define (print-sum lst sum)
  (if (null? lst)
      sum
      (let* ((val (car lst))
             (new-sum (+ sum val)))
        (if (null? (cdr lst))
            (begin
              (display val)
              (display " = ")
              (display new-sum)
              (newline)
              new-sum)
            (begin
              (display val)
              (display " + ")
              (print-sum (cdr lst) new-sum))))))

;;; n*...n = product
(define (print-product lst product)
  (if (null? lst)
      product
      (let* ((val (car lst))
             (new-product (* product val)))
        (if (null? (cdr lst))
            (begin
              (display val)
              (display " = ")
              (display new-product)
              (newline)
              new-product)
            (begin
              (display val)
              (display " * ")
              (print-product (cdr lst) new-product))))))

;;;  filters strings to numbers
(define (translate-input input)
  (if (null? input)
      '()
      (let ((number (match-word (car input))))
        (if number
            (cons number (translate-input (cdr input)))
            (translate-input (cdr input))))))

(define (go input)
  (if (null? input)
      '()
      (let ((result (translate-input input)))
        
        (display "Translation: ")
        (display result)
        (newline)

        (if (not (null? result))
            (begin
               (display "Addition: ")
               (print-sum result 0)
               
               (display "Multiplication: ")
               (print-product result 1)
               (newline))
            #f) ;;; #f fixed the else issue
        
        result)))
;;; exists to test
(define (assert-eq expected actual message)
  (if (equal? expected actual)
      (begin 
        (display "WORKING:") 
        (display message) 
        (newline))
      (begin 
        (display "FAILING: ") 
        (display message) 
        (display " | Expected: ") (display expected)
        (display " Got: ") (display actual)
        (newline))))

(define (main)
  (let ((input '("yi" "nine" "six" "ba")))
    (go input))

  (let ((input '("yi" "josh" "three" "si")))
    (go input))

  (let ((numbers '(1 2 3 4 5 6 7 8 9 10)))
    (assert-eq 
     numbers
     (go '("skip" "yi" "er" "san" "si" "wu" "liu" "qi" "ba" "jiu" "shi"))
     "Chinese Check")

    (assert-eq 
     numbers
     (go '("skip" "one" "two" "three" "four" "five" "six" "seven" "eight" "nine" "ten"))
     "English Check")))

;;;
(main)