TEXT nominal_text.greet(SB)
  LEAQ  -80(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $64, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  $24, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -24(BP)
  MOVQ  $2, AX
  MOVQ  AX, -32(BP)
  MOVQ  -24(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -24(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -24(BP), AX
  MOVQ  16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT nominal_text.main(SB)
  LEAQ  -352(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $336, SP
  LEAQ  str.1(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  nominal_text.greet(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  $24, AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  $2, AX
  MOVQ  AX, -64(BP)
  MOVQ  -56(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -56(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -56(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.trim(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  nominal_text.greet(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -136(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  $1, AX
  MOVQ  AX, -152(BP)
  MOVQ  -144(BP), AX
  MOVQ  -152(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  LEAQ  str.5(SB), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  JMP   L3
LABEL L2
  JMP   L3
LABEL L3
  LEAQ  str.6(SB), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  nominal_text.greet(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -216(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  nominal_text.greet(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, -248(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -256(BP)
  MOVQ  $24, AX
  MOVQ  AX, -264(BP)
  MOVQ  -264(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -272(BP)
  MOVQ  $2, AX
  MOVQ  AX, -280(BP)
  MOVQ  -272(BP), AX
  MOVQ  -280(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -272(BP), AX
  MOVQ  -256(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -272(BP), AX
  MOVQ  -248(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  $0, AX
  MOVQ  AX, -304(BP)
  MOVQ  -304(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
