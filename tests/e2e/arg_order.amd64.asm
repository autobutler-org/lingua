TEXT arg_order.noisy(SB)
  LEAQ  -96(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L1
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $80, SP
  LEAQ  str.0(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -16(BP)
  MOVQ  $24, AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -32(BP)
  MOVQ  $2, AX
  MOVQ  AX, -40(BP)
  MOVQ  -32(BP), AX
  MOVQ  -40(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -32(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -32(BP), AX
  MOVQ  -16(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -32(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -56(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.0(SB), AX
  JMP   rt.fault(SB)

TEXT arg_order.noisy_text(SB)
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
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  16(BP), AX
  MOVQ  AX, 24(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L1
  LEAQ  site.1(SB), AX
  JMP   rt.fault(SB)

TEXT arg_order.diff(SB)
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

TEXT arg_order.main(SB)
  LEAQ  -880(SP), R11
  CMPQ  R11, rt.stack_limit(SB)
  JCS   L4
  PUSHQ BP
  MOVQ  SP, BP
  SUBQ  $864, SP
  LEAQ  str.1(SB), AX
  MOVQ  AX, -8(BP)
  MOVQ  $1, AX
  MOVQ  AX, -16(BP)
  MOVQ  -16(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -24(BP)
  MOVQ  -24(BP), AX
  MOVQ  AX, -32(BP)
  MOVQ  $3, AX
  MOVQ  AX, -40(BP)
  MOVQ  -40(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -48(BP)
  MOVQ  -48(BP), AX
  MOVQ  AX, -56(BP)
  MOVQ  -56(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -32(BP), AX
  MOVQ  AX, 8(SP)
  CALL  arg_order.diff(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -64(BP)
  MOVQ  -64(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -72(BP)
  MOVQ  $24, AX
  MOVQ  AX, -80(BP)
  MOVQ  -80(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -88(BP)
  MOVQ  $2, AX
  MOVQ  AX, -96(BP)
  MOVQ  -88(BP), AX
  MOVQ  -96(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -88(BP), AX
  MOVQ  -8(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -88(BP), AX
  MOVQ  -72(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -88(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -104(BP)
  MOVQ  -104(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -112(BP)
  MOVQ  $2, AX
  MOVQ  AX, -120(BP)
  MOVQ  -120(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -128(BP)
  MOVQ  -128(BP), AX
  MOVQ  AX, -136(BP)
  MOVQ  $1, AX
  MOVQ  AX, -144(BP)
  MOVQ  -144(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -152(BP)
  MOVQ  -152(BP), AX
  MOVQ  AX, -160(BP)
  MOVQ  $16, AX
  MOVQ  AX, -168(BP)
  MOVQ  -168(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -176(BP)
  MOVQ  -176(BP), AX
  MOVQ  -160(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -176(BP), AX
  MOVQ  -136(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -176(BP), AX
  MOVQ  AX, -184(BP)
  LEAQ  str.2(SB), AX
  MOVQ  AX, -192(BP)
  MOVQ  -184(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -200(BP)
  MOVQ  -200(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -208(BP)
  LEAQ  str.3(SB), AX
  MOVQ  AX, -216(BP)
  MOVQ  -184(BP), AX
  MOVQ  8(AX), AX
  MOVQ  AX, -224(BP)
  MOVQ  -224(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -232(BP)
  MOVQ  $40, AX
  MOVQ  AX, -240(BP)
  MOVQ  -240(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -248(BP)
  MOVQ  $4, AX
  MOVQ  AX, -256(BP)
  MOVQ  -248(BP), AX
  MOVQ  -256(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -248(BP), AX
  MOVQ  -192(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -248(BP), AX
  MOVQ  -208(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -248(BP), AX
  MOVQ  -216(BP), CX
  MOVQ  CX, 24(AX)
  MOVQ  -248(BP), AX
  MOVQ  -232(BP), CX
  MOVQ  CX, 32(AX)
  MOVQ  -248(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -264(BP)
  MOVQ  -264(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -272(BP)
  LEAQ  str.4(SB), AX
  MOVQ  AX, -280(BP)
  MOVQ  $5, AX
  MOVQ  AX, -288(BP)
  MOVQ  -288(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -296(BP)
  MOVQ  -296(BP), AX
  MOVQ  AX, -304(BP)
  MOVQ  $4, AX
  MOVQ  AX, -312(BP)
  MOVQ  -312(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -320(BP)
  MOVQ  -320(BP), AX
  MOVQ  AX, -328(BP)
  MOVQ  -328(BP), AX
  MOVQ  -304(BP), CX
  CMPQ  AX, CX
  SETGE AX
  MOVBQZX AX, AX
  MOVQ  AX, -344(BP)
  MOVQ  -344(BP), AX
  TESTQ AX, AX
  JNE   L1
  JMP   L2
LABEL L1
  MOVQ  -328(BP), AX
  MOVQ  AX, -336(BP)
  JMP   L3
LABEL L2
  MOVQ  -304(BP), AX
  MOVQ  AX, -336(BP)
  JMP   L3
LABEL L3
  MOVQ  -336(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -352(BP)
  MOVQ  $24, AX
  MOVQ  AX, -360(BP)
  MOVQ  -360(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -368(BP)
  MOVQ  $2, AX
  MOVQ  AX, -376(BP)
  MOVQ  -368(BP), AX
  MOVQ  -376(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -368(BP), AX
  MOVQ  -280(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -368(BP), AX
  MOVQ  -352(BP), CX
  MOVQ  CX, 16(AX)
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
  LEAQ  str.5(SB), AX
  MOVQ  AX, -400(BP)
  MOVQ  -400(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -408(BP)
  MOVQ  -408(BP), AX
  MOVQ  AX, -416(BP)
  LEAQ  str.6(SB), AX
  MOVQ  AX, -424(BP)
  MOVQ  -424(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -432(BP)
  MOVQ  -432(BP), AX
  MOVQ  AX, -440(BP)
  LEAQ  site.5(SB), AX
  MOVQ  AX, -448(BP)
  MOVQ  -440(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -416(BP), AX
  MOVQ  AX, 8(SP)
  MOVQ  -448(BP), AX
  MOVQ  AX, 16(SP)
  CALL  rt.split(SB)
  MOVQ  24(SP), AX
  MOVQ  AX, -456(BP)
  MOVQ  -456(BP), AX
  MOVQ  AX, -464(BP)
  LEAQ  str.7(SB), AX
  MOVQ  AX, -472(BP)
  MOVQ  -464(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -480(BP)
  MOVQ  -480(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -488(BP)
  MOVQ  $24, AX
  MOVQ  AX, -496(BP)
  MOVQ  -496(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -504(BP)
  MOVQ  $2, AX
  MOVQ  AX, -512(BP)
  MOVQ  -504(BP), AX
  MOVQ  -512(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -504(BP), AX
  MOVQ  -472(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -504(BP), AX
  MOVQ  -488(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -504(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -520(BP)
  MOVQ  -520(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -528(BP)
  LEAQ  str.8(SB), AX
  MOVQ  AX, -536(BP)
  MOVQ  $7, AX
  MOVQ  AX, -544(BP)
  MOVQ  -544(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -552(BP)
  MOVQ  $16, AX
  MOVQ  AX, -560(BP)
  MOVQ  -560(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -568(BP)
  MOVQ  $1, AX
  MOVQ  AX, -576(BP)
  MOVQ  -568(BP), AX
  MOVQ  -576(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -568(BP), AX
  MOVQ  -552(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -568(BP), AX
  MOVQ  AX, -584(BP)
  MOVQ  $6, AX
  MOVQ  AX, -592(BP)
  MOVQ  -592(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -600(BP)
  MOVQ  $16, AX
  MOVQ  AX, -608(BP)
  MOVQ  -608(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -616(BP)
  MOVQ  $1, AX
  MOVQ  AX, -624(BP)
  MOVQ  -616(BP), AX
  MOVQ  -624(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -616(BP), AX
  MOVQ  -600(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -616(BP), AX
  MOVQ  AX, -632(BP)
  MOVQ  -632(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -584(BP), AX
  MOVQ  AX, 8(SP)
  CALL  rt.join(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -640(BP)
  MOVQ  -640(BP), AX
  MOVQ  0(AX), AX
  MOVQ  AX, -648(BP)
  MOVQ  -648(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -656(BP)
  MOVQ  $24, AX
  MOVQ  AX, -664(BP)
  MOVQ  -664(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -672(BP)
  MOVQ  $2, AX
  MOVQ  AX, -680(BP)
  MOVQ  -672(BP), AX
  MOVQ  -680(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -672(BP), AX
  MOVQ  -536(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -672(BP), AX
  MOVQ  -656(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -672(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -688(BP)
  MOVQ  -688(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -696(BP)
  MOVQ  $9223372036854775807, AX
  MOVQ  AX, -704(BP)
  MOVQ  -704(BP), AX
  MOVQ  AX, -712(BP)
  LEAQ  str.9(SB), AX
  MOVQ  AX, -720(BP)
  MOVQ  $1, AX
  MOVQ  AX, -728(BP)
  MOVQ  -712(BP), AX
  MOVQ  -728(BP), CX
  ADDQ  CX, AX
  JOS   L5
  MOVQ  AX, -736(BP)
  MOVQ  -736(BP), AX
  MOVQ  AX, -744(BP)
  MOVQ  $9, AX
  MOVQ  AX, -752(BP)
  MOVQ  -752(BP), AX
  MOVQ  AX, 0(SP)
  CALL  arg_order.noisy(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -760(BP)
  MOVQ  -760(BP), AX
  MOVQ  AX, -768(BP)
  MOVQ  -768(BP), AX
  MOVQ  AX, 0(SP)
  MOVQ  -744(BP), AX
  MOVQ  AX, 8(SP)
  CALL  arg_order.diff(SB)
  MOVQ  16(SP), AX
  MOVQ  AX, -776(BP)
  MOVQ  -776(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.int_to_text(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -784(BP)
  MOVQ  $24, AX
  MOVQ  AX, -792(BP)
  MOVQ  -792(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.alloc(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -800(BP)
  MOVQ  $2, AX
  MOVQ  AX, -808(BP)
  MOVQ  -800(BP), AX
  MOVQ  -808(BP), CX
  MOVQ  CX, 0(AX)
  MOVQ  -800(BP), AX
  MOVQ  -720(BP), CX
  MOVQ  CX, 8(AX)
  MOVQ  -800(BP), AX
  MOVQ  -784(BP), CX
  MOVQ  CX, 16(AX)
  MOVQ  -800(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.text_concat(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -816(BP)
  MOVQ  -816(BP), AX
  MOVQ  AX, 0(SP)
  CALL  rt.log(SB)
  MOVQ  8(SP), AX
  MOVQ  AX, -824(BP)
  MOVQ  $0, AX
  MOVQ  AX, -832(BP)
  MOVQ  -832(BP), AX
  MOVQ  AX, 16(BP)
  MOVQ  BP, SP
  POPQ  BP
  RET
LABEL L4
  LEAQ  site.4(SB), AX
  JMP   rt.fault(SB)
LABEL L5
  LEAQ  site.6(SB), AX
  JMP   rt.fault(SB)
