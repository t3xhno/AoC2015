{-# LANGUAGE TemplateHaskell #-}

import Data.Hash.MD5
import Data.String.Utils

main = λ head $ dropWhile (not . startswith  "00000" . md5s . Str) $ map (("yzbqklnj"++) . show) [0..]""