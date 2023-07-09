module Geometry (lineCircleIntersect) where

import Problem (Pos(..))

lineCircleIntersect :: Pos -> Pos -> Pos -> Double -> Bool
lineCircleIntersect e l c r =
  let d = minus l e
      f = minus e c
      a = dot d d
      b = 2 * dot f d
      c1 = dot f f - r*r
      disc = b*b - 4*a*c1
  in
      if disc <= 0 then
        False
      else
        let disc' = sqrt disc
            t1 = ((-b) - disc') / (2 * a)
            t2 = ((-b) + disc') / (2 * a)
        in
           (t1 >= 0 && t1 <= 1) || (t2 >= 0 && t2 <= 1)

minus :: Pos -> Pos -> Pos
minus (Pos a b) (Pos c d) = Pos (a - c) (b - d)

dot :: Pos -> Pos -> Double
dot (Pos a b) (Pos c d) = a*c + b*d 