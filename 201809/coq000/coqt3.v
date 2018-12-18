Require Import List.

Theorem app_nil_r : forall (A : Type)(l : list A), l ++ nil = l.
intros.
induction l.
reflexivity.
simpl.
apply (f_equal (cons a)). (* cons a という関数をsubgoalから取り出すイメージ *)
apply IHl.
Qed.

Theorem app_nil_r' : forall (A : Type)(l : list A), l = l ++ nil.
intros.
induction l.
reflexivity.
simpl.
apply (f_equal (cons a)). (* cons a という関数をsubgoalから取り出すイメージ *)
apply IHl.
Qed.

Theorem app_assoc : forall (A : Type)(l1 l2 l3 : list A), l1 ++ (l2 ++ l3) = (l1 ++ l2) ++ l3.
intros.
induction l1.
induction l2.
induction l3.
simpl.
reflexivity.
simpl.
reflexivity.
simpl.
reflexivity.
simpl.
apply (f_equal (cons a)).
apply IHl1.
Qed.

Theorem app_assoc' : forall (A : Type)(l1 l2 l3 : list A), l1 ++ (l2 ++ l3) = (l1 ++ l2) ++ l3.
intros.
induction l1.
reflexivity.
simpl.
apply (f_equal (cons a)).
apply IHl1.
Qed.

(*
Theorem app_assoc'' : forall (A : Type)(l1 l2 l3 : list A), l1 ++ (l2 ++ l3) = (l1 ++ l2) ++ l3.
intros.
induction l3.
reflexivity.
*)
(* なんでinduction l1で詰まないのにinduction l3で詰むのかわからなさすぎる *)

Theorem rev_app_distr : forall (A : Type)(l1 l2 : list A), rev (l1 ++ l2) = rev l2 ++ rev l1.
intros.
induction l1.
simpl.
apply (app_nil_r' A (rev l2)).
simpl.
rewrite app_assoc.
f_equal.
apply IHl1.
Qed.

Theorem app_assoc'' : forall (A : Type)(l1 l2 l3 : list A), l1 ++ (l2 ++ l3) = (l1 ++ l2) ++ l3.
intros.
induction l1.
simpl.
reflexivity.
simpl.
apply (f_equal (cons a)).
apply IHl1.
Qed.

Theorem app_test : forall (A:Type) (l1 l2 : list A) (a : A) , l1 ++ a::l2 = (l1 ++ a::nil) ++l2.
intros.
induction l1.
simpl.
reflexivity.
simpl.
apply (f_equal (cons a0)).
apply IHl1.
Qed.

Theorem rev_involutive : forall (A : Type)(l : list A), rev (rev l) = l.
intros.
induction l.
simpl.
reflexivity.
simpl.
rewrite rev_app_distr.
rewrite IHl.
simpl.
reflexivity.
Qed.


Theorem fold_right_app : forall (A B : Type)(f : B -> A -> A)(l l' : list B)(i : A),
  fold_right f i (l ++ l') = fold_right f (fold_right f i l') l.
intros.
induction l.
simpl.
reflexivity.
simpl.
rewrite IHl.
reflexivity.
Qed.













