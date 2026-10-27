TEXT splice.main(SB)
  LEAQ  -448(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L11
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $432, SP
  MOVQ  $3, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $0, AX
  MOVQ  AX, -24(BP)
  MOVQ  -16(BP), AX
  MOVQ  -24(BP), CX
  CMPQ  AX, CX
  SETGT AX
  MOVBQZX AX, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  $1, AX
  MOVQ  AX, -40(BP)
  MOVQ  -16(BP), AX
  MOVQ  -40(BP), CX
  SUBQ  CX, AX
  JOS   L12
  MOVQ  AX, -48(BP)
  MOVQ  $1, AX
  MOVQ  AX, -56(BP)
  MOVQ  -16(BP), AX
  MOVQ  -56(BP), CX
  SUBQ  CX, AX
  JOS   L13
  MOVQ  AX, -64(BP)
  MOVQ  -48(BP), AX
  MOVQ  -64(BP), CX
  ADDQ  CX, AX
  JOS   L14
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $16, AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  $1, AX
  MOVQ  AX, -112(BP)
  MOVQ  -104(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -104(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  $0, AX
  MOVQ  AX, -136(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, -144(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, -152(BP)
  JMP   L3
LABEL L2
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L3
  MOVQ  -152(BP), AX
  MOVQ  -144(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  TESTQ AX, AX
  JNE   L4
  JMP   L5
LABEL L4
  MOVQ  -152(BP), AX
  MOVQ  AX, -168(BP)
  MOVQ  -16(BP), AX
  MOVQ  -168(BP), CX
  SUBQ  CX, AX
  JOS   L15
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  $16, AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  $1, AX
  MOVQ  AX, -208(BP)
  MOVQ  -200(BP), AX
  MOVQ  -208(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -200(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 8(AX)
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
  MOVQ  $1, AX
  MOVQ  AX, -232(BP)
  MOVQ  -152(BP), AX
  MOVQ  -232(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -152(BP)
  JMP   L3
LABEL L5
  MOVQ  $0, AX
  MOVQ  AX, -240(BP)
  MOVQ  -80(BP), AX
  MOVQ  -16(BP), CX
  SUBQ  CX, AX
  JOS   L16
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, -256(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, -264(BP)
  JMP   L6
LABEL L6
  MOVQ  -264(BP), AX
  MOVQ  -256(BP), CX
  CMPQ  AX, CX
  SETLT AX
  MOVBQZX AX, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L8
LABEL L7
  MOVQ  -264(BP), AX
  MOVQ  AX, -280(BP)
  MOVQ  -80(BP), AX
  MOVQ  -16(BP), CX
  SUBQ  CX, AX
  JOS   L17
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  -280(BP), CX
  SUBQ  CX, AX
  JOS   L18
  MOVQ  AX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -304(BP)
  MOVQ  $16, AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  $1, AX
  MOVQ  AX, -328(BP)
  MOVQ  -320(BP), AX
  MOVQ  -328(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -320(BP), AX
  MOVQ  -304(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -344(BP)
  MOVQ  $1, AX
  MOVQ  AX, -352(BP)
  MOVQ  -264(BP), AX
  MOVQ  -352(BP), CX
  ADDQ  CX, AX
  MOVQ  AX, -264(BP)
  JMP   L6
LABEL L8
  MOVQ  $7, AX
  MOVQ  AX, -360(BP)
  MOVQ  -80(BP), AX
  MOVQ  -360(BP), CX
  SUBQ  CX, AX
  JOS   L19
  MOVQ  AX, -368(BP)
  MOVQ  $0, AX
  MOVQ  AX, -376(BP)
  MOVQ  -368(BP), AX
  MOVQ  -376(BP), CX
  CMPQ  AX, CX
  SETGT AX
  MOVBQZX AX, AX
  MOVQ  AX, -384(BP)
  MOVQ  -384(BP), AX
  TESTQ AX, AX
  JNE   L9
  JMP   L10
LABEL L9
  LEAQ  str.0(SB), AX
  MOVQ  AX, -392(BP)
  MOVQ  -392(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -400(BP)
  MOVQ  $0, AX
  MOVQ  AX, -408(BP)
  MOVQ  -408(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L10
  LEAQ  site.10(SB), AX
  JMP   rt.fault(SB)
LABEL L11
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L12
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L13
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)
LABEL L14
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L15
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
LABEL L16
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L17
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)
LABEL L18
  LEAQ  site.8(SB), AX
  JMP   rt.fault(SB)
LABEL L19
  LEAQ  site.9(SB), AX
  JMP   rt.fault(SB)
