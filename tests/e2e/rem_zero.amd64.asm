TEXT rem_zero.main(SB)
  LEAQ  -208(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $192, SP
  MOVQ  $10, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -32(BP), AX
  MOVQ  -16(BP), CX
  TESTQ CX, CX
  JEQ   L2
  CMPQ  CX, $-1
  JNE   L4
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L3
LABEL L4
  CQO
  IDIVQ CX
  MOVQ  DX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  $24, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  $2, AX
  MOVQ  AX, -80(BP)
  MOVQ  -72(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -72(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -72(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  -16(BP), AX
  MOVQ  -32(BP), CX
  TESTQ CX, CX
  JEQ   L5
  CMPQ  CX, $-1
  JNE   L7
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L6
LABEL L7
  CQO
  IDIVQ CX
  MOVQ  DX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, -112(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -120(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  $24, AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  $2, AX
  MOVQ  AX, -152(BP)
  MOVQ  -144(BP), AX
  MOVQ  -152(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -144(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -144(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  MOVQ  $0, AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L3
  LEAQ  site.1.1(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  LEAQ  site.2.1(SB), AX
  JMP   rt.fault(SB)
