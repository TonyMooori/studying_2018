Require Import List.

Inductive InList (A : Type)(a : A) : list A -> Prop :=
  | headIL : forall xs, InList A a (a::xs)
  | consIL : forall x xs, InList A a xs -> InList A a (x::xs).

Goal forall (A : Type)(a : A)(l : list A),
  (forall (x y : A), x = y \/ x <> y) -> InList A a l \/ ~ InList A a l.
intros.
induction l.
  right.
  unfold not.
  intros.
  inversion H0.
case IHl.
  intros.
  unfold not in H0.
  left.
  right.
  auto.
intros.
unfold not in H0.
assert (a=a0\/a<>a0).
  auto.
destruct H1.
  subst.
  left.
  left.
right.
unfold not.
intros.
inversion H2.
  subst.
  auto.
auto.
Qed.

Goal forall (A : Type)(a : A)(l : list A),
  (forall (x y : A), x = y \/ x <> y) -> InList A a l \/ ~ InList A a l.
intros.
induction l.
 right.
 intro.
 inversion H0.

 destruct IHl.
  left.
  constructor.
  apply H0.

  destruct (H a a0).
   left.
   rewrite H1.
   constructor.

   right.
   intro.
   inversion H2.
   apply (H1 H4).
   apply (H0 H4).
Qed.

Theorem fold_symmetric :
  forall (A:Type) (f:A -> A -> A),
    (forall x y z:A, f x (f y z) = f (f x y) z) ->
    (forall x y:A, f x y = f y x) ->
    forall (a0:A) (l:list A), fold_left f l a0 = fold_right f a0 l.
intros.
destruct l.
  simpl.
  auto.
  
  simpl.
  generalize a,a0.

  induction l.
    intros.
    simpl.
    apply (H0 a2 a1).
    
    simpl.
    intros.
    rewrite H.
    rewrite <-(H a3 a2 a1).
    rewrite (IHl (f a2 a1) a3).
    auto.
Qed.

Lemma S_n_isnt_n : forall (n:nat), S n <> n.
intros.
induction n.
auto.
auto.
Qed.

Lemma SS_n_isnt_n : forall (n:nat), S (S n) <> n.
intros.
induction n.     
auto.
auto.
Qed.

Goal forall (n m : nat), n = m \/ n <> m.
induction n.
intros.
induction m.
auto.
auto.
intros.
destruct (IHn (pred m)).
induction m.
right.
induction n.
auto.
auto.
rewrite (PeanoNat.Nat.pred_succ m) in H.
left.
rewrite H.
auto.
induction m.
auto.
right.
rewrite (PeanoNat.Nat.pred_succ m) in H.
apply (PeanoNat.Nat.succ_inj_wd_neg n m).
auto.
Qed.

Goal forall (P Q : nat -> Prop),(forall n, P n -> Q n) -> ((forall n, P n) -> (forall n, Q n)).
intros.
apply (H n).
apply (H0 n).
Qed.

Goal forall (P Q : nat -> Prop),((forall n, P n) -> (forall n, Q n)) -> (forall n, P n -> Q n).
intros.


Definition P1 (n : nat) : Prop := match n with
  | 0 => True
  | n => False
end.
Definition Q1 (n : nat) : Prop := True.

