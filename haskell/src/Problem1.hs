module Problem1
    ( exampleFunc
    ) where

import Data.Maybe (isNothing)
import Text.Read

import Lib (splitWhen)


readPacks :: [String] -> [ [Integer] ]
readPacks lines = map (map catMaybes) packs
    where
        maybeLines = map readMaybe lines
        packs = splitWhen isNothing maybeLines

exampleFunc :: IO ()
exampleFunc = do
    fileLines <- lines <$> readFile "input/puzzle.1.example"
    let packs = readPacks fileLines

    mapM_ print packs
