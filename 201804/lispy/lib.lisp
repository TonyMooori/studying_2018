(do
    (def inc (fn (n) (+ n 1)))
    (def dec (fn (n) (- n 1)))

    (def range (fn (n) 
        (do 
            (def f (fn (m) 
                (if (< m n)
                    (cons m (f (inc m)))
                    (list))))
            (f 0))))

    (def nth (fn (xs n) (
        (if (= n 0)
            (first xs)
            (nth (rest xs) (dec n))))))
    
    (def mod (fn (n m) (do
        (def d (int (/ n m)))
        (- n (* d m)))))        
)