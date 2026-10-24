TEXT primes.is_prime(SB)
  LEAQ  -144(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L10
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $128, SP
  MOVQ  $2, AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  -8(BP), CX
  CMPQ  AX, CX
  SETLT AX
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
  MOVQ  $2, AX
  MOVQ  AX, -32(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, -48(BP)
  JMP   L4
LABEL L4
  MOVQ  -48(BP), AX
  MOVQ  -40(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  TESTQ AX, AX
  JNE   L5
  JMP   L6
LABEL L5
  MOVQ  -48(BP), AX
  MOVQ  AX, -64(BP)
  MOVQ  16(BP), AX
  MOVQ  -64(BP), CX
  TESTQ CX, CX
  JEQ   L11
  CMPQ  CX, $-1
  JNE   L13
  MOVQ  $-9223372036854775808, DX
  CMPQ  AX, DX
  JEQ   L12
LABEL L13
  CQO
  IDIVQ CX
  MOVQ  DX, -72(BP)
  MOVQ  $0, AX
  MOVQ  AX, -80(BP)
  MOVQ  -72(BP), AX
  MOVQ  -80(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L8
LABEL L6
  MOVQ  $1, AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L7
  MOVQ  $0, AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L8
  JMP   L9
LABEL L9
  MOVQ  $1, AX
  MOVQ  AX, -104(BP)
  MOVQ  -48(BP), AX
  MOVQ  -104(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -48(BP)
  JMP   L4
LABEL L10
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L11
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L12
  LEAQ  site.1.1(SB), AX
  JMP   rt.fault(SB)

TEXT primes.main(SB)
  LEAQ  -240(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L7
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $224, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $2, AX
  MOVQ  AX, -24(BP)
  MOVQ  $50, AX
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
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  primes.is_prime(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  TESTQ AX, AX
  JNE   L4
  JMP   L5
LABEL L3
  LEAQ  str.0(SB), AX
  MOVQ  AX, -152(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  $24, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  $2, AX
  MOVQ  AX, -184(BP)
  MOVQ  -176(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -176(BP), AX
  MOVQ  -152(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -176(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  $0, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  $16, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $1, AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -96(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  $1, AX
  MOVQ  AX, -128(BP)
  MOVQ  -16(BP), AX
  MOVQ  -128(BP), CX
  ADDQ  CX, AX
  JOS   L8
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, -16(BP)
  JMP   L6
LABEL L5
  JMP   L6
LABEL L6
  MOVQ  $1, AX
  MOVQ  AX, -144(BP)
  MOVQ  -48(BP), AX
  MOVQ  -144(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -48(BP)
  JMP   L1
LABEL L7
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L8
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
