-- Press a button to generate a random number between 1 and 6.
--
-- Read how it works:
--   https://guide.elm-lang.org/effects/random.html
--

module Main exposing (..)

import Svg
import Svg.Attributes exposing (..)

import Browser
import Html exposing (..)
import Html.Events exposing (..)
import Random



-- MAIN


main =   Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }


dice number =
  case number of
    Just n -> if n == 1 then
        diceSvgFace1
      else if n == 2 then
        diceSvgFace2
      else if n == 3 then
        diceSvgFace3
      else if n == 4 then
        diceSvgFace4
      else if n == 5 then
        diceSvgFace5
      else
        diceSvgFace6
    Nothing -> diceSvgFace1

diceSvgFace1 =
  Svg.svg
    [ width "90"
    , height "90"
    , viewBox "0 0 100 100"
    ]
    [ 
      boxBorder
    , whiteBox
    , dot1
    ]
    
diceSvgFace2 =
  Svg.svg
    [ width "90"
    , height "90"
    , viewBox "0 0 100 100"
    ]
    [ 
      boxBorder
    , whiteBox
    , dot2_1
    , dot2_2
    ]
    
diceSvgFace3 =
  Svg.svg
    [ width "90"
    , height "90"
    , viewBox "0 0 100 100"
    ]
    [ 
      boxBorder
    , whiteBox
    , dot3_1
    , dot3_2
    , dot3_3
    ]
    
diceSvgFace4 =
  Svg.svg
    [ width "90"
    , height "90"
    , viewBox "0 0 100 100"
    ]
    [ 
      boxBorder
    , whiteBox
    , dot4_1
    , dot4_2
    , dot4_3
    , dot4_4
    ]
    
diceSvgFace5 =
  Svg.svg
    [ width "90"
    , height "90"
    , viewBox "0 0 100 100"
    ]
    [ 
      boxBorder
    , whiteBox
    , dot5_1
    , dot5_2
    , dot5_3
    , dot5_4
    , dot5_5
    ]
    
    
diceSvgFace6 =
  Svg.svg
    [ width "90"
    , height "90"
    , viewBox "0 0 100 100"
    ]
    [ 
      boxBorder
    , whiteBox
    , dot6_1
    , dot6_2
    , dot6_3
    , dot6_4
    , dot6_5
    , dot6_6
    ]
    
    
boxBorder = 
  Svg.rect
        [ x "8"
        , y "8"
        , width "80"
        , height "80"
        , rx "15"
        , ry "15"
        , fill "#000000"
        ]
        [] 

whiteBox =
  Svg.rect
        [ x "10"
        , y "10"
        , width "76"
        , height "76"
        , rx "15"
        , ry "15"
        , fill "#FFFFFF"
        ]
        []

dot1 = 
  Svg.circle
        [ cx "48"
        , cy "48"
        , r "5"
        ]
        []
        

dot2_1 = 
  Svg.circle
        [ cx "33"
        , cy "33"
        , r "5"
        ]
        []
  
dot2_2 = 
  Svg.circle
        [ cx "63"
        , cy "63"
        , r "5"
        ]
        []

        
dot3_1 = 
  Svg.circle
        [ cx "33"
        , cy "33"
        , r "5"
        ]
        []
        
dot3_2 = 
  Svg.circle
        [ cx "48"
        , cy "48"
        , r "5"
        ]
        []

dot3_3 = 
  Svg.circle
        [ cx "63"
        , cy "63"
        , r "5"
        ]
        []

dot4_1 = 
  Svg.circle
        [ cx "33"
        , cy "33"
        , r "5"
        ]
        []
        
dot4_2 = 
  Svg.circle
        [ cx "33"
        , cy "63"
        , r "5"
        ]
        []

dot4_3 = 
  Svg.circle
        [ cx "63"
        , cy "33"
        , r "5"
        ]
        []
        
dot4_4 = 
  Svg.circle
        [ cx "63"
        , cy "63"
        , r "5"
        ]
        []
        
        
dot5_1 = 
  Svg.circle
        [ cx "33"
        , cy "33"
        , r "5"
        ]
        []

dot5_2 = 
  Svg.circle
        [ cx "33"
        , cy "63"
        , r "5"
        ]
        []
        
dot5_3 = 
  Svg.circle
        [ cx "63"
        , cy "33"
        , r "5"
        ]
        []

dot5_4 = 
  Svg.circle
        [ cx "63"
        , cy "63"
        , r "5"
        ]
        []


dot5_5 = 
  Svg.circle
        [ cx "48"
        , cy "48"
        , r "5"
        ]
        []


dot6_1 = 
  Svg.circle
        [ cx "33"
        , cy "33"
        , r "5"
        ]
        []


dot6_2 = 
  Svg.circle
        [ cx "33"
        , cy "48"
        , r "5"
        ]
        []


dot6_3 = 
  Svg.circle
        [ cx "33"
        , cy "63"
        , r "5"
        ]
        []

dot6_4 = 
  Svg.circle
        [ cx "63"
        , cy "33"
        , r "5"
        ]
        []
        
dot6_5 = 
  Svg.circle
        [ cx "63"
        , cy "48"
        , r "5"
        ]
        []
        
dot6_6 = 
  Svg.circle
        [ cx "63"
        , cy "63"
        , r "5"
        ]
        []

-- MODEL


type alias Model =
  { dieFaces: List Int
  }


init : () -> (Model, Cmd Msg)
init _ =
  ( Model [1, 1]
  , Cmd.none
  )



-- UPDATE


type Msg
  = Roll
  | NewFaces (List Int)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    Roll ->
      ( model
      , Random.generate NewFaces (Random.list (List.length model.dieFaces) (Random.int 1 6))
      )

    NewFaces newFace ->
      ( Model newFace
      , Cmd.none
      )
      


-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none



-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ h1 [] [ text "Roll!" ]
    , dice (List.head model.dieFaces)
    , dice (List.head (List.drop 1 model.dieFaces))
    , div [] []
    , button [ onClick Roll ] [ text "Roll" ]
    ]

