Goal forall (n : nat), n = n + 0.
intros.
induction n.
reflexivity.
simpl.
rewrite <- IHn.
reflexivity.
Qed.

Require Import Arith.
Require Import List.
Require Import Arith.
Goal forall (n : nat), (exists m : nat, n = m * 4) -> exists k : nat, n = k * 2.
intros.
destruct H.
exists (x * 2).
rewrite mult_assoc_reverse.
simpl.
apply H.
Qed.

Inductive InList (A : Type)(a : A) : list A -> Prop :=
  | headIL : forall xs, InList A a (a::xs)   (* 1 *)
  | consIL : forall x xs, InList A a xs -> InList A a (x::xs). 

Theorem lt_Snm_nm : forall (n m : nat), S n < m -> n < m.
intros.
apply (lt_trans n (S n) m).
apply lt_n_Sn.
apply H.
Qed.

Lemma lemma1 : forall(a n : nat), n < S(a + n).
intros.
induction a.
simpl.
auto.
auto.
Qed.

Theorem pigeonhole : forall (xs : list nat),
  length xs < fold_right plus 0 xs -> exists x : nat, InList nat (S (S x)) xs.
intros.
induction xs.
simpl in H.
apply False_ind.
apply (lt_n_O O H).
destruct a.
apply (lt_Snm_nm (length xs) (fold_right Init.Nat.add 0 xs)) in H.
apply IHxs in H.
destruct H.
exists x.
apply (consIL nat (S (S x)) 0 xs).
apply H.
simpl in H.
destruct a.
simpl in H.
apply lt_S_n in H.
apply IHxs in H.
destruct H.
exists x.
right.
apply H.
exists a.
left.
Qed.

Require Import List.
Require Import Arith.

Theorem pigeonhole' : forall (xs : list nat),
  length xs < fold_right plus 0 xs -> exists x : nat, InList nat (S (S x)) xs.
intros.
induction xs.
simpl in H.
apply Nat.nlt_0_r in H.
contradiction.
destruct a.
remember (length (0::xs)).
simpl in Heqn.
rewrite Heqn in H.
apply lt_Snm_nm in H.
apply IHxs in H.
destruct H.
exists x.
right.
apply H.
destruct a.
simpl in H.
apply lt_S_n in H.
apply IHxs in H.
destruct H.
exists x.
right.
apply H.
exists a.
left.
Qed.

Goal forall (A : Type)(l : list A)(a a' : A), InList A a (a' :: l) -> a = a' \/ InList A a l.
intros.
inversion H.
left.
auto.
right.
auto.
Qed.

Lemma commutative_law : (forall (n m : nat) , n + m = m + n ).
intros.
induction n.
auto.
assert (m + S n = S m + n).
rewrite (plus_Snm_nSm m n).
reflexivity.
rewrite H.
rewrite (Nat.add_succ_l n m).
rewrite (Nat.add_succ_l m n).
apply eq_S.
apply IHn.
Qed.


Lemma add_one  : (forall (k:nat), S k = k + 1).
intros.
rewrite (commutative_law k 1).
unfold Nat.add.
reflexivity.
Qed.


Goal forall (n : nat), (exists m, n = 2 * m) \/ (exists m, n = 2 * m + 1).
intros.
induction n.
left.
exists 0.
auto.
induction n.
right.
exists 0.
auto.
destruct IHn.
right.
destruct H.
exists x.
rewrite <-(add_one (2*x)).
apply eq_S.
apply H.
left.
destruct H.
exists (x+1).
rewrite (Nat.mul_add_distr_l 2 x 1).
assert (2*1 = 1 + 1).
auto.
rewrite H0.
rewrite <-(plus_assoc_reverse (2*x) 1 1).
rewrite <-H.
rewrite (add_one (S n)).
reflexivity.
Qed.

Goal forall (A : Type)(l m:list A) (a:A), InList A a (l ++ m) -> InList A a l \/ InList A a m.
intros.
induction l.
  auto.
simpl in H.
inversion H.
  left.
  left.
subst.
apply IHl in H1.
destruct H1.
  left.
  right.
  apply H0.
right.
apply H0.
Qed.

















