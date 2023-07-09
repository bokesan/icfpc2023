{-# LANGUAGE OverloadedStrings, NamedFieldPuns #-}
module Problem (Problem(..),
                Attendee(..),
                Pos(..),
                Pillar(..),
                readProblem, readSolution) where

import qualified Data.ByteString.Lazy as B
import Data.Aeson
import Data.Text.Encoding

data Pos = Pos !Double !Double deriving (Eq, Ord, Show)

data Problem = Problem { roomWidth, roomHeight, stageWidth, stageHeight :: !Double
                       , stageBottomLeft :: [Double]
                       , musicians :: [Int]
                       , attendees :: [Attendee]
                       , pillars :: [Pillar]
                       } deriving (Show)

instance FromJSON Problem where
  parseJSON = withObject "Problem" $ \v -> Problem
      <$> v .: "room_width"
      <*> v .: "room_height"
      <*> v .: "stage_width"
      <*> v .: "stage_height"
      <*> v .: "stage_bottom_left"
      <*> v .: "musicians"
      <*> v .: "attendees"
      <*> v .: "pillars"

data Attendee = Attendee { x, y :: !Double, tastes :: [Double] } deriving (Show)

instance FromJSON Attendee where
  parseJSON = withObject "Attendee" $ \v -> Attendee
      <$> v .: "x"
      <*> v .: "y"
      <*> v .: "tastes"

data Pillar = Pillar { center :: [Double], radius :: !Double } deriving (Show)

instance FromJSON Pillar where
  parseJSON = withObject "Pillar" $ \v -> Pillar
      <$> v .: "center"
      <*> v .: "radius"

instance FromJSON Pos where
  parseJSON = withObject "Position" $ \v -> Pos <$> v .: "x" <*> v .: "y"

instance ToJSON Pos where
  toJSON (Pos x y) = object [ "x" .= x, "y" .= y ]
  toEncoding (Pos x y) = pairs ("x" .= x <> "y" .= y)

data Solution = Solution { placements :: [Pos] } deriving (Show)

instance FromJSON Solution where
  parseJSON = withObject "Solution" $ \v -> Solution <$> v .: "placements"

instance ToJSON Solution where
  toJSON s = object [ "placements" .= placements s ]
  toEncoding s = pairs ("placements" .= placements s)

{-
instance ToJSON Problem where
  toJSON (Problem{id_, data_}) =
    object [ "id" .= id_, "data" .= data_ ]
    
  toEncoding (Problem{id_, data_}) =
    pairs ("id" .= id_ <> "data" .= data_)
-}

readProblem :: FilePath -> IO Problem
readProblem f = do bs <- B.readFile f
                   case eitherDecode bs of
                     Right (String s) -> case eitherDecode (B.fromStrict (encodeUtf8 s)) of
                                           Right p -> return p
                                           Left err -> error ("Could not decode problem: " ++ err)
                     Right other -> error ("Cound not decode problem. Unexpected value: " ++ show other)
                     Left err -> error ("Could not decode problem: " ++ err) 

readSolution :: FilePath -> IO Solution
readSolution f = do bs <- B.readFile f
                    case eitherDecode bs of
                      Right s -> return s
                      Left err -> error ("Could not decode solution: " ++ err)