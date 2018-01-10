; 楽な実行方法
; lein repl
; (load-file "main.clj")

; 構文参考メモ
; http://killingout-n-bita.hateblo.jp/entry/2015/03/21/122739

(require '[clojure.string :as str])

; GCD

(defn read-ints []
    (map read-string (str/split (read-line) #" ")))

(defn gcd [n m]
    (def a (min n m))
    (def b (max n m))
    (if (= 0 (mod b a))
        a
        (gcd a (mod b a))))


(defn solve-1 []
    (def inputs (read-ints))
    (println (gcd (first inputs) (second inputs))))

; fib

(require '[clojure.string :as str])

(defn read-ints []
    (map read-string (str/split (read-line) #" ")))

(defn fib [n]
    (if (<= n 1)
        0
        (if (= n 2)
            1
            (+ (fib (- n 1)) (fib (- n 2))))))

(defn solve-2 []
    (def n (first (read-ints)))
    (println (fib n)))

;;;;;;;;;;;;;;;;;;;;;;

(defn solve-3 []
    (def s1 (read-line))
    (def s2 (read-line))
    (def ls (map vector s1 s2))
    (loop [n 0]
        (if (< n (count s1))
            (do
                (def c1 (nth s1 n))
                (def c2 (nth s2 n))
                (print (str c1 c2))
                (recur (inc n)))
            0 ))

    (print "\n"))
(solve-3)