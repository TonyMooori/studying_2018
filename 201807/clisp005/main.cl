(defun say-hello ()
  (princ "Please type your name:")
  (let ((name (read-line)))
    (princ "Nice to meet you,")
    (princ name)))

(defun add-five ()
  (princ "please enter a number;")
  (let ((n (read)))
    (princ "the answer is ")
    (princ (+ n 5))))

(defun game-repl()
  (let ((cmd (game-read)))
    (unless (eq (car cmd) 'quit)
      (game-print (game-eval cmd))
      (game-repl))))

(defun game-read()
  (let ((cmd (read-from-string
	       (concatenate 'string "(" (read-line) ")"))))
    (labels ((quote-it (x) (list 'quote x)))
      (cons (first cmd) (mapcar #'quote-it (rest cmd))))))

(defparameter *allowed-commdands* '(look walk pickup inventory))

(defun game-eval (sexp)
  (if (member (first sexp) *allowed-commands*)
    (eval sexp)
    '(i do not know that command.)))

; caps is t -> convert the first letter of first word is capital case.
; lit is t -> the symbol between #\" is as it is 
(defun tweak-text (lst caps lit)
  (when lst
    (let ((item (first lst))
	  (r (rest lst)))
      (cond
	((eql item #\space) (cons item (tweak-text r caps lit)))
	((member item '(#\! #\? #\.)) (cons item (tweak-text r t lit)))
	((eql item #\") (tweak-text r caps (not lit)))
	(lit (cons item (tweak-text r nil lit)))
	(caps (cons (char-upcase item) (tweak-text r nil lit)))
	(t (cons (char-downcase item) (tweak-text r nil nil)))))))

(defun game-print (lst)
  (princ (coerce (tweak-text (coerce (string-trim "() "
						   (prin1-to-string lst))
				      'list)
			      t
			      nil)
		  'string))
  (fresh-line))





