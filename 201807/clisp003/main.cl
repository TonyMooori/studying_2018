(defvar *arch-enemy* nil)
(defun puddingeater (person)
    (cond   ((eq person 'henry) 
                (setf *arch-enemy* 'stupid-lisp-alien)
                '(curse you lisp alien - you ate my pudding))
            ((eq person 'johnny)
                (setf *arch-enemy* 'useless-old-johnny)
                '(i hope you chocked on my pudding johnny))
            (t  '(why you eat my pudding stranger ?))))

(defun pudding-eater (person)
    (case person
        ((henry)    (setf *arch-enemy* 'stupid-lisp-alien)
                    '(curse you lisp alien - you ate my pudding))
        ((johnny)  (setf *arch-enemy* 'useless-old-johnny)
                    '(i hope you chocked on my pudding johnny))
        (otherwise '(why you eat my pudding stranger ?))))

(equal (member 3 '(1 2 3 4)) '(3 4))
(equal (find-if #'evenp '(1 2 3 4)) 2)


(defparameter *nodes*
    '(
        (living-room (you are in the living room. a wizard is snoring loudly on the couch.))
        (garden (you are in a beautiful garden. there is a well in front of you.))
        (attic (you are in the attic. there is a giant welding torch in the corner.))))
;; (assoc 'garden *nodes*) 
;; -> (GARDEN (YOU ARE IN A BEAUTIFUL GARDEN. THERE IS A WELL IN FRONT OF YOU.))

(defun describe-location (location nodes)
    (second (assoc location nodes)))

;; (describe-location 'garden *nodes*)
;; -> (YOU ARE IN A BEAUTIFUL GARDEN. THERE IS A WELL IN FRONT OF YOU.)

(defparameter *edges* '(
        (living-room
            (garden west door)
            (attic upstairs ladder))
        (garden
            (living-room east door))
        (attic
            (living-room downstairs ladder))))

(defun describe-path (edge)
    `(there is a ,(third edge) going ,(second edge) from here.))

(defun describe-paths (location edges)
    (apply #'append (mapcar #'describe-path (rest (assoc location edges)))))

;; (describe-paths 'living-room *edges*)
;; -> (THERE IS A DOOR GOING WEST FROM HERE. THERE IS A LADDER GOING UPSTAIRS FROM HERE.)

(defparameter *objects* '(whiskey bucket frog chain))
(defparameter *object-locations* '(
    (whiskey living-room)
    (bucket living-room)
    (chain garden)
    (frog garden)))

(defun objects-at (location objects object-locations)
    (labels (
        (f (obj) (eq location (second (assoc obj object-locations)))))
        (remove-if-not #'f objects)))

;; (objects-at 'living-room *objects* *object-locations*)
;; ->(WHISKEY BUCKET)

(defun describe-objects (location objects object-locations)
    (labels(
            (f (obj) `(you see a ,obj on the floor.)))
            (apply #'append (mapcar #'f (objects-at location objects object-locations)))))

(defparameter *location* 'living-room)

(defun look()
    (append 
        (describe-location *location* *nodes*)
        (describe-paths *location* *edges*)
        (describe-objects *location* *objects* *object-locations*)))

(defun walk (direction)
    (let ((next (find direction
                    (rest (assoc *location* *edges*))
                    :key #'second)))
        (if next
            (progn 
                (setf *location* (first next))
                (look))
            '(you cannot go that way))))

(defun pickup (object)
    (cond 
        ((member object (objects-at *location* *objects* *object-locations*))
         (push (list object 'body) *object-locations*)
         `(you are now carrying the ,object))
        (t
         '(you cannot get that.))))

(defun inventory ()
    (cons 'items- (objects-at 'body *objects* *object-locations*)))

;; memo http://rbtnn.hateblo.jp/entry/2015/10/18/204719