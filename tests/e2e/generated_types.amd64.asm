TEXT main.describe(SB)
  LEAQ  -208(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $192, SP
  MOVQ  16(BP), AX
  MOVQ  16(AX), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  LEAQ  str.0(SB), AX
  MOVQ  AX, -16(BP)
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  $48, AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  $5, AX
  MOVQ  AX, -80(BP)
  MOVQ  -72(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -72(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -72(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -72(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -72(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -72(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 40(AX)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L2
  JMP   L3
LABEL L3
  LEAQ  str.0(SB), AX
  MOVQ  AX, -96(BP)
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -120(BP)
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -128(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -136(BP)
  MOVQ  $48, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  $5, AX
  MOVQ  AX, -160(BP)
  MOVQ  -152(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -152(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -152(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -152(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -152(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -152(BP), AX
  MOVQ  -136(BP), CX
  MOVQ  CX, 40(AX)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT main.promo_text(SB)
  LEAQ  -256(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L8
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $240, SP
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
  LEAQ  str.4(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L4
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -136(BP)
  MOVQ  $2, AX
  MOVQ  AX, -144(BP)
  MOVQ  -136(BP), AX
  MOVQ  -144(BP), CX
  CMPQ  AX, CX
  SETEQ AX
  MOVBQZX AX, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  TESTQ AX, AX
  JNE   L7
  JMP   L6
LABEL L5
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -96(BP)
  MOVQ  $24, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $2, AX
  MOVQ  AX, -120(BP)
  MOVQ  -112(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -112(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L6
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L7
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  LEAQ  str.6(SB), AX
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
  MOVQ  -176(BP), CX
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
  MOVQ  AX, -8(BP)
  JMP   L1
LABEL L8
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)

TEXT main.offer_text(SB)
  LEAQ  -160(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $144, SP
  LEAQ  str.7(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -24(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -32(BP)
  MOVQ  16(BP), AX
  MOVQ  16(AX), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  16(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  LEAQ  str.10(SB), AX
  MOVQ  AX, -80(BP)
  MOVQ  16(BP), AX
  MOVQ  24(AX), AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  main.promo_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  $72, AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $8, AX
  MOVQ  AX, -120(BP)
  MOVQ  -112(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -112(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -112(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -112(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -112(BP), AX
  MOVQ  -48(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -112(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 40(AX)
  MOVQ  -112(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 48(AX)
  MOVQ  -112(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 56(AX)
  MOVQ  -112(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 64(AX)
  MOVQ  -112(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)

TEXT main.main(SB)
  LEAQ  -560(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $544, SP
  MOVQ  $7, AX
  MOVQ  AX, -8(BP)
  LEAQ  str.11(SB), AX
  MOVQ  AX, -16(BP)
  MOVQ  $1, AX
  MOVQ  AX, -24(BP)
  MOVQ  $24, AX
  MOVQ  AX, -32(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -40(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -40(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  main.describe(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  $8, AX
  MOVQ  AX, -64(BP)
  LEAQ  str.12(SB), AX
  MOVQ  AX, -72(BP)
  MOVQ  $0, AX
  MOVQ  AX, -80(BP)
  MOVQ  $24, AX
  MOVQ  AX, -88(BP)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -96(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -96(BP), AX
  MOVQ  -80(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  main.describe(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $3, AX
  MOVQ  AX, -120(BP)
  MOVQ  $4, AX
  MOVQ  AX, -128(BP)
  MOVQ  $16, AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -144(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -144(BP), AX
  MOVQ  AX, -152(BP)
  LEAQ  str.13(SB), AX
  MOVQ  AX, -160(BP)
  MOVQ  -152(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  LEAQ  str.14(SB), AX
  MOVQ  AX, -184(BP)
  MOVQ  -152(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -192(BP)
  MOVQ  -192(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -200(BP)
  LEAQ  str.15(SB), AX
  MOVQ  AX, -208(BP)
  MOVQ  -152(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -216(BP)
  MOVQ  -152(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -224(BP)
  MOVQ  -216(BP), AX
  MOVQ  -224(BP), CX
  ADDQ  CX, AX
  JOS   L2
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  $56, AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -256(BP)
  MOVQ  $6, AX
  MOVQ  AX, -264(BP)
  MOVQ  -256(BP), AX
  MOVQ  -264(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -256(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -256(BP), AX
  MOVQ  -176(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -256(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -256(BP), AX
  MOVQ  -200(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -256(BP), AX
  MOVQ  -208(BP), CX
  MOVQ  CX, 40(AX)
  MOVQ  -256(BP), AX
  MOVQ  -240(BP), CX
  MOVQ  CX, 48(AX)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -280(BP)
  MOVQ  $1, AX
  MOVQ  AX, -288(BP)
  MOVQ  $250, AX
  MOVQ  AX, -296(BP)
  MOVQ  $2, AX
  MOVQ  AX, -304(BP)
  LEAQ  var.shapes.Promo.NoPromo(SB), AX
  MOVQ  AX, -312(BP)
  MOVQ  $32, AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -328(BP)
  MOVQ  -328(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -328(BP), AX
  MOVQ  -296(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -328(BP), AX
  MOVQ  -304(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -328(BP), AX
  MOVQ  -312(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -328(BP), AX
  MOVQ  AX, 0(SP)
  CALL  main.offer_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -344(BP)
  MOVQ  $2, AX
  MOVQ  AX, -352(BP)
  MOVQ  $999, AX
  MOVQ  AX, -360(BP)
  MOVQ  $100, AX
  MOVQ  AX, -368(BP)
  MOVQ  $10, AX
  MOVQ  AX, -376(BP)
  MOVQ  $1, AX
  MOVQ  AX, -384(BP)
  MOVQ  $16, AX
  MOVQ  AX, -392(BP)
  MOVQ  -392(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -400(BP)
  MOVQ  -400(BP), AX
  MOVQ  -384(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -400(BP), AX
  MOVQ  -376(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  $32, AX
  MOVQ  AX, -408(BP)
  MOVQ  -408(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -416(BP)
  MOVQ  -416(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -416(BP), AX
  MOVQ  -360(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -416(BP), AX
  MOVQ  -368(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -416(BP), AX
  MOVQ  -400(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -416(BP), AX
  MOVQ  AX, 0(SP)
  CALL  main.offer_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  $3, AX
  MOVQ  AX, -440(BP)
  MOVQ  $0, AX
  MOVQ  AX, -448(BP)
  MOVQ  $1, AX
  MOVQ  AX, -456(BP)
  MOVQ  $75, AX
  MOVQ  AX, -464(BP)
  MOVQ  $2, AX
  MOVQ  AX, -472(BP)
  MOVQ  $16, AX
  MOVQ  AX, -480(BP)
  MOVQ  -480(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -488(BP)
  MOVQ  -488(BP), AX
  MOVQ  -472(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -488(BP), AX
  MOVQ  -464(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  $32, AX
  MOVQ  AX, -496(BP)
  MOVQ  -496(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -504(BP)
  MOVQ  -504(BP), AX
  MOVQ  -440(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -504(BP), AX
  MOVQ  -448(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -504(BP), AX
  MOVQ  -456(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -504(BP), AX
  MOVQ  -488(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -504(BP), AX
  MOVQ  AX, 0(SP)
  CALL  main.offer_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -512(BP)
  MOVQ  -512(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -520(BP)
  MOVQ  $0, AX
  MOVQ  AX, -528(BP)
  MOVQ  -528(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.5(SB), AX
  JMP   rt.fault(SB)
