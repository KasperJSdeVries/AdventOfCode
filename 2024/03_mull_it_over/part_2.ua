Input ← &fras "input.txt"
# Input   ← "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
Strings ← regex "don't\\(\\)|do\\(\\)|mul\\([0-9]+,[0-9]+\\)" Input
Do      ← ▽¬\(◿2↥)+⊃(=□"don't()")(×2=□"do()") .♭Strings
Muls    ← ▽≠□"do()".Do
Values  ← ↯⊂:2⧻ .⋕≡(regex "[0-9]+") Muls
/+≡/×Values
