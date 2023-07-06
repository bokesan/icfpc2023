module Main (main) where

import System.Environment (getArgs)
import Text.Printf

import Problem

main :: IO ()
main = do args <- getArgs
          mapM_ processArg args

processArg :: String -> IO ()
processArg f = do problem <- readProblem f
                  printf "Problem loaded: %s\n" (show problem)
