(load "graph-util.cl")

(defparameter *city-nodes* nil)
(defparameter *city-edges* nil)
(defparameter *visited-nodes* nil)
(defparameter *node-num* 30)
(defparameter *edge-num* 45)
(defparameter *worm-num* 3)
(defparameter *cop-odds* 15)

(defun random-node()
  (1+ (random *node-num*)))

(defun edge-pair (a b)
  (unless (eql a b)
    ; ((a . b) (b . a))
    (list (cons a b) (cons b a))))

(defun make-edge-list ()
  (apply #'append
	 (loop repeat *edge-num*
	       collect (edge-pair (random-node) (random-node)))))

; nodeから行ける場所一覧を取得
(defun direct-edges(node edge-list)
  (remove-if-not (lambda (x) (eql (car x) node))
		 edge-list))

; 出発点から行けるノード一覧を取得
(defun get-connected (node edge-list)
  (let ((visited nil))
    ; nodeから行ける場所に行って行って無い場所があったらそこから行ける場所をマークしていく
    (labels ((traverse (node)
		       ; nodeがまだvisitedに含まれていない，すなわち訪れてない場合
		       (unless(member node visited)
			 ; nodeをvisitedに追加してそこからまた拡散する
			 (push node visited)
			 (mapc (lambda (edge)
				 (traverse (cdr edge)))
			       (direct-edges node edge-list)))))
      (traverse node))
    visited))

(defun find-islands (nodes edge-list)
  (let ((islands nil))
    ; このnodesはまだislandsに入ってないnodeの一覧
    (labels ((find-island (nodes)
			  (let* ((connected (get-connected (car nodes) edge-list))
				 (unconnected (set-difference nodes connnected)))
			    ; connected: それぞれ繋がってるノードのかたまり
			    ; unconnected: 繋がってないノード一覧
			    ; island: 島一覧(島は島の任意のノードから任意のノードへ行けるノードの集まり)
			    (push connected islands)
			    (when unconnected
			      (find-island unconnected)))))
      (find-island nodes))
    islands))

(defun connect-with-bridges (islands)
  (when (cdr islands)
    ; (car islands)は(from-loc . to-loc)という形
    ; (caar islands)でfrom-loc，(caadr islands)でto-locが出る
    (append (edge-pair (caar islands) (caadr islands))
	    (connect-with-bridges (cdr islands)))))

(defun connect-all-islands (nodes edge-list)
  ; edge-listに新しいedgeを追加して孤立がなくなるようにする
  (append (connect-with-bridges (find-islands nodes edge-list)) edge-list))

(defun make-city-edges ()
  (let* ((nodes (loop for i from 1 to *node-num* collect i))
	 (edge-list (connect-all-islands nodes (make-edge-list)))
	 (cops (remove-if-not (lambda (x) (zerop (random *cop-odds*)))
			      edge-list)))
    (add-cops (edges-to-alist edge-list) cops)))

(defun edges-to-alist (edge-list)
  (mapcar (lambda (node1)
	    (cons node1
		  (mapcar (lambda (edge)
			    (list (cdr edge)))
			  (remove-duplicates (direct-edges node1 edge-list)
					     :test #'equal))))
	  (remove-duplicates (mapcar #'car edge-list))))

(defun add-cops (edge-alist edge-with-cops)
  (mapcar (lambda (x)
	    (let ((node1 (car x))
		  (node1-edges (cdr x)))
	      (cons node1
		    (mapcar (lambda (edge)
			      (let ((node2 (car edge)))
				(if (intersection (edge-pair node1 node2) 
						  edges-with-cops
						  :test #'equal)
				  (list node2 'cops)
				  edge)))
			    node1-edges))))
	  edge-alist))

