; 6. w”‚¦

(defparameter *fingers* 
  (list   
    "index finger" "thumb" "index finger" "middle finger"
    "ring finger" "little finger" "ring finger" "middle finger"))
			  

(defun solve-6 (n)
  (nth (mod n 8) *fingers*))
