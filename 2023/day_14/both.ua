TestInput ← $ O....#....
            $ O.OO#....#
            $ .....##...
            $ OO.#O....O
            $ .O.....O#.
            $ O.#..O.#.#
            $ ..O..#O..O
            $ .......O..
            $ #....###..
            $ #OO..#....

Lines ← ⊜∘≠@\n.

Tilt ← (
  ∩(⊕□:°⊟⍉⊚=)@#⊙@O.:⧻.☇1
  ≡(
    :∩°□
    ⊙(..¤)
    ≡(
      (+1/↥|0)=0⧻.▽<
    )
    (⊟⊜(⊃⧻(-1⊢)).+1|[])=0⧻.
    ⍥(⊂@.):"":⊙:
    ∧(⍜⊡;⊙⊙@#)♭:⊙:
    (∧(∧(⍜⊡;⊙⊙@O)+⇡°⊟)⍉|∘;)=0⧻.:
  )
  ⍉
)

Cycle ← (
  ⍥(≡⇌⍉Tilt)4
)

FindLoop ← (
  ⍢(⊃(Cycle|⊂:□))(¬∊□) : []
  +◿∩(-:)⊃(⋅⧻|,1e9.⊗□|⋅∘)
)

Count ← (/+×⇌+1⇡⧻.≡/+=@O)

PartOne ← Count Tilt Lines
PartTwo ← ⊐Count⊏ FindLoop Lines

⍤⊃⋅∘≍ 136 PartOne TestInput
⍤⊃⋅∘≍ 64 PartTwo TestInput

PartTwo &fras "input.txt"
