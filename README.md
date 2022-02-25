# Dominion :crown:

Generate kingdoms (card configurations) for the deck builder [Dominion][game].

## Running?

No nifty binaries, use `cargo run -- --help` to see the available options.

## Why?

This is a port of Haskell project that I have for generating and tracking favorite kingdoms. I wanted to see what Rust is all about.

## Example

If you run, say
```shell
cargo run -- --ban-cards Witch \
             --include-cards YoungWitch \
             --include-expansions Base2 Renaissance \
             --project-count 2 \
             --hists \
             --pretty
```

You might get a kingdom like
```haskell
------------------ SETUP ------------------

._______________.
|               |
| Kingdom Cards |
|_______________|

Base2/Base1
 - Festival 
 - Gardens 
 - Militia 
 - Moneylender 
 - Remodel 
Renaissance
 - CargoShip  (Bane)
 - MountainVillage 
 - OldWitch 
 - Recruiter 
 - Spices 
Cornucopia
 - YoungWitch 

._______________.
|               |
| Project Cards |
|_______________|

Renaissance
 - CropRotation
 - Pageant


------------------ HISTS ------------------

Cards' Costs:
-------------
2:  (0)
3: ■ (1)
4: ■■■■■■ (6)
5: ■■■■ (4)
6:  (0)

Cards' Types:
-------------
Action  : ■■■■■■■■■ (9)
Attack  : ■■■ (3)
Reaction:  (0)
Victory : ■ (1)
Treasure: ■ (1)
Duration: ■ (1)

Expansions' Cards:
-----------------
Base1      : ■■■■■ (5)
Base2      : ■■■■■ (5)
Renaissance: ■■■■■ (5)
Cornucopia : ■ (1)
```

 [game]: https://www.riograndegames.com/games/dominion/
