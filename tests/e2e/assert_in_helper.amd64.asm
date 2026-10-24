TEXT assert_in_helper.safe_half(SB)
  LEAQ  -80(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L3
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $64, SP
  MOVQ  $2, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  TESTQ CX, CX
  JEQ   L4
  CMPQ  CX, $-1
  JNE   L6
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L5
LABEL L6
  CQO
  IDIVQ CX
  MOVQ  DX, -16(BP)
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  -16(BP), AX
  MOVQ  -24(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  $2, AX
  MOVQ  AX, -40(BP)
  MOVQ  16(BP), AX
  MOVQ  -40(BP), CX
  TESTQ CX, CX
  JEQ   L7
  CMPQ  CX, $-1
  JNE   L9
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L8
LABEL L9
  CQO
  IDIVQ CX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L3
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L4
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.1.1(SB), AX
  JMP   rt.fault(SB)
LABEL L7
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L8
  LEAQ  site.3.1(SB), AX
  JMP   rt.fault(SB)

TEXT assert_in_helper.main(SB)
  LEAQ  -192(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $176, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  $10, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  assert_in_helper.safe_half(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  MOVQ  $24, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  $2, AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -48(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -48(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -80(BP)
  MOVQ  $7, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  assert_in_helper.safe_half(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  $24, AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  $2, AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -120(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -120(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  $0, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
