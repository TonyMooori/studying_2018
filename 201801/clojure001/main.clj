; 楽な実行方法
; lein repl
; (load-file "main.clj")

; 構文参考メモ
; http://killingout-n-bita.hateblo.jp/entry/2015/03/21/122739

; 練習問題
; http://vipprog.net/wiki/exercise.html

(println "Fizz Buzz")

(defn fizz-buzz-convert [n]
    (if (= (mod n 15) 0 )
        "Fizz Buzz"
        (if (= (mod n 3) 0 )
            "Fizz"
            (if (= (mod n 5) 0)
                "Buzz"
                (str n)))))

(defn fizz-buzz [n] 
    (take n (map fizz-buzz-convert (rest (range)))))

(apply println (fizz-buzz 20))

;;;;;;;;;;;;;;;;;;;;;;;;

(println "素数列挙")

(defn is-prime [n]
    (not (some true?
            (map #(= 0 (mod n %1)) (range 2 n)))))

(defn primes [n]
    (filter is-prime (range 2 (+ n 1))))

(println (primes 100))

;;;;;;;;;;;;;;;;;;;;;;;;

(println "平方根")
; a_{n+1}=(a_n + v/a_n)/2

(defn abs [x]
    (if (< x 0) (* -1 x) x))

(defn search-sqrt [n s0]
    (if (< (abs (- n (* s0 s0))) 1e-8)
        s0
        (search-sqrt n (/ (+ s0 (/ n s0)) 2))))

(defn my-sqrt [n]
    (double (search-sqrt n n)))

(println (my-sqrt 2))

;;;;;;;;;;;;;;;;;;;;;;;;

(println "フィボナッチ数列")

(defn fib-1 [n]
    (if (<= n 1) 
        1
        (+ (fib (- n 1)) (fib (- n 2)))))

(defn fib-kernel [n0 n1 n_rest]
    (if (= 0 n_rest)
        '()
        (cons n0 (fib-kernel n1 (+ n0 n1) (- n_rest 1)))))

(defn fib-2 [n]
    (nth (fib-kernel 1 1 (+ n 2)) n))

(defn fib-kernel-2 [n0 n1]
    (lazy-seq (cons n0 (fib-kernel-2 n1 (+ n0 n1)))))

(defn fib-3 [n]
    (nth (fib-kernel-2 1 1) n))

(println (map fib-1 (range 10)))
(println (map fib-2 (range 10)))
(println (map fib-3 (range 10)))

;;;;;;;;;;;;;;;;;;;;;;;;

(println "pow関数の実装")

(defn pow-1 [x n]
    (if (= n 0)
        1
        (* x (pow-1 x (- n 1)))))

(defn pow-kernel [x n]
    (if (= n 0)
        '(1)
        (let [leaf (pow-kernel (* x x) (quot n 2))]
            (if (= (mod n 2) 0)
                leaf
                (cons x leaf)))))

(defn pow-2 [x n]
    (apply * (pow-kernel x n)))

(println (pow-1 3 3))
(println (pow-2 3 3))

;;;;;;;;;;;;;;;;;;;;;;;;

(println "回分判定")

(defn is-adam [s]
    (def len (count s))
    (if (<= len 1)
        true
        (if (= (nth s 0) (nth s (- len 1)))
            (is-adam (subs s 1 (- len 1)))
            false)))

(defn check-adam [s]
    (print (str s " is "))
    (if (is-adam s)
        (println "adam!!!")
        (println "not adam!!!")))

(check-adam "madamimadam")
(check-adam "neko")
(check-adam "a")
(check-adam "aa")
(check-adam "")
