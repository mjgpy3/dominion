# Dominion

:crown: <br />
:crab:

Generate kingdoms (card configurations) for the deck builder [Dominion][game].

## Site

The [site branch](https://github.com/mjgpy3/dominion/tree/site) is automatically deployed to https://mjgpy3.github.io/dominion/.

This site is using `wasm-pack` to generate WASM code from the same rust code that powers the binary.

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

┌───────────────┐
│ Kingdom Cards │
└───────────────┘

Cornucopia
 - Young Witch 
Base2/Base1
 - Chapel 
 - Festival 
 - Gardens 
 - Laboratory 
 - Moneylender (Bane)
 - Throne Room 
Renaissance
 - Old Witch 
 - Patron 
 - Villain 
Base2
 - Sentry 

┌───────────────┐
│ Project Cards │
└───────────────┘

Renaissance
 - CropRotation
 - Piazza


------------------ HISTS ------------------

Cards' Costs:
-------------
2: ■ (1)
3:  (0)
4: ■■■■■ (5)
5: ■■■■■ (5)
6:  (0)

Cards' types:
-------------
Action  : ■■■■■■■■■■ (10)
Attack  : ■■■ (3)
Reaction: ■ (1)
Victory : ■ (1)
Treasure:  (0)
Duration:  (0)

Expansions' cards:
-----------------
Base1      : ■■■■■■ (6)
Base2      : ■■■■■■■ (7)
Renaissance: ■■■ (3)
Cornucopia : ■ (1)
```

 [game]: https://www.riograndegames.com/games/dominion/
