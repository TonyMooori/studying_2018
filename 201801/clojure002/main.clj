; 楽な実行方法
; lein repl
; (load-file "main.clj")

; 構文参考メモ
; http://killingout-n-bita.hateblo.jp/entry/2015/03/21/122739

(require '[clojure.string :as str])

; 問題といてたけど読み違えてた

(defn eval-f [x a-list b-list]
    (def a (first a-list))
    (def b (first b-list))
    (if (= a-list '())
        0
        (+ 
            (* (/ a b) (Math/pow x (+ 1 b)))
            (eval-f x (rest a-list) (rest b-list)))))

(defn solve-1 []
    (defn f [] (map read-string (str/split (read-line) #" ")))
    (def a-list (f))
    (def b-list (f))
    (def x-list (f))
    
    (println a-list)
    (println b-list)
    (println x-list)
    (def val-1 (eval-f (first  x-list) a-list b-list))
    (def val-2 (eval-f (second x-list) a-list b-list))
    (println val-1)
    (println val-2))


;(solve-1)

; パスカルの三角形

(defn disp [ls]
    (when (not= ls '())
        (print (first ls))
        (print " ")
        (disp (rest ls))))

(defn make-pascal [depth n ls]
    (def ls1 
        (map 
            #(+ (first %) (second %)) 
            (map vector (cons 0 ls) (concat ls '(0)))))
    (disp ls)
    (print "\n")
    (when (not= n depth)
        (make-pascal (+ depth 1) n ls1)))

(defn solve-2 []
    (def n (read-string (read-line)))
    (make-pascal 1 n '(1)))

(solve-2)