
; 楽な実行方法
; lein repl
; (load-file "main.clj")

; 構文参考メモ
; http://killingout-n-bita.hateblo.jp/entry/2015/03/21/122739

; template
(require '[clojure.string :as str])

(defn read-ints []
    (map read-string (str/split (read-line) #" ")))

(defn read-ints []
    (map #(Integer/parseInt %) (str/split (read-line) #" ")))

(defn read-int []
    (first (read-ints)))

;;;;;;;;;;;;;;;;;;;;;;;

(def prime 100000007)

(def solve
    (memoize (fn[n]
        (if (<= n 1)
            1
            (let[
                m  (dec n)
                xs (range n)
                ys (map #(mod (* (solve %) (solve (- m %))) prime) xs)
            ]
            (mod (apply + ys) prime))))))

(defn control[]
    (let[
        n (read-int)
    ]
    (loop [m n]
        (when (not= m -1)
            (println (solve (read-int)))
            (recur (dec m))
        ))))

;;;;;;;;;;;;;;;;;;;;;;;;;

(defn vec-sub [p1 p2]
    (when (and (not-empty p1) (not-empty p2))
        (cons 
            (- (first p1) (first p2)) 
            (vec-sub (rest p1) (rest p2)))))

(defn vec-div [p1 x]
    (map #(/ % x) p1))

(defn norm2 [xs]
    (if (empty? xs) 
        0
        (let[
            x (first xs)
            y (* x x)
            z (norm2 (rest xs))
        ]
            (+ y z))))

(defn norm [xs]
    (Math/sqrt (norm2 xs)))

(defn dot [e1 e2]
    (if (or (empty? e1) (empty? e2))
        0
        (let[
            v (* (first e1) (first e2))
            re1 (rest e1)
            re2 (rest e2)
            leaf (dot re1 re2)
        ]
        (+ leaf v))))

(defn calc-theta [v1 v2]
    (let[
        n1 (norm v1)
        n2 (norm v2)
        e1 (vec-div v1 n1)
        e2 (vec-div v2 n2)
    ]
    (Math/acos (dot e1 e2))))


(defn calc-p3 [p1 p2 p3]
    (let [
        v1 (vec-sub p2 p1)
        v2 (vec-sub p3 p1)
        theta (calc-theta v1 v2)
        a (norm v1)
        b (norm v2)
    ]
    (* 0.5 a b (Math/sin theta))))

(defn zip[xs ys]
    (if (or (empty? xs) (empty? ys))
        '()
        (let[
            x (first xs)
            y (first ys)
        ]
        (cons [x y] (zip (rest xs) (rest ys))))))

(defn calc-poly [ps]
    (let[
        p0 (first ps)
        xs (rest ps)
        zs (zip xs (rest xs))
    ]
    (apply + (map #(calc-p3 p0 (first %) (second %)) zs))))

(defn solve[]
    (let[
        n  (read-int)
        xs (for [i (range n)] (read-ints))
    ]
    (println (calc-poly xs))))

(solve)