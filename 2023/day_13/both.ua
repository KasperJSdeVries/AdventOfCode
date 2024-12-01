Split ← ⊜□¬⌕"\n\n".
Lines ← ⊜∘≠ @\n.
Parse ← ≡(⍜°□Lines) Split

GetMirror! ← (
  ⊃(↘1⇡⧻|¤°□) # cut locations, fixed pattern
  ≡(
    ⊃(⇌↙)↘ # Split at cut location
    ↧∩⧻ ,, # Get the length of the smallest cut
    ∩↙ ,:
    ^2
  )
)
Solve! ← (
  /+≡(/+⍜⊡(×100)1⊟∩(/+×+1⇡⧻.GetMirror!^2) ⍉.)
)

PartOne ← Solve!≍ Parse
PartTwo ← (
  Parse
  Solve!(
    /+♭≠ # Counts amount of differences between two arrays
    =1   # If its only one we have found it
  )
)

&fras "input.txt"

PartOne .
PartTwo :
