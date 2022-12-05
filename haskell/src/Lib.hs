module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = do
    fileLines <- lines <$> readFile "input/puzzle.1.example"
    mapM_ (putStr . show) fileLines

-- split a function based on a predicate, throwing away False elements
splitWhen :: (a -> Bool) -> [a] -> [ [a] ]
splitWhen p l = foldr f []
    where
        f agg [] = agg
        f agg (y : ys) = 
