; 楽な実行方法
; lein repl
; (load-file "main.clj")

; 構文参考メモ
; http://killingout-n-bita.hateblo.jp/entry/2015/03/21/122739

(require '[clojure.string :as str])

(defn read-ints []
    (map read-string (str/split (read-line) #" ")))

(defn read-int []
    (first (read-ints)))

(defn make-pairs [s]
    (if (= 0 (count s))
        ""
        (str 
            (second s) 
            (first s) 
            (make-pairs (nthrest s 2)))))

(defn solve-kernel [n]
    (when (not= n 0)
        (def s (read-line))
        (def ps (make-pairs s))
        (println (reduce str ps))
        (solve-kernel (dec n))))

(defn solve-kernel-2 [n]
    (when (not= n 0)
        (def s (read-line))
        (println
            (reduce str
                (for [i (range (quot (count s) 2))]
                    (str (nth s (inc (* 2 i))) (nth s (* 2 i)) ))))
        (solve-kernel-2 (dec n))))


(defn make-idx [n]
    (lazy-seq (concat (list (inc n) n) (make-idx (+ n 2)))))

(defn solve-kernel-3 [n]
    (when (not= n 0)
        (def s (read-line))
        (def idx (take (count s) (make-idx 0)))
        (println (apply str (map #(nth s %) idx)))
        (solve-kernel-3 (dec n))))

(defn solve []
    (def t (read-int))
    (solve-kernel-3 t))

(solve)


