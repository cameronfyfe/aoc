import System.Environment
import Data.List
import Data.List.Split

paperArea :: [Int] -> Int
paperArea sideLens = 2*l*(w + h) + 3*w*h
  where
    [h, w, l] = sort sideLens

ribbonLen :: [Int] -> Int
ribbonLen sideLens = 2*(w + h) + l*w*h
  where
    [h, w, l] = sort sideLens

sumForBoxes :: ([Int]-> Int) -> String -> Int
sumForBoxes boxFn input =
    sum $ map boxFn boxes
  where
    boxes = map
        (map read)
        (map (splitOn "x") (lines input))

part1 :: String -> Int
part2 :: String -> Int
part1 = sumForBoxes paperArea
part2 = sumForBoxes ribbonLen

main :: IO ()
main = do
    args <- getArgs
    input <- readFile (head args)
    mapM_ putStrLn
        [ "--- Part 1 ---"
        , "Sqft Paper: " ++ show (part1 input)
        , "--- Part 2 ---"
        , "Ft Ribbon: " ++ show (part2 input)
        ]
    