(defun solve-4 (n-soldier n)
    (if (= n-soldier 0)
        (print (format nil "~{~A~}" (list "it takes " (write-to-string n) " steps.")))
        (progn 
            (print "2 young man go to left.")
            (print "1 young man go to right")
            (print "1 soldier go to left.")
            (print "1 young man go to right.")
                (solve-4 (1- n-soldier) (+ 4 n)))))

(solve-4 25 0)
