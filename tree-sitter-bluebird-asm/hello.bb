  
  JMP over_here:
start:
  PUSH 4
  ST 0, [-4]
  PUSH 4
  ST 0, [-4]
loop:
  CMP [-8], 99
  BGT over:
  ADD 1, [-8], [-8]
  ADD [-8], [-4], [-4]
  JMP loop:
over:
  OUT [-4]
  POP 8

over_here:
  PUSH 4
  ST 0, [-4]
for_loop:
  CMP [-4], 400000000
  BGT done:
  ADD [-4], 2, [-4]
  JMP for_loop:
done:
  OUT [-4]
  POP 4
  JMP end:


modulo_example:
  PUSH 8
  ST 10, [-8]
  ST 3, [-4]
  MOD [-8], [-4], [-4]
  OUT [-4]
  POP 8



calculate_example:
  PUSH 4
  PUSH 4
  PUSH 4
loop1_start:
  CMP [-4], 10
  BGT loop1_end:

  PUSH 4
loop2_start:
  CMP [-4], 10
  BGT loop2_end:

  ADD [-4], [-8], [-12]
  ADD [-12], [-16], [-16]

  ADD [-4], 1, [-4]
  JMP loop2_start:

loop2_end:
  POP 4

  ADD 1, [-4], [-4]
  JMP loop1_start:
loop1_end:
  OUT [-12]
  POP 4
  POP 4
  POP 4
  JMP end:


end: