# smart-road
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
_______________|   |   |   |   |   |   |________________
                           |               
_______________            |            ________________
                           |               
_______________            |            ________________
                           |               
___________________________|____________________________
                           |
_______________            |            ________________
                           |
_______________            |            ________________
                           |
_______________            |            ________________
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |


## Dependance
```bash
    sudo apt-get install libsdl2-dev
    sudo apt-get install libsdl2-image-dev
    sudo apt-get install libsdl2-ttf-dev
```

je veux une fonction qui va permettre  de prevenir les collision entre les diferents vehicule dans le tableau avec ces informations.
Cas de collision possible:
- si la direction du vehicule est Direction::North(DirectionPath::TurnLeft) il peut entre en collision avec les vehicules qui ont ces directions
"
Direction::South(DirectionPath::TurnLeft),
Direction::South(DirectionPath::GoStraight),
Direction::West(DirectionPath::TurnLeft),
Direction::East(DirectionPath::GoStraight),
Direction::East(DirectionPath::TurnLeft),
"
- si la direction du vehicule est Direction::North(DirectionPath::GoStraight) il peut entre en collision avec les vehicules qui ont ces directions
"
Direction::South(DirectionPath::TurnLeft),
Direction::West(DirectionPath::GoStraight),
Direction::West(DirectionPath::TurnLeft),
Direction::East(DirectionPath::GoStraight),
"

- si la direction du vehicule est Direction::South(DirectionPath::TurnLeft) il peut entre en collision avec les vehicules qui ont ces directions
"
Direction::West(DirectionPath::TurnLeft),
Direction::West(DirectionPath::GoStraight),
Direction::North(DirectionPath::TurnLeft),
Direction::East(DirectionPath::GoStraight),
Direction::East(DirectionPath::TurnLeft),
"

- si la direction du vehicule est Direction::South(DirectionPath::GoStraight) il peut entre en collision avec les vehicules qui ont ces directions
"
Direction::East(DirectionPath::TurnLeft),
Direction::East(DirectionPath::GoStraight),
Direction::North(DirectionPath::TurnLeft),
Direction::West(DirectionPath::GoStraight),
"

- si la direction du vehicule est Direction::East(DirectionPath::TurnLeft) il peut entre en collision avec les vehicules qui ont ces directions
"
Direction::South(DirectionPath::TurnLeft),
Direction::South(DirectionPath::GoStraight),
Direction::West(DirectionPath::TurnLeft),
Direction::West(DirectionPath::GoStraight),
Direction::North(DirectionPath::TurnLeft),
"

- si la direction du vehicule est Direction::East(DirectionPath::GoStraight) il peut entre en collision avec les vehicules qui ont ces directions
"
Direction::West(DirectionPath::TurnLeft),
Direction::South(DirectionPath::GoStraight),
Direction::North(DirectionPath::TurnLeft),
Direction::North(DirectionPath::GoStraight),
"

- si la direction du vehicule est Direction::West(DirectionPath::TurnLeft) il peut entre en collision avec les vehicules qui ont ces directions
"
Direction::South(DirectionPath::TurnLeft),
Direction::South(DirectionPath::GoStraight),
Direction::East(DirectionPath::TurnLeft),
Direction::North(DirectionPath::GoStraight),
Direction::North(DirectionPath::TurnLeft),
"

- si la direction du vehicule est Direction::West(DirectionPath::GoStraight) il peut entre en collision avec les vehicules qui ont ces directions
"
Direction::South(DirectionPath::TurnLeft),
Direction::South(DirectionPath::GoStraight),
Direction::East(DirectionPath::TurnLeft),
Direction::North(DirectionPath::GoStraight),
"

L'algorithme doit fonctionner de cette maniere si le vehicule peut rentrer en collision avec un vehicule devant lui il doit ralentire et le vehicule devant doit augmenter sa vitesse pour eviter la collision.