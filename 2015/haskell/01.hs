import System.Environment
import Data.List

move :: Char -> Int
move '(' = 1
move ')' = -1

part1 :: String -> Int
part1 input =
    sum (map move input)

part2 :: String -> Maybe Int
part2 input =
    elemIndex (-1) (
        scanl1 (+) (
            map move input
        )
    )

main = do
    args <- getArgs
    input <- readFile (head args)
    mapM putStrLn
        [ "--- Part 1 ---"
        , "Floor: " ++ show (part1 input)
        , "--- Part 2 ---"
        , "Basement Entry; " ++ show (part2 input)
        ]
