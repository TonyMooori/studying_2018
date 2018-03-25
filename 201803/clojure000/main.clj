
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

(defn zip[xs ys]
    (if (or (empty? xs) (empty? ys))
        '()
        (let[
            x (first xs)
            y (first ys)
        ]
        (cons [x y] (zip (rest xs) (rest ys))))))

(def prime 100000007)

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

(defn f [n]
    (do 
        (defn g [m] 
            (if (< m n)
                (cons m (g (inc m)))
                '())
        )
        (g 0)
    )
)









