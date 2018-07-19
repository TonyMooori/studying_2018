; アルゴリズムパズル 2

(defun solve-2-a (xs)
    (1+ (apply #'+ xs)))

(defun solve-2-b (xs)
    (let*(
        (ys (sort xs #'<))
        (min-val (first ys))
        (rest-ys (rest ys)))
        (+ (* 2 (apply #'+ rest-ys)) 1 min-val)))

(print (solve-2-a '(2 3 5)))
(print (solve-2-b '(2 3 5)))

