; 楽な実行方法
; lein repl
; (load-file main.clj)

; 構文参考メモ
; http://killingout-n-bita.hateblo.jp/entry/2015/03/21/122739

; https://ja.wikipedia.org/wiki/ハーシャッド数
; ハーシャッド数（ハーシャッドすう、英: harshad number）とは、各位の和（数字和）が元の数の約数であるような自然数である。

(println "ハーシャッド数の列挙")

(defn get_digits [n]
    (if (= n 0)
        '()
        (cons (mod n 10) (get_digits (quot n 10)))))

(defn is_harshad [n]
    (= 0 (mod n (apply + (get_digits n)))))

(apply println 
    (take 100 (filter is_harshad (rest(range)))))