# Legions AI
An AI for the card game "Legions" on PlatoApp which is a 2-player card game. All rights to the Legions game and Plato app are reserved for (c) Plato Team Inc. I am  in no way affiliated with any of them.

## How to use:
This here is the source code which is written in Rust programming language. You have to install Cargo and Rust's compiler to be able to run it using the command "cargo run --release" on the root directory. Note that the --release switch is of utmost importance since this program does an enormous number of operations and therefor will run very slow on a debug build without the release optimizations.

First you enter Blue player's card types and Red player's card types repectively in order as such :
1 2 3 4
With a space inbetween each number that indicates card type.

Then each turn you enter the desired card to be played and the position as such :
[Deck_Index][Row][Column]
For example if I want to put the 1st card in my deck on top leftmost cell on the board the command will be : 111
Since you have 8 cards to choose from and the board is 4x5 there is no need to enter a space inbetween these values.

If you wish for the AI to play this turn, Simply hit enter on turn input providing it with an empty input.

Also, entering "b" as input in this stage will undo the last played move. Useful for when you entered a wrong input.

## Notes on the game itself
Using this AI, I ended up in the top 10 of ranked players in this game. However, saying this AI is unbeatable is not true at all and here's why. This is a card capturing game and the one who plays first, the Blue player, is at a significant disadvantage. Even though the developers of the game added 1 point to the score of the Blue player to remedy this, the Red player still has a substantial advantage so much so that the Blue player can only win if the Red player either makes a blunder or chooses worse cards.
Therefore, as of patch 5.0.0 of this game and version 1.2.1 of this project, it is not possible to win 100% of the time using this AI.
Also as a footnote, the lancer's pierce abilty seems to trigger the Same mechanic which seems to be a bug. I did not reflect this change of the game on this project yet. I may do so in the future.

## Technical Notes
This AI uses minimax with alpha-beta pruning to search for the best possible move in game's decision tree. However, I have implemented a minimum pruning depth to the original approach because of the nature of this game so that as many moves as possible are looked at. This way, at the very least, for each move the AI looks a couple of moves ahead to ensure that an initial bad move that leads to a better move later on does not get prematurely pruned.

Also, the AI uses multi-threading to make use of the entire capacity of the CPU divding each child of the root of the decision tree into a seperate thread which, in turn, recursively calls upon minimax function to find the best possible score for that move.
Do note that even though move ordering is used when fetching available moves, each child of the root is treated seperately and there is no passing the alpha or beta values between children of the root. This was implemented as such to avoid possible pruning of moves that appear to be a blunder at first but could lead to a better score farther down the tree.
Each thread calculates it's best score and sends it back to the main thread using a MPSC channel. Since there is no sharing of resources between threads, there is no need for a Mutex. Each resource (board, decks, etc) is copied and the clone is given to the spawned thread.

## For Rustaceans
I started this project to understand the intricacies of the Rust programming language. I'm ashamed to admit that I wanted to quit more than a couple of times in the course of finishing this project. The borrowing rule, if not understood clearly and fully, feels like heavy chains strapped tight to your feet, dangling as you write each line of code and take each step. However, as I finished the project and fully understood each rule of Rust, I feel my eyes have opened to many things that I took for granted over the years using different programming languages. The safety for memory management that Rust offers is enlightening, showing me just how unsafe most of the code written by those without a complete understaning of how memory works in low-level programming languages are. Now I see that each and every rule is how it should have been from the very beginning. It was arduous to break away from my old ways that had been deeply engraved but I am more than glad that I did.
