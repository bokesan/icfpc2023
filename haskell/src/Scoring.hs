module Scoring (score) where

import Problem
import Geometry

score :: Problem -> [Pos] -> Double
score problem placements = sum [ happiness problem attendee placements | attendee <- attendees problem ]

happiness :: Problem -> Attendee -> [Pos] -> Double
happiness problem attendee placements = sum (map (impact problem attendee placements) (zip [0..] placements))

impact :: Problem -> Attendee -> [Pos] -> (Int, Pos) -> Double
impact problem attendee placements (k, p@(Pos px py))
  | isBlocked problem attendee placements p = 0
  | otherwise = let dx = x attendee - px
                    dy = y attendee - py
                    d = sqrt(dx*dx + dy*dy)
                    instrument = musicians problem !! k
                    taste = tastes attendee !! instrument
                 in
                    fromIntegral (ceiling (1000000 * taste / (d*d)))

isBlocked :: Problem -> Attendee -> [Pos] -> Pos -> Bool
isBlocked problem attendee placements p =
    any blockingMusician placements || any blockingPillar (pillars problem)
  where
    a = Pos (x attendee) (y attendee)
    blockingMusician p1 = p1 /= p && lineCircleIntersect a p p1 5
    blockingPillar pil = lineCircleIntersect a p (pCenter pil) (radius pil)
    pCenter pil = Pos (center pil !! 0) (center pil !! 1)
