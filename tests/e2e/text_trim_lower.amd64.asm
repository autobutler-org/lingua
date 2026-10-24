TEXT text_trim_lower.main(SB)
  LEAQ  -576(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $560, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  -8(BP), AX
  MOVQ  AX, -16(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -24(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.trim(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -40(BP)
  MOVQ  $32, AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  $3, AX
  MOVQ  AX, -64(BP)
  MOVQ  -56(BP), AX
  MOVQ  -64(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -56(BP), AX
  MOVQ  -24(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -56(BP), AX
  MOVQ  -32(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -56(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 24(AX)
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
  LEAQ  str.1(SB), AX
  MOVQ  AX, -104(BP)
  MOVQ  -96(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.trim(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -120(BP)
  MOVQ  $32, AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $3, AX
  MOVQ  AX, -144(BP)
  MOVQ  -136(BP), AX
  MOVQ  -144(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -136(BP), AX
  MOVQ  -104(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -136(BP), AX
  MOVQ  -112(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -136(BP), AX
  MOVQ  -120(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -136(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -160(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, -176(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -184(BP)
  MOVQ  -176(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.trim(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -192(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -200(BP)
  MOVQ  $32, AX
  MOVQ  AX, -208(BP)
  MOVQ  -208(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -216(BP)
  MOVQ  $3, AX
  MOVQ  AX, -224(BP)
  MOVQ  -216(BP), AX
  MOVQ  -224(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -216(BP), AX
  MOVQ  -184(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -216(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -216(BP), AX
  MOVQ  -200(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -216(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -232(BP)
  MOVQ  -232(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -240(BP)
  LEAQ  str.5(SB), AX
  MOVQ  AX, -248(BP)
  MOVQ  -248(BP), AX
  MOVQ  AX, -256(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -264(BP)
  MOVQ  -256(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.trim(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -272(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -280(BP)
  MOVQ  $32, AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  $3, AX
  MOVQ  AX, -304(BP)
  MOVQ  -296(BP), AX
  MOVQ  -304(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -296(BP), AX
  MOVQ  -264(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -296(BP), AX
  MOVQ  -272(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -296(BP), AX
  MOVQ  -280(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -296(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -328(BP)
  MOVQ  -328(BP), AX
  MOVQ  AX, -336(BP)
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.lower(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -344(BP)
  MOVQ  -344(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -352(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -360(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, -368(BP)
  MOVQ  -368(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.lower(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -376(BP)
  MOVQ  -376(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -384(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -392(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -400(BP)
  MOVQ  -400(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.trim(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -408(BP)
  MOVQ  -408(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.lower(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -416(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -424(BP)
  MOVQ  $32, AX
  MOVQ  AX, -432(BP)
  MOVQ  -432(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -440(BP)
  MOVQ  $3, AX
  MOVQ  AX, -448(BP)
  MOVQ  -440(BP), AX
  MOVQ  -448(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -440(BP), AX
  MOVQ  -392(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -440(BP), AX
  MOVQ  -416(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -440(BP), AX
  MOVQ  -424(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -440(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -456(BP)
  MOVQ  -456(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -464(BP)
  LEAQ  str.1(SB), AX
  MOVQ  AX, -472(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -480(BP)
  MOVQ  -480(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.lower(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -488(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -496(BP)
  MOVQ  $32, AX
  MOVQ  AX, -504(BP)
  MOVQ  -504(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -512(BP)
  MOVQ  $3, AX
  MOVQ  AX, -520(BP)
  MOVQ  -512(BP), AX
  MOVQ  -520(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -512(BP), AX
  MOVQ  -472(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -512(BP), AX
  MOVQ  -488(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -512(BP), AX
  MOVQ  -496(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -512(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -528(BP)
  MOVQ  -528(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -536(BP)
  MOVQ  $0, AX
  MOVQ  AX, -544(BP)
  MOVQ  -544(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)
