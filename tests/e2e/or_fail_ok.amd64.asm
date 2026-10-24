TEXT or_fail_ok.main(SB)
  LEAQ  -448(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L11
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $432, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -16(BP)
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -32(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.int_in_text(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -48(BP)
  MOVQ  $0, AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  -56(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  TESTQ AX, AX
  JNE   L3
  JMP   L2
LABEL L1
  MOVQ  -8(BP), AX
  MOVQ  AX, -120(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $24, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  $2, AX
  MOVQ  AX, -160(BP)
  MOVQ  -152(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -152(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -152(BP), AX
  MOVQ  -136(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -184(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -192(BP)
  MOVQ  $24, AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $2, AX
  MOVQ  AX, -216(BP)
  MOVQ  -208(BP), AX
  MOVQ  -216(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -208(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -208(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -208(BP), AX
  MOVQ  AX, -224(BP)
  MOVQ  $1, AX
  MOVQ  AX, -240(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.list_index(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -256(BP)
  MOVQ  $0, AX
  MOVQ  AX, -264(BP)
  MOVQ  -256(BP), AX
  MOVQ  -264(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  TESTQ AX, AX
  JNE   L8
  JMP   L7
LABEL L2
  MOVQ  -40(BP), AX
  MOVQ  0(AX), AX
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
  JNE   L5
  JMP   L4
LABEL L3
  MOVQ  -40(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  MOVQ  -40(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -112(BP)
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L6
  MOVQ  -232(BP), AX
  MOVQ  AX, -328(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -336(BP)
  MOVQ  $24, AX
  MOVQ  AX, -344(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -352(BP)
  MOVQ  $2, AX
  MOVQ  AX, -360(BP)
  MOVQ  -352(BP), AX
  MOVQ  -360(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -352(BP), AX
  MOVQ  -336(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -352(BP), AX
  MOVQ  -328(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -352(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -376(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -384(BP)
  MOVQ  -384(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -392(BP)
  MOVQ  $0, AX
  MOVQ  AX, -400(BP)
  MOVQ  -400(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L7
  MOVQ  -248(BP), AX
  MOVQ  0(AX), AX
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
  JNE   L10
  JMP   L9
LABEL L8
  MOVQ  -248(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -280(BP)
  MOVQ  -280(BP), AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  AX, -232(BP)
  JMP   L6
LABEL L9
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L10
  MOVQ  -248(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -320(BP)
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L11
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
