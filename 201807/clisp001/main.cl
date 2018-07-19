(defvar N_GROVE_R 10) ; the number of red grove
(defvar N_GROVE_G  6) ; the number of green grove
(defvar N_GROVE_B  4) ; the number of blue grove

(defun range (n)
    (labels (
        (f (m) 
            (if (< m n) 
                (cons m (f (1+ m)))
                '())))
        (f 0)))

; exclude nil from xs
(defun exclude-nil (xs)
    (if xs
        (let (
            (y (first xs))
            (ys (rest xs)))
            (if y
                (cons y (exclude-nil ys))
                (exclude-nil ys)))
        '()))

; all combination that take n groves
(defun take-grove (n)
    (exclude-nil (take-grove-r n)))

; take red grove and take blue, and green groves
(defun take-grove-r (n)
    (labels (
        ; if we can take m red groves then take it and take green, blue groves
        (f (m)
            (if (< N_GROVE_R m)
                '()
                (take-grove-g m (- n m)))))
        (apply #'append (mapcar #'f (range (1+ n))))))

; take green groves and take blue groves
(defun take-grove-g (r n)
    (labels (
        ; if we can take m green groves then take it and take blue groves
        (f (m)
            (if (< N_GROVE_G m)
                '()
                (take-grove-b r m (- n m)))))
        (mapcar #'f (range (1+ n)))))

; if we can take n blue groves then take it and return the list (r g b).
(defun take-grove-b (r g n)
    (if (< N_GROVE_B n)
        '()
        (list r g n)))

; if all element of xs satisfied condition of a, then T
(defun check-a (xs)
    (if xs 
        (check-a-core xs)
        nil))

; if all element of xs satisfied condition of a, then T
(defun check-a-core (xs)
    (if xs
        (let* (
            (y (first xs))
            (ys (rest xs))
            (r (first y))
            (g (second y))
            (b (third y)))
            (if (or (>= r 2) (>= g 2) (>= b 2))
                (check-a-core ys)
                nil))
        T))

(defun solve-a (n)
    (if (check-a (take-grove n))
        n
        (solve-a (1+ n))))

; if all element of xs satisfied condition of a, then T
(defun check-b (xs)
    (if xs 
        (check-b-core xs)
        nil))

; if all element of xs satisfied condition of a, then T
(defun check-b-core (xs)
    (if xs
        (let* (
            (y (first xs))
            (ys (rest xs))
            (r (first y))
            (g (second y))
            (b (third y)))
            (if (and (>= r 2) (>= g 2) (>= b 2))
                (check-b-core ys)
                nil))
        T))

(defun solve-b (n)
    (if (check-b (take-grove n))
        n
        (solve-b (1+ n))))
