
; (find-doc "nth")
; (doc nth)

(defn fact [n]
    (if (= n 0) 
        1
        (* n (fact (dec n)))))

(assert 
    (= 1 (*)))

(assert
    (=  [1 2 3 4]
        (concat [1 2] [3 4])))

(assert
    (=
        (quot 22 7)
        3))

(assert 
    (=
        (rem 22 7)
        1))

(assert 
    (=
        (mod 22 7)
        1))
                
(assert (= (conj [1] 2) [1 2]))
(assert (= (cons 1 [2]) [1 2]))

; 全部trueならtrue
;(every? even? [1 2 3])
; 1個でもtrueなら要素番号を返す
;(some even? [1 2 3])

(defn and-s [xs]
    (every? #(identity %1) xs))

(defn or-s [xs]
    (not (nil? (some #(identity %1) xs))))


(assert (= (sort > [1 2 3]) [3 2 1]))

(defn primes []
    (let[
        f (fn [xs]
            (let[
                x (first xs)
                g #(not= 0 (mod %1 x))
                r (filter g xs)
            ]
            (lazy-seq (cons x r))))]
        (f (iterate inc 2))))
