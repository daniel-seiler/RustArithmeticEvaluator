{-# LANGUAGE GADTs #-}

module Main where
main :: IO ()
main = return ()
--
-- Here's the AST for our example.


data E = One | Zero | ETrue | EFalse
       | Plus E E | Mult E E
       | Or E E deriving Eq


-- 1 * 0 + 1
ex = Plus (Mult One Zero) One

-- The AST for 1 * (0 + 1) equals
ex1 = Mult One (Plus Zero One)

-- Point to note. Parantheses are not required in the AST representation,
-- as they are captured by the AST structure.

-- Further examples.

-- 1 + true
ex2 = Plus One ETrue

-- false || true
ex3 = Or EFalse ETrue

-- false || 1
ex4 = Or EFalse One

-- true || 1
ex5 = Or ETrue One


-- Pretty printing

instance Show E where
    show One = "1"
    show Zero = "0"
    show ETrue = "true"
    show EFalse = "false"
    show (Plus e1 e2) = "(" ++ show e1 ++ " + " ++ show e2 ++ ")"
    show (Mult e1 e2) = "(" ++ show e1 ++ " * " ++ show e2 ++ ")"
    show (Or e1 e2) = "(" ++ show e1 ++ " || " ++ show e2 ++ ")"




-- Task 1: Let's write an evaluater for our litte language.

eval :: E -> Maybe (Either Int Bool)
eval One = Just (Left 1)
eval Zero = Just (Left 0)
eval ETrue = Just (Right True)
eval EFalse = Just (Right False)
eval (Plus e1 e2) =
     let r1 = eval e1
         r2 = eval e2
     in case (r1, r2) of
         (Just (Left i1), Just (Left i2)) -> Just (Left (i1 + i2))
         (_,_)  -> Nothing
eval (Mult e1 e2) =
     let r1 = eval e1
         r2 = eval e2
     in case (r1, r2) of
         (Just (Left i1), Just (Left i2)) -> Just (Left (i1 * i2))
         (_,_)  -> Nothing
eval (Or e1 e2) =
     case (eval e1) of
        Nothing -> Nothing
        (Just Left{}) -> Nothing
        (Just (Right True)) -> Just (Right True)
        _                   -> case (eval e2) of
                                 Nothing -> Nothing
                                 (Just Left{}) -> Nothing
                                 (Just (Right True)) -> Just (Right True)
                                 _                   -> Just (Right False)
       -- We apply short-circuit evaluation!
       -- The outer don't care pattern "_" covers the case "Just (Right False)".
       -- The same holds for the inner don't care pattern
       -- NOTE: We need to check for "Nothing" and "(Just Left{})" before.
       --  Nothing indicates evaluation got stuck.
       --  (Just Left{}) says the operand e1 evaluates to an Integer expression and
       --  therefore we're stuck.

-- Task 2: Let's write a type checker.

data Type = TInt | TBool deriving Show

typecheck :: E -> Maybe Type
typecheck Zero = Just TInt
typecheck One = Just TInt
typecheck ETrue = Just TBool
typecheck EFalse = Just TBool
typecheck (Plus e1 e2) =
     case (typecheck e1, typecheck e2) of
       (Just TInt, Just TInt) -> Just TInt
       (_, _) -> Nothing
typecheck (Mult e1 e2) =
     case (typecheck e1, typecheck e2) of
       (Just TInt, Just TInt) -> Just TInt
       (_, _) -> Nothing
typecheck (Or e1 e2) =
     case (typecheck e1, typecheck e2) of
       (Just TBool, Just TBool) -> Just TBool
       (_, _) -> Nothing

-- We can now write an evaluator for well-typed expressions.
-- (1) There's no need to check for ill-typed expressions
--    at run-time anymore!
-- (2) The below patterns do not cover all cases.
-- (3) This is not an issue, assuming we call evalT
-- with a well-typed expression only.

evalT :: E -> Either Int Bool
evalT One = Left 1
evalT Zero = Left 0
evalT ETrue = Right True
evalT EFalse = Right False
evalT (Plus e1 e2) =
    case (evalT e1, evalT e2) of
       (Left i1, Left i2) -> Left (i1 + i2)
evalT (Mult e1 e2) =
    case (evalT e1, evalT e2) of
       (Left i1, Left i2) -> Left (i1 * i2)
evalT (Or e1 e2) =
     case (evalT e1) of
       Right True -> Right True
       _          -> evalT e2



-- With generalized algebraic data types (GADTs),
-- we can encode the typing rules in the construction of values.
-- So by construction, we can only ever generate well-typed expressions!


data Exp a where
   One_Exp :: Exp Int
   Zero_Exp :: Exp Int
   ETrue_Exp :: Exp Bool
   EFalse_Exp :: Exp Bool
   Plus_Exp :: Exp Int -> Exp Int -> Exp Int
   Mult_Exp :: Exp Int -> Exp Int -> Exp Int
   Or_Exp :: Exp Bool -> Exp Bool -> Exp Bool


test1 :: Exp Int
test1 = Plus_Exp (Mult_Exp One_Exp Zero_Exp) One_Exp

-- will not type check
-- test2 = Or_Exp One_Exp ETrue_Exp

-- We can write an evaluator for the GADT.
-- Look at the type of the evaluator!
-- Unlike evalT, evalExp accepts only well-typed expressions.

evalExp :: Exp a -> a
evalExp One_Exp = 1
evalExp Zero_Exp = 1
evalExp ETrue_Exp = True
evalExp EFalse_Exp = False
evalExp (Plus_Exp e1 e2) = evalExp e1 + evalExp e2
evalExp (Mult_Exp e1 e2) = evalExp e1 * evalExp e2
evalExp (Or_Exp e1 e2) = evalExp e1 || evalExp e2

-- Task 3: Let's write a simplifyer for expressions where
-- we apply the law that 0 * x = 0
-- We use E.

simp :: E -> E
simp One = One
simp Zero = Zero
simp ETrue = ETrue
simp EFalse = EFalse
simp (Plus e1 e2) = Plus (simp e1) (simp e2)
simp (Mult Zero _) = Zero
simp (Mult e1 e2) = Mult (simp e1) (simp e2)
simp (Or e1 e2) = Or e1 e2

-- Consider the following two examples.
-- What do you observe before/after simplification?

-- (0 * 1) * 1
s1 = Mult (Mult Zero One) One

-- 0 * (0 * False)
s2 = Mult Zero (Or Zero EFalse)

-- Assumes we have defined equality among ASTs.
simpFix :: E -> E
simpFix e = let e2 = simp e
            in if e2 == e then e
               else simpFix e2
