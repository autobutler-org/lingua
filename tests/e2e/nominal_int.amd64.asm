TEXT nominal_int.subtotal(SB)
  LEAQ  -32(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $16, SP
  MOVQ  16(BP), AX
  MOVQ  24(BP), CX
  IMULQ CX, AX
  JOS   L2
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)

TEXT nominal_int.change(SB)
  LEAQ  -32(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $16, SP
  MOVQ  16(BP), AX
  MOVQ  24(BP), CX
  SUBQ  CX, AX
  JOS   L2
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, 32(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.3(SB), AX
  JMP   rt.fault(SB)

TEXT nominal_int.main(SB)
  LEAQ  -528(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $512, SP
  MOVQ  $250, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $3, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 8(SP)
  CALL  nominal_int.subtotal(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -56(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  $24, AX
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -80(BP)
  MOVQ  $2, AX
  MOVQ  AX, -88(BP)
  MOVQ  -80(BP), AX
  MOVQ  -88(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -80(BP), AX
  MOVQ  -56(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -80(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -96(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  $1000, AX
  MOVQ  AX, -112(BP)
  MOVQ  -112(BP), AX
  MOVQ  AX, -120(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -128(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 8(SP)
  CALL  nominal_int.change(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -144(BP)
  MOVQ  $24, AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  MOVQ  $2, AX
  MOVQ  AX, -168(BP)
  MOVQ  -160(BP), AX
  MOVQ  -168(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -160(BP), AX
  MOVQ  -128(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -160(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -192(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 8(SP)
  CALL  nominal_int.change(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  MOVQ  $24, AX
  MOVQ  AX, -216(BP)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -224(BP)
  MOVQ  $2, AX
  MOVQ  AX, -232(BP)
  MOVQ  -224(BP), AX
  MOVQ  -232(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -224(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -224(BP), AX
  MOVQ  -208(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  $0, AX
  MOVQ  AX, -256(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, -264(BP)
  MOVQ  -264(BP), AX
  MOVQ  -48(BP), CX
  ADDQ  CX, AX
  JOS   L2
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, -264(BP)
  MOVQ  -264(BP), AX
  MOVQ  -16(BP), CX
  ADDQ  CX, AX
  JOS   L3
  MOVQ  AX, -280(BP)
  MOVQ  -280(BP), AX
  MOVQ  AX, -264(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -288(BP)
  MOVQ  -264(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  $24, AX
  MOVQ  AX, -304(BP)
  MOVQ  -304(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -312(BP)
  MOVQ  $2, AX
  MOVQ  AX, -320(BP)
  MOVQ  -312(BP), AX
  MOVQ  -320(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -312(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -312(BP), AX
  MOVQ  -296(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -328(BP)
  MOVQ  -328(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -336(BP)
  MOVQ  -264(BP), AX
  MOVQ  AX, -344(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -352(BP)
  MOVQ  $-2000, AX
  MOVQ  AX, -360(BP)
  MOVQ  -344(BP), AX
  MOVQ  -360(BP), CX
  ADDQ  CX, AX
  JOS   L4
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -376(BP)
  MOVQ  $24, AX
  MOVQ  AX, -384(BP)
  MOVQ  -384(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -392(BP)
  MOVQ  $2, AX
  MOVQ  AX, -400(BP)
  MOVQ  -392(BP), AX
  MOVQ  -400(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -392(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -392(BP), AX
  MOVQ  -376(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -392(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -408(BP)
  MOVQ  -408(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -416(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -424(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  $24, AX
  MOVQ  AX, -440(BP)
  MOVQ  -440(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -448(BP)
  MOVQ  $2, AX
  MOVQ  AX, -456(BP)
  MOVQ  -448(BP), AX
  MOVQ  -456(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -448(BP), AX
  MOVQ  -424(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -448(BP), AX
  MOVQ  -432(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -448(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -464(BP)
  MOVQ  -464(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -472(BP)
  MOVQ  $0, AX
  MOVQ  AX, -480(BP)
  MOVQ  -480(BP), AX
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
LABEL L3
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
LABEL L4
  LEAQ  site.7(SB), AX
  JMP   rt.fault(SB)
