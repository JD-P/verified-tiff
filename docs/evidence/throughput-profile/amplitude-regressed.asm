<verified_tiff::jpeg_block::amplitude>:
   38300:	push   %rbp
   38301:	push   %r15
   38303:	push   %r14
   38305:	push   %r13
   38307:	push   %r12
   38309:	push   %rbx
   3830a:	push   %rax
   3830b:	mov    %rcx,%rax
   3830e:	test   %r8,%r8
   38311:	je     3843e <verified_tiff::jpeg_block::amplitude+0x13e>
   38317:	mov    %rdi,(%rsp)
   3831b:	mov    (%rax),%r9
   3831e:	mov    0x8(%rax),%rbx
   38322:	mov    0x10(%rax),%rcx
   38326:	movzbl 0x18(%rax),%r12d
   3832b:	xor    %r11d,%r11d
   3832e:	lea    -0x2d435(%rip),%rdi        # af00 <anon.852f3d10e02ed531f90e77db4f8d107a.12.llvm.9238785854870498451+0xc98>
   38335:	xor    %r15d,%r15d
   38338:	jmp    38377 <verified_tiff::jpeg_block::amplitude+0x77>
   3833a:	nopw   0x0(%rax,%rax,1)
   38340:	mov    %r12d,%ecx
   38343:	shr    $0x7,%ecx
   38346:	mov    $0x1,%r10d
   3834c:	add    %r10,%r9
   3834f:	mov    %r9,(%rax)
   38352:	mov    %rbx,0x8(%rax)
   38356:	movq   $0x7,0x10(%rax)
   3835e:	mov    %r12b,0x18(%rax)
   38362:	lea    (%rcx,%r11,2),%r11
   38366:	inc    %r15
   38369:	mov    $0x7,%ecx
   3836e:	cmp    %r8,%r15
   38371:	jae    38410 <verified_tiff::jpeg_block::amplitude+0x110>
   38377:	test   %rcx,%rcx
   3837a:	je     383c0 <verified_tiff::jpeg_block::amplitude+0xc0>
   3837c:	mov    %r8,%r10
   3837f:	sub    %r15,%r10
   38382:	cmp    %r10,%rcx
   38385:	cmovb  %rcx,%r10
   38389:	cmp    $0x11,%r10
   3838d:	jae    38474 <verified_tiff::jpeg_block::amplitude+0x174>
   38393:	sub    %r10,%rcx
   38396:	movzbl %r12b,%r13d
   3839a:	shr    %cl,%r13
   3839d:	mov    (%rdi,%r10,8),%rbp
   383a1:	lea    -0x1(%rbp),%r14d
   383a5:	and    %r14d,%r13d
   383a8:	imul   %rbp,%r11
   383ac:	add    %r13,%r11
   383af:	mov    %rcx,0x10(%rax)
   383b3:	add    %r10,%r15
   383b6:	cmp    %r8,%r15
   383b9:	jb     38377 <verified_tiff::jpeg_block::amplitude+0x77>
   383bb:	jmp    38410 <verified_tiff::jpeg_block::amplitude+0x110>
   383bd:	nopl   (%rax)
   383c0:	mov    %rbx,%rcx
   383c3:	sub    %r9,%rcx
   383c6:	je     38459 <verified_tiff::jpeg_block::amplitude+0x159>
   383cc:	cmp    %rdx,%r9
   383cf:	jae    38489 <verified_tiff::jpeg_block::amplitude+0x189>
   383d5:	movzbl (%rsi,%r9,1),%r12d
   383da:	cmp    $0xff,%r12
   383e1:	jne    38340 <verified_tiff::jpeg_block::amplitude+0x40>
   383e7:	cmp    $0x2,%rcx
   383eb:	jb     38459 <verified_tiff::jpeg_block::amplitude+0x159>
   383ed:	lea    0x1(%r9),%rcx
   383f1:	cmp    %rdx,%rcx
   383f4:	jae    3849f <verified_tiff::jpeg_block::amplitude+0x19f>
   383fa:	cmpb   $0x0,(%rsi,%rcx,1)
   383fe:	jne    38459 <verified_tiff::jpeg_block::amplitude+0x159>
   38400:	mov    $0x1,%ecx
   38405:	mov    $0x2,%r10d
   3840b:	jmp    3834c <verified_tiff::jpeg_block::amplitude+0x4c>
   38410:	mov    (%rdi,%r8,8),%rcx
   38414:	neg    %rcx
   38417:	cmp    -0x8(%rdi,%r8,8),%r11
   3841c:	lea    0x1(%r11,%rcx,1),%rcx
   38421:	cmovae %r11,%rcx
   38425:	movups (%rax),%xmm0
   38428:	movups 0x10(%rax),%xmm1
   3842c:	mov    (%rsp),%rdi
   38430:	movups %xmm1,0x20(%rdi)
   38434:	movups %xmm0,0x10(%rdi)
   38438:	mov    %rcx,0x8(%rdi)
   3843c:	jmp    38455 <verified_tiff::jpeg_block::amplitude+0x155>
   3843e:	movups (%rax),%xmm0
   38441:	movups 0x10(%rax),%xmm1
   38445:	movups %xmm1,0x20(%rdi)
   38449:	movups %xmm0,0x10(%rdi)
   3844d:	movq   $0x0,0x8(%rdi)
   38455:	xor    %eax,%eax
   38457:	jmp    38463 <verified_tiff::jpeg_block::amplitude+0x163>
   38459:	mov    (%rsp),%rdi
   3845d:	movb   $0x1,0x1(%rdi)
   38461:	mov    $0x1,%al
   38463:	mov    %al,(%rdi)
   38465:	add    $0x8,%rsp
   38469:	pop    %rbx
   3846a:	pop    %r12
   3846c:	pop    %r13
   3846e:	pop    %r14
   38470:	pop    %r15
   38472:	pop    %rbp
   38473:	ret
   38474:	lea    0x54f55(%rip),%rdx        # 8d3d0 <anon.fc71f9b3ebc022ebc648ec7245baf12b.28.llvm.5578679714243452636+0x588>
   3847b:	mov    $0x11,%esi
   38480:	mov    %r10,%rdi
   38483:	call   *0x57eb7(%rip)        # 90340 <_DYNAMIC+0x368>
   38489:	lea    0x551e0(%rip),%rax        # 8d670 <anon.fc71f9b3ebc022ebc648ec7245baf12b.28.llvm.5578679714243452636+0x828>
   38490:	mov    %r9,%rdi
   38493:	mov    %rdx,%rsi
   38496:	mov    %rax,%rdx
   38499:	call   *0x57ea1(%rip)        # 90340 <_DYNAMIC+0x368>
   3849f:	lea    0x551e2(%rip),%rax        # 8d688 <anon.fc71f9b3ebc022ebc648ec7245baf12b.28.llvm.5578679714243452636+0x840>
   384a6:	mov    %rcx,%rdi
   384a9:	mov    %rdx,%rsi
   384ac:	mov    %rax,%rdx
   384af:	call   *0x57e8b(%rip)        # 90340 <_DYNAMIC+0x368>
   384b5:	int3
   384b6:	int3
   384b7:	int3
   384b8:	int3
   384b9:	int3
   384ba:	int3
   384bb:	int3
   384bc:	int3
   384bd:	int3
   384be:	int3
   384bf:	int3