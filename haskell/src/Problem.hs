{-# LANGUAGE OverloadedStrings, NamedFieldPuns #-}
module Problem (Problem(..), readProblem) where

import qualified Data.ByteString.Lazy as B
import Data.Text (Text)
import Data.Aeson

data Problem = Problem { id_   :: !Int
                       , data_ :: !Text
                       } deriving (Show)

instance FromJSON Problem where
  parseJSON = withObject "Problem" $ \v -> Problem
      <$> v .: "id"
      <*> v .: "data"

instance ToJSON Problem where
  toJSON (Problem{id_, data_}) =
    object [ "id" .= id_, "data" .= data_ ]
    
  toEncoding (Problem{id_, data_}) =
    pairs ("id" .= id_ <> "data" .= data_)

readProblem :: FilePath -> IO Problem
readProblem f = do bs <- B.readFile f
                   case eitherDecode bs of
                     Right problem -> return problem
                     Left err -> error ("Could not decode problem: " ++ err) 
