
  PUSH 4
  JMP call_test:


function_0:
  PUSH 4
  ST 0, [-4]
  PUSH 4
  ST 22, [-4]
loop_start:
  CMP [-8], 16
  BGT loop_end:

  CALL multiply:
  ADD [-8], 1, [-8]

  JMP loop_start:
loop_end:
  POP 4
  POP 4
  RET

multiply:
  MUL [-8], 2, [-8]
  RET
  


call_test:
  CALL function_0:
  ST call_test:, [-4]
  JMP [-4]



































