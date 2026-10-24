TEXT sum_no_payload.next_light(SB)
  LEAQ  -128(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L8
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $112, SP
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -16(BP)
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
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -8(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -48(BP)
  MOVQ  $2, AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  -56(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L4
LABEL L3
  LEAQ  var.sum_no_payload.Light.Green(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
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
  JNE   L7
  JMP   L6
LABEL L5
  LEAQ  var.sum_no_payload.Light.Yellow(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L7
  LEAQ  var.sum_no_payload.Light.Red(SB), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L8
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT sum_no_payload.light_name(SB)
  LEAQ  -128(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L8
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $112, SP
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -16(BP)
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
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -8(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
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
  JNE   L5
  JMP   L4
LABEL L3
  LEAQ  str.0(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -80(BP)
  MOVQ  $2, AX
  MOVQ  AX, -88(BP)
  MOVQ  -80(BP), AX
  MOVQ  -88(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L6
LABEL L5
  LEAQ  str.1(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L7
  LEAQ  str.2(SB), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L8
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)

TEXT sum_no_payload.main(SB)
  LEAQ  -144(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $128, SP
  LEAQ  var.sum_no_payload.Light.Red(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  $4, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -48(BP)
  JMP   L1
LABEL L1
  MOVQ  -48(BP), AX
  MOVQ  -40(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  TESTQ AX, AX
  JNE   L2
  JMP   L3
LABEL L2
  MOVQ  -48(BP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  sum_no_payload.light_name(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  sum_no_payload.next_light(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $1, AX
  MOVQ  AX, -96(BP)
  MOVQ  -48(BP), AX
  MOVQ  -96(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -48(BP)
  JMP   L1
LABEL L3
  MOVQ  $0, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
