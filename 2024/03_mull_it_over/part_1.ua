Input      ← &fras "input.txt"
Values     ← regex "mul\\(([0-9]+),([0-9]+)\\)" Input
ToMultiply ← ⋕°⍉↘1⍉ Values

/+≡/×ToMultiply
