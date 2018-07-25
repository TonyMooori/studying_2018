; �z���X�g�������ꍇ�ɂ͂����t�ɂ���
(setf *print-circle* t)

; setf�ŕύX����ϐ��� '(1 2 3)�Ƃ����`�ł͂Ȃ� (list 1 2 3)�Ƃ����`�ō��
(defparameter foo (list 1 2 3))

(setf (cdddr foo) foo)

(defparameter *drink-order* '((bill . double-espresso)
			      (lisa . small-driop-coffee)
			      (john . medium-latte)))


; lisa�̒�����\��
(print (concatenate 'string 
		    "drink-order is " 
		    (prin1-to-string (assoc 'lisa *drink-order*))))

; ������ǉ�
(push '(lisa . soda) *drink-order*)

; lisa�̒�����\��
(print (concatenate 'string 
		    "drink-order is " 
		    (prin1-to-string (assoc 'lisa *drink-order*))))

(defparameter *house* '((walls (mortar (cement)
				       (waer)
				       (sand))
			       (bricks))
			(windows (glass)
				 (frame)
				 (curtains))
			(roof (shingles)
			      (chimney))))

(defun dot-name(exp)
  (map 'string
       (lambda (c) (if (alphanumericp c) c #\_))
       (prin1-to-string exp))) 

(defparameter *max-label-length* 30)

(defun dot-label (exp)
  (if exp
    ; write-to-string�̓V���{���𕶎���ɕς�����̂�
    ; pretty��nil�ɂ��邱�Ƃŗ]���ȕ��������Ȃ��悤�ɂ���
    (let ((s (write-to-string exp :pretty nil)))
      (if (> (length s) *max-label-length*)
	(concatenate 'string (subseq s 0 (- *max-label-length* 3)) "...")
	s))
    ""))

(defun nodes->dot (nodes)
  (mapc (lambda (node)
	  (fresh-line)
	  (princ (dot-name (first node)))
	  (princ "[label=\"")
	  (princ (dot-label node))
	  (princ "\"];"))
	nodes))

(defparameter *wizard-nodes* '((living-room (you are in the living room.))
			       (garden (you are in a beautiful garden.))
			       (attic (you are in a beautiful garden.))))

(defparameter *wizard-edges* '((living-room (garden west door) 
					    (attic upstairs ladder))
			       (garden (living-room east door))
			       (attic (living-room downstairs ladder))))


(defun edges->dot (edges)
  (mapc (lambda (node)
	  (mapc (lambda (edge)
		  (fresh-line)
		  (princ (dot-name (first node)))
		  (princ "->")
		  (princ (dot-name (first edge)))
		  (princ "[label=\"")
		  (princ (dot-label (rest edge)))
		  (princ "\"];"))
		(rest node)))
	edges))

(defun graph->dot (nodes edges)
  (princ "digraph{")
  (nodes->dot nodes)
  (edges->dot edges)
  (princ "}")
  nil)

(with-open-file (my-stream
		  "test.txt"
		  :direction  :output ; �����o���悤�ł��邱�Ƃ𖾋L
		  :if-exists  :supersede) ; ���Ԃ�����ȑO�̓��e���̂Ă�
  (princ "Hello file!" my-stream))

(defun dot->png (fname thunk)
  (with-open-file (*standard-output*
		    (concatenate 'string fname ".dot")
		    :direction :output
		    :if-exists :supersede)
    (funcall thunk))
  (ext:shell (concatenate 'string "dot -Tpng -O" fname)))


(defun graph-png (fname nodes edges)
  (dot->png fname
	    (lambda ()
	      (graph->dot nodes edges))))

(defun uedges->dot(edges-alist)
  (maplist (lambda (lst)
	     (let*(
		   (rest-edges-alist (rest lst))
		   (current-edges (first lst))
		   (from-location (first current-edges))
		   (roads (rest current-edges)))
	       (mapc (lambda (road)
		       (let*(
			     (to-location (first road))
			     (road-info (rest road)))
		       (unless (assoc to-location rest-edges-alist )
			 (fresh-line)
			 (princ (dot-name from-location))
			 (princ "--")
			 (princ (dot-name to-location))
			 (princ "[label=\"")
			 (princ (dot-label road-info))
			 (princ "\"];"))))
		     roads)))
	       edges-alist))

  (defun ugraph->dot (nodes edges)
    (princ "graph{")
    (nodes->dot nodes)
    (uedges->dot edges)
    (princ "}"))

  (defun ugraph->png (fname nodes edges)
    (dot->png fname
	      (lambda ()
		(ugraph->dot nodes edges))))





















