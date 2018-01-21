
; 楽な実行方法
; lein repl
; (load-file "main.clj")

; 構文参考メモ
; http://killingout-n-bita.hateblo.jp/entry/2015/03/21/122739

; template
(require '[clojure.string :as str])

(
    do
    (defn read-ints []
        (map read-string (str/split (read-line) #" ")))

    (defn read-ints []
        (map #(Integer/parseInt %) (str/split (read-line) #" ")))

    (defn read-int []
        (first (read-ints)))

    ;;;;;;;;;;;;;;;;;;;;;;;

    (def nodes (hash-map
        :living-room "You are in the living-room. a wizard is snoring loudly on the couch."
        :garden      "You are in a beautiful garden. there is a beautiful garden."
        :attic       "You are in the attic. there is a giant welding torch in the corner"))
    
    (def edges (hash-map
        :living-room '(
            ; destination, direction, method
            (:garden :west :door) 
            (:attic :upstairs :ladder))
        :garden
            (:living-room :east :door)
        :attic
            (:living-room :downstairs :ladder)))
    
    ; nodesも引数に入れて副作用をなくすほうが適切
    (defn describe-location [loc nodes]
        (get nodes loc))
    
    (println (describe-location :living-room nodes))
    
    
    (defn describe-path [edge]
        (str "There is a " (first edge) " going, " (second edge) " from here."))
    
    (defn describe-paths [loc edges]
        (apply str (map describe-path (edges loc))))
    
    (println (describe-paths :living-room edges))

    (println '(I have a pen.))
    (println (concat '(I have a pen.) '(I have an apple.)))
    

    (def objects '(:whiskey :bucket :frog :chain))
    (def object-locations (hash-map
        :whiskey   :living-room
        :bucket    :living-room
        :chain     :garden
        :frog      :garden))

    (defn objects-at [loc objs obj-locs]
        ; locから見えるオブジェクトのリストを返す
        (def at-loc? (fn [obj] (= loc (get obj-locs obj)))) 
        (filter at-loc? objs))
    
    (println (objects-at :living-room objects object-locations))

    (defn describe-objects [loc objs obj-loc]
        (def near-objs (objects-at loc objs obj-loc))
        (defn f[obj] (str "You see a " (str obj) " on the floor."))
        (apply str (map f near-objs)))

    (println (describe-objects :living-room objects object-locations))

    ; プレイヤーが現在居る場所
    (def location :living-room)

    (defn look []
        (str 
            (describe-location location nodes)
            (describe-paths location edges)
            (describe-objects location objects object-locations)))
    
    (println (look))

    (defn walk [direction]
        (let[
            next (find direction (get edges location))
        ]))
)







