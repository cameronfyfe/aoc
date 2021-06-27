import System.Environment
import Data.List

data Point = Point
    { x :: Int
    , y :: Int
    } deriving (Eq)

evens :: [a] -> [a]
evens (x:xs) = x:odds xs
evens _ = []

odds :: [a] -> [a]
odds (_:xs) = evens xs
odds _ = []

move :: Point -> Char -> Point
move p '^' = Point (x p    ) (y p + 1)
move p 'v' = Point (x p    ) (y p - 1)
move p '>' = Point (x p + 1) (y p    )
move p '<' = Point (x p - 1) (y p    )
move p _ = p

housesOnPath :: [Char] -> [Point]
housesOnPath chs = scanl move (Point 0 0) chs

part1 :: String -> Int
part1 input =
    (length . nub) (housesOnPath input)

part2 :: String -> Int
part2 input =
    (length . nub) (santaHouses ++ robotHouses)
  where
    santaHouses = housesOnPath (evens input)
    robotHouses = housesOnPath (odds input)

main = do
    args <- getArgs
    input <- readFile (head args)
    mapM putStrLn
        [ "--- Part 1 ---"
        , "Num Houses: " ++ show (part1 input)
        , "--- Part 2 ---"
        , "Num Houses: " ++ show (part2 input)
        ]
