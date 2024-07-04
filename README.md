# smart-road
```
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
_______________| ← | ↓ | → |   |   |   |________________
                           |            ↑  
_______________            |            ________________
                           |            ←   
_______________            |            ________________
                           |            ↓   
___________________________|____________________________
               ↑           |
_______________            |            ________________
               →           |
_______________            |            ________________
               ↓           |
_______________            |            ________________
               |   |   |   | ← | ↑ | → |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
               |   |   |   |   |   |   |
```

# Smart Road Simulation

Ce projet est une simulation de trafic routier en utilisant la bibliothèque SDL2 pour le rendu graphique. Les véhicules peuvent être créés et se déplacer dans différentes directions, en respectant des trajectoires spécifiques. 

## Fonctionnalités

- Simulation graphique de véhicules se déplaçant sur une intersection
- Création de véhicules à l'aide des touches fléchées (haut, bas, gauche, droite)
- Gestion des collisions et ajustement dynamique de la vitesse des véhicules
- Affichage des statistiques des véhicules dans une fenêtre graphique

## Prérequis

- [Rust](https://www.rust-lang.org/) (version stable)
- [SDL2](https://www.libsdl.org/download-2.0.php)
- [SDL2_image](https://www.libsdl.org/projects/SDL_image/)

## Dependance
Installer SDL2, SDL2_image et SDL2_ttf. Sur Debian, vous pouvez utiliser :
```bash
    sudo apt-get install libsdl2-dev
    sudo apt-get install libsdl2-image-dev
    sudo apt-get install libsdl2-ttf-dev
```

## Installation

1. Cloner le dépôt :
    ```sh
    git clone https://github.com:Betzalel75/smart-road.git
    cd smart-road
    ```

2. Installer les dépendances Rust :
    ```sh
    cargo build
    ```

## Utilisation

1. Exécuter la simulation :
    ```sh
    cd src
    cargo run
    ```

2. Utiliser les touches fléchées pour créer des véhicules :
    - `Flèche Haut` : Créer un véhicule se déplaçant vers le nord
    - `Flèche Bas` : Créer un véhicule se déplaçant vers le sud
    - `Flèche Gauche` : Créer un véhicule se déplaçant vers l'ouest
    - `Flèche Droite` : Créer un véhicule se déplaçant vers l'est
    - `Touche R` : Créer un véhicule de manière alléatoire.

3. Appuyer sur `Échap` pour quitter la simulation et afficher les statistiques.

## Structure du Projet

- `src/main.rs` : Point d'entrée de l'application.
- `src/event.rs` : Gestion des événements et logique de simulation.
- `src/lib.rs` : Définition des structures et implémentation des méthodes pour les véhicules et les directions.
- `assets/` : Contient les images utilisées pour les véhicules et l'intersection.
- `src/utils/mod.rs` : Contient les fonctions utilitaires.

## Contribuer

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou soumettre une pull request pour toute amélioration ou suggestion.

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

