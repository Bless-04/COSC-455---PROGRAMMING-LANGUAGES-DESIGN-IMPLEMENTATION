#lang racket


;;; From prev questions
(define (first x) (car x))
(define (second x) (cadr x))
(define (rest x) (cdr x))

;;; mapping the words to numbers
(define translation-map
  '(("ling" 0) ("zero" 0)
    ("yi" 1)   ("one" 1)
    ("er" 2)   ("two" 2)
    ("san" 3)  ("three" 3)
    ("si" 4)   ("four" 4)
    ("wu" 5)   ("five" 5)
    ("liu" 6)  ("six" 6)
    ("qi" 7)   ("seven" 7)
    ("ba" 8)   ("eight" 8)
    ("jiu" 9)  ("nine" 9)
    ("shi" 10) ("ten" 10)))


;;; match-word: Matches word to number
(define (match-word input)
  (let ((found-pair (assoc (string-downcase input) translation-map)))
    (if found-pair
        (second found-pair) 
        #f))) ; must have #f or problems

;;; converts the words to numbers
(define (translate-input input)
  (cond
    ((null? input) '()) ; empty list for basecase
    (else
     (let ((word (first input))) 
       (let ((num (match-word word)))
         (if num
             ;;  keep  and process  rest if valid
             (cons num (translate-input (rest input)))
             ;; skip when not valid
             (translate-input (rest input))))))))

(define (print-sum lst sum)
  (if (null? lst)
      sum
      (let* ((val (first lst))
             (new-sum (+ sum val)))
        (if (null? (rest lst))
            (begin
              (display val) (display " = ") (display new-sum) (newline)
              new-sum)
            (begin
              (display val) (display " + ")
              (print-sum (rest lst) new-sum))))))


(define (print-product lst product)
  (if (null? lst)
      product
      (let* ((val (first lst))
             (new-product (* product val)))
        (if (null? (rest lst))
            (begin
              (display val) (display " = ") (display new-product) (newline)
              new-product)
            (begin
              (display val) (display " * ")
              (print-product (rest lst) new-product))))))

;;; the part that matters
(define (go input)
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
        #f)))

;;; Testing

(go '("yi" "nine" "six" "ba"))

(go '("yi" "josh" "three" "si"))



(define (main)
  (let ((input '("yi" "nine" "six" "ba")))
    (go input))

  (let ((input '("yi" "josh" "three" "si")))
    (go input))

  )