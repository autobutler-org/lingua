TEXT mutual_recursion.is_even(SB)
  LEAQ  -80(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $64, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  $1, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  MOVQ  $1, AX
  MOVQ  AX, -32(BP)
  MOVQ  16(BP), AX
  MOVQ  -32(BP), CX
  SUBQ  CX, AX
  JOS   L5
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  JMP   mutual_recursion.is_odd(SB)
LABEL L4
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)

TEXT mutual_recursion.is_odd(SB)
  LEAQ  -80(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $64, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  MOVQ  $1, AX
  MOVQ  AX, -32(BP)
  MOVQ  16(BP), AX
  MOVQ  -32(BP), CX
  SUBQ  CX, AX
  JOS   L5
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  JMP   mutual_recursion.is_even(SB)
LABEL L4
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)

TEXT mutual_recursion.down(SB)
  LEAQ  -64(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $48, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  CMPQ  AX, CX
  SETLE AX
  MOVBQZX AX, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  16(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  MOVQ  $2, AX
  MOVQ  AX, -24(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, -32(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  JMP   mutual_recursion.step(SB)
LABEL L4
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)

TEXT mutual_recursion.step(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  16(BP), AX
  MOVQ  24(BP), CX
  SUBQ  CX, AX
  JOS   L2
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  JMP   mutual_recursion.down(SB)
LABEL L1
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)

TEXT mutual_recursion.tally(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -24(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  JMP   mutual_recursion.add(SB)
LABEL L1
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)

TEXT mutual_recursion.add(SB)
  LEAQ  -112(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $96, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  24(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  MOVQ  $1, AX
  MOVQ  AX, -24(BP)
  MOVQ  24(BP), AX
  MOVQ  -24(BP), CX
  ADDQ  CX, AX
  JOS   L5
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, -40(BP)
  MOVQ  $1, AX
  MOVQ  AX, -48(BP)
  MOVQ  16(BP), AX
  MOVQ  -48(BP), CX
  SUBQ  CX, AX
  JOS   L6
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  JMP   mutual_recursion.again(SB)
LABEL L4
  LEAQ  site.8(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.9(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  LEAQ  site.10(SB), AX
  JMP   rt.fault(SB)

TEXT mutual_recursion.again(SB)
  LEAQ  -48(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $32, SP
  MOVQ  16(BP), AX
  MOVQ  AX, -8(BP)
  MOVQ  24(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  JMP   mutual_recursion.add(SB)
LABEL L1
  LEAQ  site.11(SB), AX
  JMP   rt.fault(SB)

TEXT mutual_recursion.main(SB)
  LEAQ  -272(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L7
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $256, SP
  MOVQ  $100000000, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 0(SP)
  CALL  mutual_recursion.is_even(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  LEAQ  str.0(SB), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  JMP   L3
LABEL L2
  JMP   L3
LABEL L3
  MOVQ  $100000001, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  mutual_recursion.is_odd(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  TESTQ AX, AX
  JNE   L4
  JMP   L5
LABEL L4
  LEAQ  str.1(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  JMP   L6
LABEL L5
  JMP   L6
LABEL L6
  LEAQ  str.2(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  $100000000, AX
  MOVQ  AX, -80(BP)
  MOVQ  $0, AX
  MOVQ  AX, -88(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 8(SP)
  CALL  mutual_recursion.down(SB)
  MOVQ  16(SP), AX
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
  MOVQ  -72(BP), CX
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
  LEAQ  str.3(SB), AX
  MOVQ  AX, -152(BP)
  MOVQ  $100000000, AX
  MOVQ  AX, -160(BP)
  MOVQ  $0, AX
  MOVQ  AX, -168(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 8(SP)
  CALL  mutual_recursion.tally(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  $24, AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  $2, AX
  MOVQ  AX, -208(BP)
  MOVQ  -200(BP), AX
  MOVQ  -208(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -200(BP), AX
  MOVQ  -152(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -200(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  $0, AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L7
  LEAQ  site.12(SB), AX
  JMP   rt.fault(SB)
