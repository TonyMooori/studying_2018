(defun solve-4 (n-soldier n)
    (if (= n-soldier 0)
        (print "it takes 0 steps.")
        (progn 
            (print "2 young man go to left.")
            (print "1 young man go to right")
            (print "1 soldier go to left.")
            (print "1 young man go to right.")
            (if (= 1 n-soldier)
                (print (format nil "~{~A~}" (list "it takes " (write-to-string (+ 4 n)) " steps.")))
                (solve-4 (1- n-soldier) (+ 4 n))))))

(solve-4 25 0)
