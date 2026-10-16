<verified_tiff::jpeg_huffman::decode>:
   363d0:	push   %rbp
   363d1:	push   %r15
   363d3:	push   %r14
   363d5:	push   %r13
   363d7:	push   %r12
   363d9:	push   %rbx
   363da:	sub    $0x38,%rsp
   363de:	mov    %rdi,0x8(%rsp)
   363e3:	mov    0x10(%rcx),%r13
   363e7:	mov    (%rcx),%rax
   363ea:	mov    0x8(%rcx),%r10
   363ee:	movzbl 0x18(%rcx),%r9d
   363f3:	lea    (%rsi,%r8,1),%rdi
   363f7:	xor    %ebx,%ebx
   363f9:	xor    %r14d,%r14d
   363fc:	xor    %r12d,%r12d
   363ff:	xor    %r15d,%r15d
   36402:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   36410:	test   %r13,%r13
   36413:	je     36430 <verified_tiff::jpeg_huffman::decode+0x60>
   36415:	dec    %r13
   36418:	movzbl %r9b,%r11d
   3641c:	xor    %ebp,%ebp
   3641e:	bt     %r13,%r11
   36422:	setb   %bpl
   36426:	jmp    36495 <verified_tiff::jpeg_huffman::decode+0xc5>
   36428:	nopl   0x0(%rax,%rax,1)
   36430:	mov    %r10,%r13
   36433:	sub    %rax,%r13
   36436:	je     36541 <verified_tiff::jpeg_huffman::decode+0x171>
   3643c:	cmp    %rdx,%rax
   3643f:	jae    3657b <verified_tiff::jpeg_huffman::decode+0x1ab>
   36445:	movzbl (%rsi,%rax,1),%r9d
   3644a:	cmp    $0xff,%r9
   36451:	je     3645e <verified_tiff::jpeg_huffman::decode+0x8e>
   36453:	mov    %r9d,%ebp
   36456:	shr    $0x7,%ebp
   36459:	inc    %rax
   3645c:	jmp    3648f <verified_tiff::jpeg_huffman::decode+0xbf>
   3645e:	mov    $0x1,%bpl
   36461:	cmp    $0x2,%r13
   36465:	jb     36543 <verified_tiff::jpeg_huffman::decode+0x173>
   3646b:	lea    0x1(%rax),%r9
   3646f:	cmp    %rdx,%r9
   36472:	jae    36591 <verified_tiff::jpeg_huffman::decode+0x1c1>
   36478:	cmpb   $0x0,(%rsi,%r9,1)
   3647d:	jne    36543 <verified_tiff::jpeg_huffman::decode+0x173>
   36483:	add    $0x2,%rax
   36487:	mov    $0xff,%r9b
   3648a:	mov    $0x1,%ebp
   3648f:	mov    $0x7,%r13d
   36495:	mov    %rax,(%rcx)
   36498:	mov    %r10,0x8(%rcx)
   3649c:	mov    %r13,0x10(%rcx)
   364a0:	mov    %r9b,0x18(%rcx)
   364a4:	lea    (%r8,%r14,1),%r11
   364a8:	cmp    %rdx,%r11
   364ab:	jae    3655e <verified_tiff::jpeg_huffman::decode+0x18e>
   364b1:	lea    0x0(,%rbx,2),%rbx
   364b9:	add    %rbp,%rbx
   364bc:	mov    %rbx,%rbp
   364bf:	sub    %r12,%rbp
   364c2:	jb     364e4 <verified_tiff::jpeg_huffman::decode+0x114>
   364c4:	movzbl (%rdi,%r14,1),%r11d
   364c9:	add    %r11,%r12
   364cc:	cmp    %r12,%rbx
   364cf:	jb     364f0 <verified_tiff::jpeg_huffman::decode+0x120>
   364d1:	add    %r12,%r12
   364d4:	add    %r11,%r15
   364d7:	inc    %r14
   364da:	cmp    $0x10,%r14
   364de:	jne    36410 <verified_tiff::jpeg_huffman::decode+0x40>
   364e4:	mov    0x8(%rsp),%rax
   364e9:	movw   $0x101,(%rax)
   364ee:	jmp    3654f <verified_tiff::jpeg_huffman::decode+0x17f>
   364f0:	add    %r15,%r8
   364f3:	lea    (%r8,%rbp,1),%rax
   364f7:	add    $0x10,%rax
   364fb:	cmp    %rdx,%rax
   364fe:	jae    365a7 <verified_tiff::jpeg_huffman::decode+0x1d7>
   36504:	movzbl (%rsi,%rax,1),%edx
   36508:	movups (%rcx),%xmm0
   3650b:	movups 0x10(%rcx),%xmm1
   3650f:	movups %xmm1,0x28(%rsp)
   36514:	movups %xmm0,0x18(%rsp)
   36519:	mov    0x8(%rsp),%rax
   3651e:	mov    %dl,0x8(%rax)
   36521:	movups 0x11(%rsp),%xmm0
   36526:	movups 0x21(%rsp),%xmm1
   3652b:	movups %xmm0,0x9(%rax)
   3652f:	movups %xmm1,0x19(%rax)
   36533:	mov    0x30(%rsp),%rcx
   36538:	mov    %rcx,0x28(%rax)
   3653c:	movb   $0x0,(%rax)
   3653f:	jmp    3654f <verified_tiff::jpeg_huffman::decode+0x17f>
   36541:	xor    %ebp,%ebp
   36543:	mov    0x8(%rsp),%rax
   36548:	mov    %bpl,0x1(%rax)
   3654c:	movb   $0x1,(%rax)
   3654f:	add    $0x38,%rsp
   36553:	pop    %rbx
   36554:	pop    %r12
   36556:	pop    %r13
   36558:	pop    %r14
   3655a:	pop    %r15
   3655c:	pop    %rbp
   3655d:	ret
   3655e:	cmp    %rdx,%r8
   36561:	cmovbe %rdx,%r8
   36565:	lea    0x56bdc(%rip),%rax        # 8d148 <anon.fc71f9b3ebc022ebc648ec7245baf12b.28.llvm.5578679714243452636+0x300>
   3656c:	mov    %r8,%rdi
   3656f:	mov    %rdx,%rsi
   36572:	mov    %rax,%rdx
   36575:	call   *0x59dc5(%rip)        # 90340 <_DYNAMIC+0x368>
   3657b:	lea    0x56c86(%rip),%rcx        # 8d208 <anon.fc71f9b3ebc022ebc648ec7245baf12b.28.llvm.5578679714243452636+0x3c0>
   36582:	mov    %rax,%rdi
   36585:	mov    %rdx,%rsi
   36588:	mov    %rcx,%rdx
   3658b:	call   *0x59daf(%rip)        # 90340 <_DYNAMIC+0x368>
   36591:	lea    0x56c88(%rip),%rax        # 8d220 <anon.fc71f9b3ebc022ebc648ec7245baf12b.28.llvm.5578679714243452636+0x3d8>
   36598:	mov    %r9,%rdi
   3659b:	mov    %rdx,%rsi
   3659e:	mov    %rax,%rdx
   365a1:	call   *0x59d99(%rip)        # 90340 <_DYNAMIC+0x368>
   365a7:	lea    0x56bb2(%rip),%rcx        # 8d160 <anon.fc71f9b3ebc022ebc648ec7245baf12b.28.llvm.5578679714243452636+0x318>
   365ae:	mov    %rax,%rdi
   365b1:	mov    %rdx,%rsi
   365b4:	mov    %rcx,%rdx
   365b7:	call   *0x59d83(%rip)        # 90340 <_DYNAMIC+0x368>
   365bd:	int3
   365be:	int3
   365bf:	int3