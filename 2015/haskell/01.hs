import System.Environment
import Data.List

move :: Char -> Int
move '(' = 1
move ')' = -1
move _ = 0

part1 :: String -> Int
part1 =
    sum . map move

part2 :: String -> Maybe Int
part2 =
    elemIndex (-1) . scanl1 (+) . map move

main :: IO ()
main = do
    args <- getArgs
    input <- readFile (head args)
    mapM_ putStrLn
        [ "--- Part 1 ---"
        , "Floor: " ++ show (part1 input)
        , "--- Part 2 ---"
        , "Basement Entry: " ++ show (part2 input)
        ]
