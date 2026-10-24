TEXT fail_stmt.main(SB)
  LEAQ  -160(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L7
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $144, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -16(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  $1, AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  -56(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  JMP   L3
LABEL L3
  LEAQ  str.1(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  $1, AX
  MOVQ  AX, -88(BP)
  MOVQ  -80(BP), AX
  MOVQ  -88(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  TESTQ AX, AX
  JNE   L4
  JMP   L5
LABEL L4
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  JMP   L6
LABEL L6
  LEAQ  str.3(SB), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $0, AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L7
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
