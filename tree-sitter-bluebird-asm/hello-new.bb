
start:
  ADD U8 1, -1, PC  ;add the two numbers togerher.
  CALL fun_1
  JMP start ;make an infinite loop.


fun_1:
  PUSH 2
  ST U8 10, [-1]
  ST U8 10, [-2]
  ADD U8 [-1], [-2], [-2] ;stores the sum into [-2] relative memory.
  POP 2
  PUSH 4
  ST F32 -3.33333333, [-4]
  ADD F32 [-4], 4.8888888, PC
  ADD F32 -3.3, 8, [-4]
  SHL U8 1, 7, [-4]
  RET

