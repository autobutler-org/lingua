TEXT text_equality.main(SB)
  LEAQ  -464(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L19
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $448, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
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
  MOVQ  -32(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -56(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $1, AX
  MOVQ  AX, -96(BP)
  MOVQ  -88(BP), AX
  MOVQ  -96(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  LEAQ  str.3(SB), AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  JMP   L3
LABEL L2
  JMP   L3
LABEL L3
  LEAQ  str.4(SB), AX
  MOVQ  AX, -128(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $1, AX
  MOVQ  AX, -144(BP)
  MOVQ  -136(BP), AX
  MOVQ  -144(BP), CX
  CMPQ  AX, CX
  SETNE AX
  MOVBQZX AX, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  TESTQ AX, AX
  JNE   L4
  JMP   L5
LABEL L4
  LEAQ  str.5(SB), AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  JMP   L6
LABEL L5
  JMP   L6
LABEL L6
  LEAQ  str.6(SB), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.lower(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -192(BP)
  MOVQ  $1, AX
  MOVQ  AX, -200(BP)
  MOVQ  -192(BP), AX
  MOVQ  -200(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L8
LABEL L7
  LEAQ  str.7(SB), AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  JMP   L9
LABEL L8
  JMP   L9
LABEL L9
  LEAQ  str.8(SB), AX
  MOVQ  AX, -232(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.trim(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -248(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -256(BP)
  MOVQ  $1, AX
  MOVQ  AX, -264(BP)
  MOVQ  -256(BP), AX
  MOVQ  -264(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  TESTQ AX, AX
  JNE   L10
  JMP   L11
LABEL L10
  LEAQ  str.10(SB), AX
  MOVQ  AX, -280(BP)
  MOVQ  -280(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -288(BP)
  JMP   L12
LABEL L11
  JMP   L12
LABEL L12
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  $1, AX
  MOVQ  AX, -304(BP)
  MOVQ  -296(BP), AX
  MOVQ  -304(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  TESTQ AX, AX
  JNE   L13
  JMP   L14
LABEL L13
  LEAQ  str.11(SB), AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -328(BP)
  JMP   L15
LABEL L14
  LEAQ  str.12(SB), AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -344(BP)
  JMP   L15
LABEL L15
  LEAQ  str.13(SB), AX
  MOVQ  AX, -352(BP)
  MOVQ  -352(BP), AX
  MOVQ  AX, -360(BP)
  LEAQ  str.14(SB), AX
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  AX, -376(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -376(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.text_eq(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -384(BP)
  MOVQ  $1, AX
  MOVQ  AX, -392(BP)
  MOVQ  -384(BP), AX
  MOVQ  -392(BP), CX
  CMPQ  AX, CX
  SETNE AX
  MOVBQZX AX, AX
  MOVQ  AX, -400(BP)
  MOVQ  -400(BP), AX
  TESTQ AX, AX
  JNE   L16
  JMP   L17
LABEL L16
  LEAQ  str.15(SB), AX
  MOVQ  AX, -408(BP)
  MOVQ  -408(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -416(BP)
  JMP   L18
LABEL L17
  JMP   L18
LABEL L18
  MOVQ  $0, AX
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L19
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
