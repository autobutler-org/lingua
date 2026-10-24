TEXT int_holes.main(SB)
  LEAQ  -624(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $608, SP
  MOVQ  $0, AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $-42, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, -48(BP)
  MOVQ  $-9223372036854775807, AX
  MOVQ  AX, -56(BP)
  MOVQ  $1, AX
  MOVQ  AX, -64(BP)
  MOVQ  -56(BP), AX
  MOVQ  -64(BP), CX
  SUBQ  CX, AX
  JOS   L2
  MOVQ  AX, -72(BP)
  MOVQ  -72(BP), AX
  MOVQ  AX, -80(BP)
  LEAQ  str.0(SB), AX
  MOVQ  AX, -88(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
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
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -144(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  $24, AX
  MOVQ  AX, -160(BP)
  MOVQ  -160(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -168(BP)
  MOVQ  $2, AX
  MOVQ  AX, -176(BP)
  MOVQ  -168(BP), AX
  MOVQ  -176(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -168(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -168(BP), AX
  MOVQ  -152(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -184(BP)
  MOVQ  -184(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -200(BP)
  MOVQ  -48(BP), AX
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
  MOVQ  -200(BP), CX
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
  LEAQ  str.3(SB), AX
  MOVQ  AX, -256(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  $24, AX
  MOVQ  AX, -272(BP)
  MOVQ  -272(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -280(BP)
  MOVQ  $2, AX
  MOVQ  AX, -288(BP)
  MOVQ  -280(BP), AX
  MOVQ  -288(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -280(BP), AX
  MOVQ  -256(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -280(BP), AX
  MOVQ  -264(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -280(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -304(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -312(BP)
  MOVQ  $5, AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -328(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -336(BP)
  MOVQ  $-5, AX
  MOVQ  AX, -344(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -352(BP)
  MOVQ  $40, AX
  MOVQ  AX, -360(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -368(BP)
  MOVQ  $4, AX
  MOVQ  AX, -376(BP)
  MOVQ  -368(BP), AX
  MOVQ  -376(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -368(BP), AX
  MOVQ  -312(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -368(BP), AX
  MOVQ  -328(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -368(BP), AX
  MOVQ  -336(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -368(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -384(BP)
  MOVQ  -384(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -392(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -400(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -408(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -416(BP)
  MOVQ  $32, AX
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  $3, AX
  MOVQ  AX, -440(BP)
  MOVQ  -432(BP), AX
  MOVQ  -440(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -432(BP), AX
  MOVQ  -400(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -432(BP), AX
  MOVQ  -408(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -432(BP), AX
  MOVQ  -416(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -432(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -448(BP)
  MOVQ  -448(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -456(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -464(BP)
  MOVQ  -48(BP), AX
  MOVQ  -80(BP), CX
  ADDQ  CX, AX
  JOS   L3
  MOVQ  AX, -472(BP)
  MOVQ  -472(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -480(BP)
  MOVQ  $24, AX
  MOVQ  AX, -488(BP)
  MOVQ  -488(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -496(BP)
  MOVQ  $2, AX
  MOVQ  AX, -504(BP)
  MOVQ  -496(BP), AX
  MOVQ  -504(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -496(BP), AX
  MOVQ  -464(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -496(BP), AX
  MOVQ  -480(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -496(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -512(BP)
  MOVQ  -512(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -520(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -528(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -536(BP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -544(BP)
  MOVQ  $32, AX
  MOVQ  AX, -552(BP)
  MOVQ  -552(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -560(BP)
  MOVQ  $3, AX
  MOVQ  AX, -568(BP)
  MOVQ  -560(BP), AX
  MOVQ  -568(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -560(BP), AX
  MOVQ  -528(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -560(BP), AX
  MOVQ  -536(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -560(BP), AX
  MOVQ  -544(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -560(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -576(BP)
  MOVQ  -576(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -584(BP)
  MOVQ  $0, AX
  MOVQ  AX, -592(BP)
  MOVQ  -592(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
LABEL L2
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)
LABEL L3
  LEAQ  site.2(SB), AX
  JMP   rt.fault(SB)
