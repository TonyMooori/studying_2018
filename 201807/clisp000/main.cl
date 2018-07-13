; (load "main.cl")


(defun guess-my-number ()
    (ash (+ *small* *big*) -1))

(defun bigger ()
    (defparameter *small* (1+ (guess-my-number)))
    (guess-my-number))

(defun smaller ()
    (defparameter *big* (guess-my-number))
    (guess-my-number))

(defun start-over ()
    (defparameter *small* 1)
    (defparameter *big* 101)
    (guess-my-number))
