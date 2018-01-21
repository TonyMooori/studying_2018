
; 楽な実行方法
; lein repl
; (load-file main.clj)

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

(defn solve []
    (let [
        n (read-int)
        hs (read-ints)
        m (apply max hs)
    ]
    ;(println m)
    (println (count (filter #(= m %) hs)))))

;;;;;;;;;;;;;;;;;;;;;;;

(defn diff [ls]
    (if (< (count ls) 2)
        '()
        (cons (- (second ls) (first ls)) (diff (rest ls)))))

(defn solve2 []
    (let [
        n (read-int)
        a_t (read-ints)
        a (sort a_t)
        d (diff a)
    ]
    (println (apply min d))))

;;;;;;;;;;;;;;;;;;;;;;;

; 円周率だけど書きにくいというか適切な書き方がわからない

(def mem-pow
    (memoize (fn [m n]
        (if (= n 0) 1N (* m (mem-pow m (- n 1N)))))))

(defn fact [k one]
    (let[
        lower0 (mem-pow 16 k)
        lower1 (+ 1N (* 8N k))
        lower2 (+ 3N lower1)
        lower3 (+ 4N lower1)
        lower4 (+ 5N lower1)
        neg1   (* -1N one)
        f1     (quot (* 4N one)  lower1)
        f2     (quot (* 2N neg1) lower2)
        f3     (quot neg1 lower3)
        f4     (quot neg1 lower4)
        upper  (+ f1 f2 f3 f4)
    ]
    (quot upper lower0)))

(defn calc-pi [k one]
    (def my-pi 0)
    (loop [n 0]
        (let[
            f (fact n one)
        ]
        (when (>= f 1N)
            (def my-pi (+ my-pi f))
            (recur (inc n)))))
    my-pi)

(calc-pi 0 (mem-pow 10 100))


