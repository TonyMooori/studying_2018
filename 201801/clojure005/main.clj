; 楽な実行方法
; lein repl
; (load-file main.clj)

; 構文参考メモ
; http://killingout-n-bita.hateblo.jp/entry/2015/03/21/122739

; template

(require '[clojure.string :as str])

(defn read-ints []
    (map read-string (str/split (read-line) #" ")))

(defn read-int []
    (first (read-ints)))

; land of lisp
; Chap 2

(def small 1)
(def big 100)

(defn guess-my-number []
    (bit-shift-right (+ small big) 1))

(defn smaller []
    (def big (dec (guess-my-number)))
    (guess-my-number))

(defn bigger []
    (def small (inc (guess-my-number)))
    (guess-my-number))

(defn start-over []
    (def small 1)
    (def big 100)
    (guess-my-number))

; Chap 3

(defn my-length [ls]
    (if (empty? ls)
        0
        (inc (my-length (rest ls))
        )))

; Chap 4

; (when val process+) -> valがtrueのときprocessを実行
; (when-not val process+) -> valがfalseのときprocessを実行
; (do process+) -> 一番最後の評価値
; cond -> if-else文みたいなの．:elseキーワードを使える
; https://clojuredocs.org/clojure.core/cond

(defn pos-neg-zero [n]
    (cond
        (< n 0) "negative"
        (> n 0) "positive"
        :else "zero"))

; case -> switch文

(defn one-two? [n]
    (case n
        1 "yes"
        2 "yes"
        "no"))

; (some f ls)はfを満たす要素があったらtrue
(defn eto? [s]
    (def etos
        '("ne","ushi","tora","u","tatsu","mi",
        "uma","hitsuji","saru","tori","inu","i"))
    (some #(= s %) etos))