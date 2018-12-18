
(*
Fixpoint ack_bad ( n m : nat) : nat := 
  match n with
    | 0 => S m
    | S n' =>
      match m with
        | 0 => ack_bad n' 1
        | S m' => ack_bad n' (ack_bad n m')
      end
  end.
*)

Fixpoint ack' (f:nat->nat) (m : nat) : nat :=
  match m with
    | 0 => f 1
    | S m' => f (ack' f m')
  end.

Fixpoint ack (n m : nat) : nat :=
  match n with
  | 0 => S m
  | S n' => ack' (ack n') m
  end.

Require Import List.
Require Import Arith.

Fixpoint merge (l1 l2 : list nat) :=
  match l1 with
    | nil => l2
    | x::xs =>
      (fix merge' l':=
        match l' with
          | nil => l1
          | y::ys => if leb x y then
              x::(merge xs l')
            else
              y::(merge' ys)
        end
      ) l2
  end.

