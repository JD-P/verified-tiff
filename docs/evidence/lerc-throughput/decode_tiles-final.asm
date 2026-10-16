
target/compat/libtiff.so.6:     file format elf64-x86-64


Disassembly of section .text:

000000000006b360 <verified_tiff::lerc_image::decode_tiles>:
   6b360:	55                   	push   %rbp
   6b361:	41 57                	push   %r15
   6b363:	41 56                	push   %r14
   6b365:	41 55                	push   %r13
   6b367:	41 54                	push   %r12
   6b369:	53                   	push   %rbx
   6b36a:	48 81 ec 38 05 00 00 	sub    $0x538,%rsp
   6b371:	48 89 cb             	mov    %rcx,%rbx
   6b374:	49 89 d6             	mov    %rdx,%r14
   6b377:	48 89 74 24 70       	mov    %rsi,0x70(%rsp)
   6b37c:	48 89 bc 24 a0 00 00 	mov    %rdi,0xa0(%rsp)
   6b383:	00 
   6b384:	48 8b 11             	mov    (%rcx),%rdx
   6b387:	4c 8b 69 08          	mov    0x8(%rcx),%r13
   6b38b:	4d 89 ec             	mov    %r13,%r12
   6b38e:	4c 0f af e2          	imul   %rdx,%r12
   6b392:	49 8d 44 24 07       	lea    0x7(%r12),%rax
   6b397:	48 c1 e8 03          	shr    $0x3,%rax
   6b39b:	74 32                	je     6b3cf <verified_tiff::lerc_image::decode_tiles+0x6f>
   6b39d:	48 ff c8             	dec    %rax
   6b3a0:	31 c9                	xor    %ecx,%ecx
   6b3a2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6b3a9:	1f 84 00 00 00 00 00 
   6b3b0:	49 39 c9             	cmp    %rcx,%r9
   6b3b3:	0f 84 d0 12 00 00    	je     6c689 <verified_tiff::lerc_image::decode_tiles+0x1329>
   6b3b9:	41 80 3c 08 ff       	cmpb   $0xff,(%r8,%rcx,1)
   6b3be:	41 0f 95 c7          	setne  %r15b
   6b3c2:	75 0e                	jne    6b3d2 <verified_tiff::lerc_image::decode_tiles+0x72>
   6b3c4:	48 39 c8             	cmp    %rcx,%rax
   6b3c7:	48 8d 49 01          	lea    0x1(%rcx),%rcx
   6b3cb:	75 e3                	jne    6b3b0 <verified_tiff::lerc_image::decode_tiles+0x50>
   6b3cd:	eb 03                	jmp    6b3d2 <verified_tiff::lerc_image::decode_tiles+0x72>
   6b3cf:	45 31 ff             	xor    %r15d,%r15d
   6b3d2:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
   6b3d7:	4c 89 44 24 28       	mov    %r8,0x28(%rsp)
   6b3dc:	4c 89 4c 24 30       	mov    %r9,0x30(%rsp)
   6b3e1:	48 8b ac 24 78 05 00 	mov    0x578(%rsp),%rbp
   6b3e8:	00 
   6b3e9:	48 8d bc 24 28 01 00 	lea    0x128(%rsp),%rdi
   6b3f0:	00 
   6b3f1:	ba 08 04 00 00       	mov    $0x408,%edx
   6b3f6:	31 f6                	xor    %esi,%esi
   6b3f8:	ff 15 6a da 12 00    	call   *0x12da6a(%rip)        # 198e68 <memset@GLIBC_2.2.5>
   6b3fe:	4d 85 ed             	test   %r13,%r13
   6b401:	0f 84 21 12 00 00    	je     6c628 <verified_tiff::lerc_image::decode_tiles+0x12c8>
   6b407:	48 8b 43 10          	mov    0x10(%rbx),%rax
   6b40b:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6b412:	00 f0 7f 
   6b415:	48 89 44 24 78       	mov    %rax,0x78(%rsp)
   6b41a:	4c 0f af e0          	imul   %rax,%r12
   6b41e:	48 8b 73 30          	mov    0x30(%rbx),%rsi
   6b422:	48 8b 7b 38          	mov    0x38(%rbx),%rdi
   6b426:	48 89 f8             	mov    %rdi,%rax
   6b429:	4c 21 c0             	and    %r8,%rax
   6b42c:	48 89 84 24 98 00 00 	mov    %rax,0x98(%rsp)
   6b433:	00 
   6b434:	31 c0                	xor    %eax,%eax
   6b436:	48 83 fe 07          	cmp    $0x7,%rsi
   6b43a:	0f 93 c0             	setae  %al
   6b43d:	41 b9 03 00 00 00    	mov    $0x3,%r9d
   6b443:	49 83 d9 00          	sbb    $0x0,%r9
   6b447:	48 83 fe 04          	cmp    $0x4,%rsi
   6b44b:	b9 00 00 01 00       	mov    $0x10000,%ecx
   6b450:	48 ba 00 00 00 00 01 	movabs $0x100000000,%rdx
   6b457:	00 00 00 
   6b45a:	48 0f 42 d1          	cmovb  %rcx,%rdx
   6b45e:	48 8d 04 85 04 00 00 	lea    0x4(,%rax,4),%rax
   6b465:	00 
   6b466:	41 ba 02 00 00 00    	mov    $0x2,%r10d
   6b46c:	4c 0f 43 d0          	cmovae %rax,%r10
   6b470:	b8 01 00 00 00       	mov    $0x1,%eax
   6b475:	4c 0f 42 c8          	cmovb  %rax,%r9
   6b479:	45 31 db             	xor    %r11d,%r11d
   6b47c:	48 89 b4 24 a8 00 00 	mov    %rsi,0xa8(%rsp)
   6b483:	00 
   6b484:	48 83 fe 02          	cmp    $0x2,%rsi
   6b488:	be 00 01 00 00       	mov    $0x100,%esi
   6b48d:	48 0f 43 f2          	cmovae %rdx,%rsi
   6b491:	48 8b 4b 20          	mov    0x20(%rbx),%rcx
   6b495:	48 89 8c 24 10 01 00 	mov    %rcx,0x110(%rsp)
   6b49c:	00 
   6b49d:	4c 89 c9             	mov    %r9,%rcx
   6b4a0:	48 89 bc 24 88 00 00 	mov    %rdi,0x88(%rsp)
   6b4a7:	00 
   6b4a8:	48 89 bc 24 80 00 00 	mov    %rdi,0x80(%rsp)
   6b4af:	00 
   6b4b0:	4c 0f 42 d0          	cmovb  %rax,%r10
   6b4b4:	4c 89 94 24 b8 00 00 	mov    %r10,0xb8(%rsp)
   6b4bb:	00 
   6b4bc:	49 0f 42 cb          	cmovb  %r11,%rcx
   6b4c0:	48 89 b4 24 c8 00 00 	mov    %rsi,0xc8(%rsp)
   6b4c7:	00 
   6b4c8:	48 d1 ee             	shr    $1,%rsi
   6b4cb:	48 89 b4 24 00 01 00 	mov    %rsi,0x100(%rsp)
   6b4d2:	00 
   6b4d3:	48 8d 46 01          	lea    0x1(%rsi),%rax
   6b4d7:	48 89 84 24 f8 00 00 	mov    %rax,0xf8(%rsp)
   6b4de:	00 
   6b4df:	31 ff                	xor    %edi,%edi
   6b4e1:	31 f6                	xor    %esi,%esi
   6b4e3:	48 c7 44 24 60 00 00 	movq   $0x0,0x60(%rsp)
   6b4ea:	00 00 
   6b4ec:	4c 89 74 24 58       	mov    %r14,0x58(%rsp)
   6b4f1:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6b4f6:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6b4fb:	4c 8b 5c 24 20       	mov    0x20(%rsp),%r11
   6b500:	4c 89 ac 24 d0 00 00 	mov    %r13,0xd0(%rsp)
   6b507:	00 
   6b508:	48 89 eb             	mov    %rbp,%rbx
   6b50b:	48 89 8c 24 20 01 00 	mov    %rcx,0x120(%rsp)
   6b512:	00 
   6b513:	49 83 ec 01          	sub    $0x1,%r12
   6b517:	0f 82 31 11 00 00    	jb     6c64e <verified_tiff::lerc_image::decode_tiles+0x12ee>
   6b51d:	4c 89 a4 24 d8 00 00 	mov    %r12,0xd8(%rsp)
   6b524:	00 
   6b525:	4c 89 da             	mov    %r11,%rdx
   6b528:	48 2b 54 24 60       	sub    0x60(%rsp),%rdx
   6b52d:	48 8b 84 24 10 01 00 	mov    0x110(%rsp),%rax
   6b534:	00 
   6b535:	48 39 c2             	cmp    %rax,%rdx
   6b538:	48 0f 43 d0          	cmovae %rax,%rdx
   6b53c:	4c 89 ed             	mov    %r13,%rbp
   6b53f:	48 29 f5             	sub    %rsi,%rbp
   6b542:	48 39 c5             	cmp    %rax,%rbp
   6b545:	48 0f 43 e8          	cmovae %rax,%rbp
   6b549:	4c 39 f3             	cmp    %r14,%rbx
   6b54c:	0f 84 0a 11 00 00    	je     6c65c <verified_tiff::lerc_image::decode_tiles+0x12fc>
   6b552:	48 89 54 24 50       	mov    %rdx,0x50(%rsp)
   6b557:	0f 83 a5 11 00 00    	jae    6c702 <verified_tiff::lerc_image::decode_tiles+0x13a2>
   6b55d:	4d 89 dc             	mov    %r11,%r12
   6b560:	4d 89 d3             	mov    %r10,%r11
   6b563:	4d 89 ca             	mov    %r9,%r10
   6b566:	48 8b 44 24 70       	mov    0x70(%rsp),%rax
   6b56b:	49 89 d9             	mov    %rbx,%r9
   6b56e:	0f b6 04 18          	movzbl (%rax,%rbx,1),%eax
   6b572:	89 c1                	mov    %eax,%ecx
   6b574:	c1 e9 02             	shr    $0x2,%ecx
   6b577:	48 8b 54 24 60       	mov    0x60(%rsp),%rdx
   6b57c:	c1 ea 03             	shr    $0x3,%edx
   6b57f:	31 ca                	xor    %ecx,%edx
   6b581:	b3 01                	mov    $0x1,%bl
   6b583:	f6 c2 0f             	test   $0xf,%dl
   6b586:	4c 8b ac 24 a8 00 00 	mov    0xa8(%rsp),%r13
   6b58d:	00 
   6b58e:	0f 85 d4 10 00 00    	jne    6c668 <verified_tiff::lerc_image::decode_tiles+0x1308>
   6b594:	89 c1                	mov    %eax,%ecx
   6b596:	83 e1 03             	and    $0x3,%ecx
   6b599:	48 89 4c 24 40       	mov    %rcx,0x40(%rsp)
   6b59e:	48 c7 84 24 30 05 00 	movq   $0x0,0x530(%rsp)
   6b5a5:	00 00 00 00 00 
   6b5aa:	a8 01                	test   $0x1,%al
   6b5ac:	48 89 74 24 10       	mov    %rsi,0x10(%rsp)
   6b5b1:	48 89 7c 24 68       	mov    %rdi,0x68(%rsp)
   6b5b6:	48 89 ac 24 b0 00 00 	mov    %rbp,0xb0(%rsp)
   6b5bd:	00 
   6b5be:	0f 85 9c 02 00 00    	jne    6b860 <verified_tiff::lerc_image::decode_tiles+0x500>
   6b5c4:	49 ff c1             	inc    %r9
   6b5c7:	48 c7 44 24 38 00 00 	movq   $0x0,0x38(%rsp)
   6b5ce:	00 00 
   6b5d0:	48 c7 44 24 48 00 00 	movq   $0x0,0x48(%rsp)
   6b5d7:	00 00 
   6b5d9:	48 83 ff 40          	cmp    $0x40,%rdi
   6b5dd:	0f 83 33 10 00 00    	jae    6c616 <verified_tiff::lerc_image::decode_tiles+0x12b6>
   6b5e3:	48 8b 84 24 70 05 00 	mov    0x570(%rsp),%rax
   6b5ea:	00 
   6b5eb:	48 8b 04 f8          	mov    (%rax,%rdi,8),%rax
   6b5ef:	48 89 04 24          	mov    %rax,(%rsp)
   6b5f3:	b1 01                	mov    $0x1,%cl
   6b5f5:	83 7c 24 40 01       	cmpl   $0x1,0x40(%rsp)
   6b5fa:	0f 85 50 02 00 00    	jne    6b850 <verified_tiff::lerc_image::decode_tiles+0x4f0>
   6b600:	49 83 fd 07          	cmp    $0x7,%r13
   6b604:	0f 84 96 02 00 00    	je     6b8a0 <verified_tiff::lerc_image::decode_tiles+0x540>
   6b60a:	49 83 fd 06          	cmp    $0x6,%r13
   6b60e:	0f 85 7d 04 00 00    	jne    6ba91 <verified_tiff::lerc_image::decode_tiles+0x731>
   6b614:	4c 89 4c 24 18       	mov    %r9,0x18(%rsp)
   6b619:	48 8b 5c 24 50       	mov    0x50(%rsp),%rbx
   6b61e:	48 0f af dd          	imul   %rbp,%rbx
   6b622:	45 84 ff             	test   %r15b,%r15b
   6b625:	0f 84 72 04 00 00    	je     6ba9d <verified_tiff::lerc_image::decode_tiles+0x73d>
   6b62b:	48 85 db             	test   %rbx,%rbx
   6b62e:	4d 89 d1             	mov    %r10,%r9
   6b631:	0f 84 0e 08 00 00    	je     6be45 <verified_tiff::lerc_image::decode_tiles+0xae5>
   6b637:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
   6b63c:	48 89 c1             	mov    %rax,%rcx
   6b63f:	4c 21 c1             	and    %r8,%rcx
   6b642:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
   6b647:	48 89 84 24 90 00 00 	mov    %rax,0x90(%rsp)
   6b64e:	00 
   6b64f:	31 d2                	xor    %edx,%edx
   6b651:	45 31 f6             	xor    %r14d,%r14d
   6b654:	45 31 ed             	xor    %r13d,%r13d
   6b657:	4d 89 da             	mov    %r11,%r10
   6b65a:	4d 89 e3             	mov    %r12,%r11
   6b65d:	4c 8b 64 24 50       	mov    0x50(%rsp),%r12
   6b662:	eb 30                	jmp    6b694 <verified_tiff::lerc_image::decode_tiles+0x334>
   6b664:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   6b66b:	00 00 00 00 00 
   6b670:	4c 89 e8             	mov    %r13,%rax
   6b673:	48 ff c0             	inc    %rax
   6b676:	31 c9                	xor    %ecx,%ecx
   6b678:	4c 39 e0             	cmp    %r12,%rax
   6b67b:	0f 94 c1             	sete   %cl
   6b67e:	41 bd 00 00 00 00    	mov    $0x0,%r13d
   6b684:	4c 0f 45 e8          	cmovne %rax,%r13
   6b688:	49 01 ce             	add    %rcx,%r14
   6b68b:	48 ff cb             	dec    %rbx
   6b68e:	0f 84 be 07 00 00    	je     6be52 <verified_tiff::lerc_image::decode_tiles+0xaf2>
   6b694:	49 8d 2c 36          	lea    (%r14,%rsi,1),%rbp
   6b698:	48 8b 44 24 60       	mov    0x60(%rsp),%rax
   6b69d:	4c 01 e8             	add    %r13,%rax
   6b6a0:	49 0f af eb          	imul   %r11,%rbp
   6b6a4:	48 01 c5             	add    %rax,%rbp
   6b6a7:	48 89 ef             	mov    %rbp,%rdi
   6b6aa:	48 c1 ef 03          	shr    $0x3,%rdi
   6b6ae:	4c 39 cf             	cmp    %r9,%rdi
   6b6b1:	0f 83 e5 0f 00 00    	jae    6c69c <verified_tiff::lerc_image::decode_tiles+0x133c>
   6b6b7:	89 e8                	mov    %ebp,%eax
   6b6b9:	f6 d0                	not    %al
   6b6bb:	41 0f b6 0c 3a       	movzbl (%r10,%rdi,1),%ecx
   6b6c0:	0f b6 c0             	movzbl %al,%eax
   6b6c3:	83 e0 07             	and    $0x7,%eax
   6b6c6:	0f a3 c1             	bt     %eax,%ecx
   6b6c9:	73 a5                	jae    6b670 <verified_tiff::lerc_image::decode_tiles+0x310>
   6b6cb:	48 3b 54 24 38       	cmp    0x38(%rsp),%rdx
   6b6d0:	0f 83 65 0f 00 00    	jae    6c63b <verified_tiff::lerc_image::decode_tiles+0x12db>
   6b6d6:	48 81 fa ff 00 00 00 	cmp    $0xff,%rdx
   6b6dd:	0f 87 32 10 00 00    	ja     6c715 <verified_tiff::lerc_image::decode_tiles+0x13b5>
   6b6e3:	8b b4 94 28 01 00 00 	mov    0x128(%rsp,%rdx,4),%esi
   6b6ea:	4c 39 84 24 98 00 00 	cmp    %r8,0x98(%rsp)
   6b6f1:	00 
   6b6f2:	48 89 54 24 40       	mov    %rdx,0x40(%rsp)
   6b6f7:	75 27                	jne    6b720 <verified_tiff::lerc_image::decode_tiles+0x3c0>
   6b6f9:	48 8b bc 24 88 00 00 	mov    0x88(%rsp),%rdi
   6b700:	00 
   6b701:	e8 9a ec 00 00       	call   7a3a0 <verified_tiff::lerc_numeric::scale_software>
   6b706:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6b70d:	00 f0 7f 
   6b710:	4c 39 44 24 08       	cmp    %r8,0x8(%rsp)
   6b715:	75 29                	jne    6b740 <verified_tiff::lerc_image::decode_tiles+0x3e0>
   6b717:	eb 47                	jmp    6b760 <verified_tiff::lerc_image::decode_tiles+0x400>
   6b719:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   6b720:	48 01 f6             	add    %rsi,%rsi
   6b723:	0f 57 c0             	xorps  %xmm0,%xmm0
   6b726:	f2 48 0f 2a c6       	cvtsi2sd %rsi,%xmm0
   6b72b:	f2 0f 59 84 24 80 00 	mulsd  0x80(%rsp),%xmm0
   6b732:	00 00 
   6b734:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6b739:	4c 39 44 24 08       	cmp    %r8,0x8(%rsp)
   6b73e:	74 20                	je     6b760 <verified_tiff::lerc_image::decode_tiles+0x400>
   6b740:	48 89 c1             	mov    %rax,%rcx
   6b743:	4c 21 c1             	and    %r8,%rcx
   6b746:	4c 39 c1             	cmp    %r8,%rcx
   6b749:	74 15                	je     6b760 <verified_tiff::lerc_image::decode_tiles+0x400>
   6b74b:	66 48 0f 6e c0       	movq   %rax,%xmm0
   6b750:	f2 0f 58 84 24 90 00 	addsd  0x90(%rsp),%xmm0
   6b757:	00 00 
   6b759:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6b75e:	eb 17                	jmp    6b777 <verified_tiff::lerc_image::decode_tiles+0x417>
   6b760:	48 8b 7c 24 48       	mov    0x48(%rsp),%rdi
   6b765:	48 89 c6             	mov    %rax,%rsi
   6b768:	e8 93 e5 00 00       	call   79d00 <verified_tiff::lerc_numeric::plus_software>
   6b76d:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6b774:	00 f0 7f 
   6b777:	48 8b 34 24          	mov    (%rsp),%rsi
   6b77b:	48 89 c1             	mov    %rax,%rcx
   6b77e:	48 09 f1             	or     %rsi,%rcx
   6b781:	48 89 f7             	mov    %rsi,%rdi
   6b784:	48 ba ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rdx
   6b78b:	ff ff 7f 
   6b78e:	48 85 d1             	test   %rdx,%rcx
   6b791:	74 23                	je     6b7b6 <verified_tiff::lerc_image::decode_tiles+0x456>
   6b793:	48 89 c1             	mov    %rax,%rcx
   6b796:	48 89 c7             	mov    %rax,%rdi
   6b799:	48 31 f1             	xor    %rsi,%rcx
   6b79c:	78 10                	js     6b7ae <verified_tiff::lerc_image::decode_tiles+0x44e>
   6b79e:	48 85 c0             	test   %rax,%rax
   6b7a1:	0f 88 8e 00 00 00    	js     6b835 <verified_tiff::lerc_image::decode_tiles+0x4d5>
   6b7a7:	48 39 f0             	cmp    %rsi,%rax
   6b7aa:	73 07                	jae    6b7b3 <verified_tiff::lerc_image::decode_tiles+0x453>
   6b7ac:	eb 08                	jmp    6b7b6 <verified_tiff::lerc_image::decode_tiles+0x456>
   6b7ae:	48 85 c0             	test   %rax,%rax
   6b7b1:	78 03                	js     6b7b6 <verified_tiff::lerc_image::decode_tiles+0x456>
   6b7b3:	48 89 f7             	mov    %rsi,%rdi
   6b7b6:	48 89 f8             	mov    %rdi,%rax
   6b7b9:	4c 21 c0             	and    %r8,%rax
   6b7bc:	4c 39 c0             	cmp    %r8,%rax
   6b7bf:	75 1f                	jne    6b7e0 <verified_tiff::lerc_image::decode_tiles+0x480>
   6b7c1:	e8 4a ee 00 00       	call   7a610 <verified_tiff::lerc_numeric::narrow_software>
   6b7c6:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6b7cd:	00 f0 7f 
   6b7d0:	eb 1b                	jmp    6b7ed <verified_tiff::lerc_image::decode_tiles+0x48d>
   6b7d2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6b7d9:	1f 84 00 00 00 00 00 
   6b7e0:	66 48 0f 6e c7       	movq   %rdi,%xmm0
   6b7e5:	f2 0f 5a c0          	cvtsd2ss %xmm0,%xmm0
   6b7e9:	66 0f 7e c0          	movd   %xmm0,%eax
   6b7ed:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6b7f2:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6b7f7:	4c 8b 5c 24 20       	mov    0x20(%rsp),%r11
   6b7fc:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6b801:	48 8b 54 24 40       	mov    0x40(%rsp),%rdx
   6b806:	48 0f af 6c 24 78    	imul   0x78(%rsp),%rbp
   6b80c:	48 03 6c 24 68       	add    0x68(%rsp),%rbp
   6b811:	48 3b ac 24 88 05 00 	cmp    0x588(%rsp),%rbp
   6b818:	00 
   6b819:	0f 83 0b 0f 00 00    	jae    6c72a <verified_tiff::lerc_image::decode_tiles+0x13ca>
   6b81f:	89 c0                	mov    %eax,%eax
   6b821:	48 8b 8c 24 80 05 00 	mov    0x580(%rsp),%rcx
   6b828:	00 
   6b829:	48 89 04 e9          	mov    %rax,(%rcx,%rbp,8)
   6b82d:	48 ff c2             	inc    %rdx
   6b830:	e9 3b fe ff ff       	jmp    6b670 <verified_tiff::lerc_image::decode_tiles+0x310>
   6b835:	48 39 f0             	cmp    %rsi,%rax
   6b838:	0f 86 75 ff ff ff    	jbe    6b7b3 <verified_tiff::lerc_image::decode_tiles+0x453>
   6b83e:	e9 73 ff ff ff       	jmp    6b7b6 <verified_tiff::lerc_image::decode_tiles+0x456>
   6b843:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6b84a:	84 00 00 00 00 00 
   6b850:	4c 89 4c 24 18       	mov    %r9,0x18(%rsp)
   6b855:	e9 18 07 00 00       	jmp    6bf72 <verified_tiff::lerc_image::decode_tiles+0xc12>
   6b85a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   6b860:	49 83 fd 07          	cmp    $0x7,%r13
   6b864:	0f 87 fe 0d 00 00    	ja     6c668 <verified_tiff::lerc_image::decode_tiles+0x1308>
   6b86a:	49 8d 4d fe          	lea    -0x2(%r13),%rcx
   6b86e:	44 89 ed             	mov    %r13d,%ebp
   6b871:	48 83 f9 05          	cmp    $0x5,%rcx
   6b875:	0f 87 25 06 00 00    	ja     6bea0 <verified_tiff::lerc_image::decode_tiles+0xb40>
   6b87b:	c1 e8 06             	shr    $0x6,%eax
   6b87e:	48 8d 15 73 c7 0d 00 	lea    0xdc773(%rip),%rdx        # 147ff8 <anon.7e66189caf5c456237a9eb57058ec56c.27.llvm.3294417352400817999+0x1598>
   6b885:	48 63 0c 8a          	movslq (%rdx,%rcx,4),%rcx
   6b889:	48 01 d1             	add    %rdx,%rcx
   6b88c:	ff e1                	jmp    *%rcx
   6b88e:	01 c0                	add    %eax,%eax
   6b890:	44 89 ed             	mov    %r13d,%ebp
   6b893:	29 c5                	sub    %eax,%ebp
   6b895:	0f 89 05 06 00 00    	jns    6bea0 <verified_tiff::lerc_image::decode_tiles+0xb40>
   6b89b:	e9 c8 0d 00 00       	jmp    6c668 <verified_tiff::lerc_image::decode_tiles+0x1308>
   6b8a0:	4c 89 4c 24 18       	mov    %r9,0x18(%rsp)
   6b8a5:	48 8b 5c 24 50       	mov    0x50(%rsp),%rbx
   6b8aa:	48 0f af dd          	imul   %rbp,%rbx
   6b8ae:	45 84 ff             	test   %r15b,%r15b
   6b8b1:	0f 84 ce 03 00 00    	je     6bc85 <verified_tiff::lerc_image::decode_tiles+0x925>
   6b8b7:	48 85 db             	test   %rbx,%rbx
   6b8ba:	4d 89 d1             	mov    %r10,%r9
   6b8bd:	0f 84 82 05 00 00    	je     6be45 <verified_tiff::lerc_image::decode_tiles+0xae5>
   6b8c3:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
   6b8c8:	48 89 c1             	mov    %rax,%rcx
   6b8cb:	4c 21 c1             	and    %r8,%rcx
   6b8ce:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
   6b8d3:	48 89 84 24 90 00 00 	mov    %rax,0x90(%rsp)
   6b8da:	00 
   6b8db:	31 d2                	xor    %edx,%edx
   6b8dd:	45 31 f6             	xor    %r14d,%r14d
   6b8e0:	45 31 ed             	xor    %r13d,%r13d
   6b8e3:	4d 89 da             	mov    %r11,%r10
   6b8e6:	4d 89 e3             	mov    %r12,%r11
   6b8e9:	4c 8b 64 24 50       	mov    0x50(%rsp),%r12
   6b8ee:	eb 24                	jmp    6b914 <verified_tiff::lerc_image::decode_tiles+0x5b4>
   6b8f0:	4c 89 e8             	mov    %r13,%rax
   6b8f3:	48 ff c0             	inc    %rax
   6b8f6:	31 c9                	xor    %ecx,%ecx
   6b8f8:	4c 39 e0             	cmp    %r12,%rax
   6b8fb:	0f 94 c1             	sete   %cl
   6b8fe:	41 bd 00 00 00 00    	mov    $0x0,%r13d
   6b904:	4c 0f 45 e8          	cmovne %rax,%r13
   6b908:	49 01 ce             	add    %rcx,%r14
   6b90b:	48 ff cb             	dec    %rbx
   6b90e:	0f 84 3e 05 00 00    	je     6be52 <verified_tiff::lerc_image::decode_tiles+0xaf2>
   6b914:	49 8d 2c 36          	lea    (%r14,%rsi,1),%rbp
   6b918:	48 8b 44 24 60       	mov    0x60(%rsp),%rax
   6b91d:	4c 01 e8             	add    %r13,%rax
   6b920:	49 0f af eb          	imul   %r11,%rbp
   6b924:	48 01 c5             	add    %rax,%rbp
   6b927:	48 89 ef             	mov    %rbp,%rdi
   6b92a:	48 c1 ef 03          	shr    $0x3,%rdi
   6b92e:	4c 39 cf             	cmp    %r9,%rdi
   6b931:	0f 83 65 0d 00 00    	jae    6c69c <verified_tiff::lerc_image::decode_tiles+0x133c>
   6b937:	89 e8                	mov    %ebp,%eax
   6b939:	f6 d0                	not    %al
   6b93b:	41 0f b6 0c 3a       	movzbl (%r10,%rdi,1),%ecx
   6b940:	0f b6 c0             	movzbl %al,%eax
   6b943:	83 e0 07             	and    $0x7,%eax
   6b946:	0f a3 c1             	bt     %eax,%ecx
   6b949:	73 a5                	jae    6b8f0 <verified_tiff::lerc_image::decode_tiles+0x590>
   6b94b:	48 3b 54 24 38       	cmp    0x38(%rsp),%rdx
   6b950:	0f 83 e5 0c 00 00    	jae    6c63b <verified_tiff::lerc_image::decode_tiles+0x12db>
   6b956:	48 81 fa ff 00 00 00 	cmp    $0xff,%rdx
   6b95d:	0f 87 b2 0d 00 00    	ja     6c715 <verified_tiff::lerc_image::decode_tiles+0x13b5>
   6b963:	8b b4 94 28 01 00 00 	mov    0x128(%rsp,%rdx,4),%esi
   6b96a:	4c 39 84 24 98 00 00 	cmp    %r8,0x98(%rsp)
   6b971:	00 
   6b972:	48 89 54 24 40       	mov    %rdx,0x40(%rsp)
   6b977:	75 27                	jne    6b9a0 <verified_tiff::lerc_image::decode_tiles+0x640>
   6b979:	48 8b bc 24 88 00 00 	mov    0x88(%rsp),%rdi
   6b980:	00 
   6b981:	e8 1a ea 00 00       	call   7a3a0 <verified_tiff::lerc_numeric::scale_software>
   6b986:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6b98d:	00 f0 7f 
   6b990:	4c 39 44 24 08       	cmp    %r8,0x8(%rsp)
   6b995:	75 29                	jne    6b9c0 <verified_tiff::lerc_image::decode_tiles+0x660>
   6b997:	eb 47                	jmp    6b9e0 <verified_tiff::lerc_image::decode_tiles+0x680>
   6b999:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   6b9a0:	48 01 f6             	add    %rsi,%rsi
   6b9a3:	0f 57 c0             	xorps  %xmm0,%xmm0
   6b9a6:	f2 48 0f 2a c6       	cvtsi2sd %rsi,%xmm0
   6b9ab:	f2 0f 59 84 24 80 00 	mulsd  0x80(%rsp),%xmm0
   6b9b2:	00 00 
   6b9b4:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6b9b9:	4c 39 44 24 08       	cmp    %r8,0x8(%rsp)
   6b9be:	74 20                	je     6b9e0 <verified_tiff::lerc_image::decode_tiles+0x680>
   6b9c0:	48 89 c1             	mov    %rax,%rcx
   6b9c3:	4c 21 c1             	and    %r8,%rcx
   6b9c6:	4c 39 c1             	cmp    %r8,%rcx
   6b9c9:	74 15                	je     6b9e0 <verified_tiff::lerc_image::decode_tiles+0x680>
   6b9cb:	66 48 0f 6e c0       	movq   %rax,%xmm0
   6b9d0:	f2 0f 58 84 24 90 00 	addsd  0x90(%rsp),%xmm0
   6b9d7:	00 00 
   6b9d9:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6b9de:	eb 17                	jmp    6b9f7 <verified_tiff::lerc_image::decode_tiles+0x697>
   6b9e0:	48 8b 7c 24 48       	mov    0x48(%rsp),%rdi
   6b9e5:	48 89 c6             	mov    %rax,%rsi
   6b9e8:	e8 13 e3 00 00       	call   79d00 <verified_tiff::lerc_numeric::plus_software>
   6b9ed:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6b9f4:	00 f0 7f 
   6b9f7:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6b9fc:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6ba01:	4c 8b 5c 24 20       	mov    0x20(%rsp),%r11
   6ba06:	48 8b 34 24          	mov    (%rsp),%rsi
   6ba0a:	48 89 c2             	mov    %rax,%rdx
   6ba0d:	48 09 f2             	or     %rsi,%rdx
   6ba10:	48 89 f1             	mov    %rsi,%rcx
   6ba13:	48 89 f7             	mov    %rsi,%rdi
   6ba16:	48 be ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rsi
   6ba1d:	ff ff 7f 
   6ba20:	48 85 f2             	test   %rsi,%rdx
   6ba23:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6ba28:	74 2a                	je     6ba54 <verified_tiff::lerc_image::decode_tiles+0x6f4>
   6ba2a:	48 89 c1             	mov    %rax,%rcx
   6ba2d:	48 31 f9             	xor    %rdi,%rcx
   6ba30:	78 13                	js     6ba45 <verified_tiff::lerc_image::decode_tiles+0x6e5>
   6ba32:	48 89 c1             	mov    %rax,%rcx
   6ba35:	48 85 c0             	test   %rax,%rax
   6ba38:	78 4c                	js     6ba86 <verified_tiff::lerc_image::decode_tiles+0x726>
   6ba3a:	48 8b 14 24          	mov    (%rsp),%rdx
   6ba3e:	48 39 d0             	cmp    %rdx,%rax
   6ba41:	73 0e                	jae    6ba51 <verified_tiff::lerc_image::decode_tiles+0x6f1>
   6ba43:	eb 0f                	jmp    6ba54 <verified_tiff::lerc_image::decode_tiles+0x6f4>
   6ba45:	48 89 c1             	mov    %rax,%rcx
   6ba48:	48 85 c0             	test   %rax,%rax
   6ba4b:	48 8b 14 24          	mov    (%rsp),%rdx
   6ba4f:	78 03                	js     6ba54 <verified_tiff::lerc_image::decode_tiles+0x6f4>
   6ba51:	48 89 d1             	mov    %rdx,%rcx
   6ba54:	48 0f af 6c 24 78    	imul   0x78(%rsp),%rbp
   6ba5a:	48 03 6c 24 68       	add    0x68(%rsp),%rbp
   6ba5f:	48 3b ac 24 88 05 00 	cmp    0x588(%rsp),%rbp
   6ba66:	00 
   6ba67:	0f 83 bd 0c 00 00    	jae    6c72a <verified_tiff::lerc_image::decode_tiles+0x13ca>
   6ba6d:	48 8b 84 24 80 05 00 	mov    0x580(%rsp),%rax
   6ba74:	00 
   6ba75:	48 89 0c e8          	mov    %rcx,(%rax,%rbp,8)
   6ba79:	48 8b 54 24 40       	mov    0x40(%rsp),%rdx
   6ba7e:	48 ff c2             	inc    %rdx
   6ba81:	e9 6a fe ff ff       	jmp    6b8f0 <verified_tiff::lerc_image::decode_tiles+0x590>
   6ba86:	48 8b 14 24          	mov    (%rsp),%rdx
   6ba8a:	48 39 d0             	cmp    %rdx,%rax
   6ba8d:	76 c2                	jbe    6ba51 <verified_tiff::lerc_image::decode_tiles+0x6f1>
   6ba8f:	eb c3                	jmp    6ba54 <verified_tiff::lerc_image::decode_tiles+0x6f4>
   6ba91:	31 c9                	xor    %ecx,%ecx
   6ba93:	4c 89 4c 24 18       	mov    %r9,0x18(%rsp)
   6ba98:	e9 d5 04 00 00       	jmp    6bf72 <verified_tiff::lerc_image::decode_tiles+0xc12>
   6ba9d:	48 85 db             	test   %rbx,%rbx
   6baa0:	4d 89 d1             	mov    %r10,%r9
   6baa3:	4d 89 da             	mov    %r11,%r10
   6baa6:	4d 89 e3             	mov    %r12,%r11
   6baa9:	4c 8b 64 24 50       	mov    0x50(%rsp),%r12
   6baae:	0f 84 7c 03 00 00    	je     6be30 <verified_tiff::lerc_image::decode_tiles+0xad0>
   6bab4:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
   6bab9:	48 89 c1             	mov    %rax,%rcx
   6babc:	4c 21 c1             	and    %r8,%rcx
   6babf:	48 89 4c 24 40       	mov    %rcx,0x40(%rsp)
   6bac4:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   6bac9:	45 31 ed             	xor    %r13d,%r13d
   6bacc:	31 ed                	xor    %ebp,%ebp
   6bace:	45 31 f6             	xor    %r14d,%r14d
   6bad1:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6bad8:	0f 1f 84 00 00 00 00 
   6badf:	00 
   6bae0:	4c 39 6c 24 38       	cmp    %r13,0x38(%rsp)
   6bae5:	0f 84 50 0b 00 00    	je     6c63b <verified_tiff::lerc_image::decode_tiles+0x12db>
   6baeb:	49 81 fd 00 01 00 00 	cmp    $0x100,%r13
   6baf2:	0f 84 e1 0b 00 00    	je     6c6d9 <verified_tiff::lerc_image::decode_tiles+0x1379>
   6baf8:	42 8b b4 ac 28 01 00 	mov    0x128(%rsp,%r13,4),%esi
   6baff:	00 
   6bb00:	4c 39 84 24 98 00 00 	cmp    %r8,0x98(%rsp)
   6bb07:	00 
   6bb08:	75 26                	jne    6bb30 <verified_tiff::lerc_image::decode_tiles+0x7d0>
   6bb0a:	48 8b bc 24 88 00 00 	mov    0x88(%rsp),%rdi
   6bb11:	00 
   6bb12:	e8 89 e8 00 00       	call   7a3a0 <verified_tiff::lerc_numeric::scale_software>
   6bb17:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6bb1e:	00 f0 7f 
   6bb21:	4c 39 44 24 40       	cmp    %r8,0x40(%rsp)
   6bb26:	75 28                	jne    6bb50 <verified_tiff::lerc_image::decode_tiles+0x7f0>
   6bb28:	eb 46                	jmp    6bb70 <verified_tiff::lerc_image::decode_tiles+0x810>
   6bb2a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   6bb30:	48 01 f6             	add    %rsi,%rsi
   6bb33:	0f 57 c0             	xorps  %xmm0,%xmm0
   6bb36:	f2 48 0f 2a c6       	cvtsi2sd %rsi,%xmm0
   6bb3b:	f2 0f 59 84 24 80 00 	mulsd  0x80(%rsp),%xmm0
   6bb42:	00 00 
   6bb44:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6bb49:	4c 39 44 24 40       	cmp    %r8,0x40(%rsp)
   6bb4e:	74 20                	je     6bb70 <verified_tiff::lerc_image::decode_tiles+0x810>
   6bb50:	48 89 c1             	mov    %rax,%rcx
   6bb53:	4c 21 c1             	and    %r8,%rcx
   6bb56:	4c 39 c1             	cmp    %r8,%rcx
   6bb59:	74 15                	je     6bb70 <verified_tiff::lerc_image::decode_tiles+0x810>
   6bb5b:	66 48 0f 6e c0       	movq   %rax,%xmm0
   6bb60:	f2 0f 58 44 24 08    	addsd  0x8(%rsp),%xmm0
   6bb66:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6bb6b:	eb 1a                	jmp    6bb87 <verified_tiff::lerc_image::decode_tiles+0x827>
   6bb6d:	0f 1f 00             	nopl   (%rax)
   6bb70:	48 8b 7c 24 48       	mov    0x48(%rsp),%rdi
   6bb75:	48 89 c6             	mov    %rax,%rsi
   6bb78:	e8 83 e1 00 00       	call   79d00 <verified_tiff::lerc_numeric::plus_software>
   6bb7d:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6bb84:	00 f0 7f 
   6bb87:	48 8b 34 24          	mov    (%rsp),%rsi
   6bb8b:	48 89 c1             	mov    %rax,%rcx
   6bb8e:	48 09 f1             	or     %rsi,%rcx
   6bb91:	48 89 f7             	mov    %rsi,%rdi
   6bb94:	48 ba ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rdx
   6bb9b:	ff ff 7f 
   6bb9e:	48 85 d1             	test   %rdx,%rcx
   6bba1:	74 23                	je     6bbc6 <verified_tiff::lerc_image::decode_tiles+0x866>
   6bba3:	48 89 c1             	mov    %rax,%rcx
   6bba6:	48 89 c7             	mov    %rax,%rdi
   6bba9:	48 31 f1             	xor    %rsi,%rcx
   6bbac:	78 10                	js     6bbbe <verified_tiff::lerc_image::decode_tiles+0x85e>
   6bbae:	48 85 c0             	test   %rax,%rax
   6bbb1:	0f 88 c0 00 00 00    	js     6bc77 <verified_tiff::lerc_image::decode_tiles+0x917>
   6bbb7:	48 39 f0             	cmp    %rsi,%rax
   6bbba:	73 07                	jae    6bbc3 <verified_tiff::lerc_image::decode_tiles+0x863>
   6bbbc:	eb 08                	jmp    6bbc6 <verified_tiff::lerc_image::decode_tiles+0x866>
   6bbbe:	48 85 c0             	test   %rax,%rax
   6bbc1:	78 03                	js     6bbc6 <verified_tiff::lerc_image::decode_tiles+0x866>
   6bbc3:	48 89 f7             	mov    %rsi,%rdi
   6bbc6:	48 89 f8             	mov    %rdi,%rax
   6bbc9:	4c 21 c0             	and    %r8,%rax
   6bbcc:	4c 39 c0             	cmp    %r8,%rax
   6bbcf:	75 1f                	jne    6bbf0 <verified_tiff::lerc_image::decode_tiles+0x890>
   6bbd1:	e8 3a ea 00 00       	call   7a610 <verified_tiff::lerc_numeric::narrow_software>
   6bbd6:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6bbdd:	00 f0 7f 
   6bbe0:	eb 1b                	jmp    6bbfd <verified_tiff::lerc_image::decode_tiles+0x89d>
   6bbe2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6bbe9:	1f 84 00 00 00 00 00 
   6bbf0:	66 48 0f 6e c7       	movq   %rdi,%xmm0
   6bbf5:	f2 0f 5a c0          	cvtsd2ss %xmm0,%xmm0
   6bbf9:	66 0f 7e c0          	movd   %xmm0,%eax
   6bbfd:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6bc02:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6bc07:	4c 8b 5c 24 20       	mov    0x20(%rsp),%r11
   6bc0c:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6bc11:	48 8d 3c 2e          	lea    (%rsi,%rbp,1),%rdi
   6bc15:	48 8b 4c 24 60       	mov    0x60(%rsp),%rcx
   6bc1a:	4c 01 f1             	add    %r14,%rcx
   6bc1d:	49 0f af fb          	imul   %r11,%rdi
   6bc21:	48 01 cf             	add    %rcx,%rdi
   6bc24:	48 0f af 7c 24 78    	imul   0x78(%rsp),%rdi
   6bc2a:	48 03 7c 24 68       	add    0x68(%rsp),%rdi
   6bc2f:	48 3b bc 24 88 05 00 	cmp    0x588(%rsp),%rdi
   6bc36:	00 
   6bc37:	0f 83 87 0a 00 00    	jae    6c6c4 <verified_tiff::lerc_image::decode_tiles+0x1364>
   6bc3d:	89 c0                	mov    %eax,%eax
   6bc3f:	48 8b 8c 24 80 05 00 	mov    0x580(%rsp),%rcx
   6bc46:	00 
   6bc47:	48 89 04 f9          	mov    %rax,(%rcx,%rdi,8)
   6bc4b:	49 ff c5             	inc    %r13
   6bc4e:	4c 89 f0             	mov    %r14,%rax
   6bc51:	48 ff c0             	inc    %rax
   6bc54:	31 c9                	xor    %ecx,%ecx
   6bc56:	4c 39 e0             	cmp    %r12,%rax
   6bc59:	0f 94 c1             	sete   %cl
   6bc5c:	41 be 00 00 00 00    	mov    $0x0,%r14d
   6bc62:	4c 0f 45 f0          	cmovne %rax,%r14
   6bc66:	48 01 cd             	add    %rcx,%rbp
   6bc69:	4c 39 eb             	cmp    %r13,%rbx
   6bc6c:	0f 85 6e fe ff ff    	jne    6bae0 <verified_tiff::lerc_image::decode_tiles+0x780>
   6bc72:	e9 b9 01 00 00       	jmp    6be30 <verified_tiff::lerc_image::decode_tiles+0xad0>
   6bc77:	48 39 f0             	cmp    %rsi,%rax
   6bc7a:	0f 86 43 ff ff ff    	jbe    6bbc3 <verified_tiff::lerc_image::decode_tiles+0x863>
   6bc80:	e9 41 ff ff ff       	jmp    6bbc6 <verified_tiff::lerc_image::decode_tiles+0x866>
   6bc85:	48 85 db             	test   %rbx,%rbx
   6bc88:	4d 89 d1             	mov    %r10,%r9
   6bc8b:	4d 89 da             	mov    %r11,%r10
   6bc8e:	4d 89 e3             	mov    %r12,%r11
   6bc91:	4c 8b 64 24 50       	mov    0x50(%rsp),%r12
   6bc96:	0f 84 94 01 00 00    	je     6be30 <verified_tiff::lerc_image::decode_tiles+0xad0>
   6bc9c:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
   6bca1:	48 89 c1             	mov    %rax,%rcx
   6bca4:	4c 21 c1             	and    %r8,%rcx
   6bca7:	48 89 4c 24 40       	mov    %rcx,0x40(%rsp)
   6bcac:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   6bcb1:	45 31 ed             	xor    %r13d,%r13d
   6bcb4:	31 ed                	xor    %ebp,%ebp
   6bcb6:	45 31 f6             	xor    %r14d,%r14d
   6bcb9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   6bcc0:	4c 39 6c 24 38       	cmp    %r13,0x38(%rsp)
   6bcc5:	0f 84 70 09 00 00    	je     6c63b <verified_tiff::lerc_image::decode_tiles+0x12db>
   6bccb:	49 81 fd 00 01 00 00 	cmp    $0x100,%r13
   6bcd2:	0f 84 01 0a 00 00    	je     6c6d9 <verified_tiff::lerc_image::decode_tiles+0x1379>
   6bcd8:	42 8b b4 ac 28 01 00 	mov    0x128(%rsp,%r13,4),%esi
   6bcdf:	00 
   6bce0:	4c 39 84 24 98 00 00 	cmp    %r8,0x98(%rsp)
   6bce7:	00 
   6bce8:	75 26                	jne    6bd10 <verified_tiff::lerc_image::decode_tiles+0x9b0>
   6bcea:	48 8b bc 24 88 00 00 	mov    0x88(%rsp),%rdi
   6bcf1:	00 
   6bcf2:	e8 a9 e6 00 00       	call   7a3a0 <verified_tiff::lerc_numeric::scale_software>
   6bcf7:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6bcfe:	00 f0 7f 
   6bd01:	4c 39 44 24 40       	cmp    %r8,0x40(%rsp)
   6bd06:	75 28                	jne    6bd30 <verified_tiff::lerc_image::decode_tiles+0x9d0>
   6bd08:	eb 46                	jmp    6bd50 <verified_tiff::lerc_image::decode_tiles+0x9f0>
   6bd0a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   6bd10:	48 01 f6             	add    %rsi,%rsi
   6bd13:	0f 57 c0             	xorps  %xmm0,%xmm0
   6bd16:	f2 48 0f 2a c6       	cvtsi2sd %rsi,%xmm0
   6bd1b:	f2 0f 59 84 24 80 00 	mulsd  0x80(%rsp),%xmm0
   6bd22:	00 00 
   6bd24:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6bd29:	4c 39 44 24 40       	cmp    %r8,0x40(%rsp)
   6bd2e:	74 20                	je     6bd50 <verified_tiff::lerc_image::decode_tiles+0x9f0>
   6bd30:	48 89 c1             	mov    %rax,%rcx
   6bd33:	4c 21 c1             	and    %r8,%rcx
   6bd36:	4c 39 c1             	cmp    %r8,%rcx
   6bd39:	74 15                	je     6bd50 <verified_tiff::lerc_image::decode_tiles+0x9f0>
   6bd3b:	66 48 0f 6e c0       	movq   %rax,%xmm0
   6bd40:	f2 0f 58 44 24 08    	addsd  0x8(%rsp),%xmm0
   6bd46:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6bd4b:	eb 1a                	jmp    6bd67 <verified_tiff::lerc_image::decode_tiles+0xa07>
   6bd4d:	0f 1f 00             	nopl   (%rax)
   6bd50:	48 8b 7c 24 48       	mov    0x48(%rsp),%rdi
   6bd55:	48 89 c6             	mov    %rax,%rsi
   6bd58:	e8 a3 df 00 00       	call   79d00 <verified_tiff::lerc_numeric::plus_software>
   6bd5d:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6bd64:	00 f0 7f 
   6bd67:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6bd6c:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6bd71:	4c 8b 5c 24 20       	mov    0x20(%rsp),%r11
   6bd76:	48 8b 34 24          	mov    (%rsp),%rsi
   6bd7a:	48 89 c2             	mov    %rax,%rdx
   6bd7d:	48 09 f2             	or     %rsi,%rdx
   6bd80:	48 89 f1             	mov    %rsi,%rcx
   6bd83:	48 89 f7             	mov    %rsi,%rdi
   6bd86:	48 be ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rsi
   6bd8d:	ff ff 7f 
   6bd90:	48 85 f2             	test   %rsi,%rdx
   6bd93:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6bd98:	74 2a                	je     6bdc4 <verified_tiff::lerc_image::decode_tiles+0xa64>
   6bd9a:	48 89 c1             	mov    %rax,%rcx
   6bd9d:	48 31 f9             	xor    %rdi,%rcx
   6bda0:	78 13                	js     6bdb5 <verified_tiff::lerc_image::decode_tiles+0xa55>
   6bda2:	48 89 c1             	mov    %rax,%rcx
   6bda5:	48 85 c0             	test   %rax,%rax
   6bda8:	78 7b                	js     6be25 <verified_tiff::lerc_image::decode_tiles+0xac5>
   6bdaa:	48 8b 14 24          	mov    (%rsp),%rdx
   6bdae:	48 39 d0             	cmp    %rdx,%rax
   6bdb1:	73 0e                	jae    6bdc1 <verified_tiff::lerc_image::decode_tiles+0xa61>
   6bdb3:	eb 0f                	jmp    6bdc4 <verified_tiff::lerc_image::decode_tiles+0xa64>
   6bdb5:	48 89 c1             	mov    %rax,%rcx
   6bdb8:	48 85 c0             	test   %rax,%rax
   6bdbb:	48 8b 14 24          	mov    (%rsp),%rdx
   6bdbf:	78 03                	js     6bdc4 <verified_tiff::lerc_image::decode_tiles+0xa64>
   6bdc1:	48 89 d1             	mov    %rdx,%rcx
   6bdc4:	48 8d 3c 2e          	lea    (%rsi,%rbp,1),%rdi
   6bdc8:	48 8b 44 24 60       	mov    0x60(%rsp),%rax
   6bdcd:	4c 01 f0             	add    %r14,%rax
   6bdd0:	49 0f af fb          	imul   %r11,%rdi
   6bdd4:	48 01 c7             	add    %rax,%rdi
   6bdd7:	48 0f af 7c 24 78    	imul   0x78(%rsp),%rdi
   6bddd:	48 03 7c 24 68       	add    0x68(%rsp),%rdi
   6bde2:	48 3b bc 24 88 05 00 	cmp    0x588(%rsp),%rdi
   6bde9:	00 
   6bdea:	0f 83 d4 08 00 00    	jae    6c6c4 <verified_tiff::lerc_image::decode_tiles+0x1364>
   6bdf0:	48 8b 84 24 80 05 00 	mov    0x580(%rsp),%rax
   6bdf7:	00 
   6bdf8:	48 89 0c f8          	mov    %rcx,(%rax,%rdi,8)
   6bdfc:	49 ff c5             	inc    %r13
   6bdff:	4c 89 f0             	mov    %r14,%rax
   6be02:	48 ff c0             	inc    %rax
   6be05:	31 c9                	xor    %ecx,%ecx
   6be07:	4c 39 e0             	cmp    %r12,%rax
   6be0a:	0f 94 c1             	sete   %cl
   6be0d:	41 be 00 00 00 00    	mov    $0x0,%r14d
   6be13:	4c 0f 45 f0          	cmovne %rax,%r14
   6be17:	48 01 cd             	add    %rcx,%rbp
   6be1a:	4c 39 eb             	cmp    %r13,%rbx
   6be1d:	0f 85 9d fe ff ff    	jne    6bcc0 <verified_tiff::lerc_image::decode_tiles+0x960>
   6be23:	eb 0b                	jmp    6be30 <verified_tiff::lerc_image::decode_tiles+0xad0>
   6be25:	48 8b 14 24          	mov    (%rsp),%rdx
   6be29:	48 39 d0             	cmp    %rdx,%rax
   6be2c:	76 93                	jbe    6bdc1 <verified_tiff::lerc_image::decode_tiles+0xa61>
   6be2e:	eb 94                	jmp    6bdc4 <verified_tiff::lerc_image::decode_tiles+0xa64>
   6be30:	48 3b 5c 24 38       	cmp    0x38(%rsp),%rbx
   6be35:	48 8b 5c 24 18       	mov    0x18(%rsp),%rbx
   6be3a:	0f 84 f8 06 00 00    	je     6c538 <verified_tiff::lerc_image::decode_tiles+0x11d8>
   6be40:	e9 f6 07 00 00       	jmp    6c63b <verified_tiff::lerc_image::decode_tiles+0x12db>
   6be45:	31 d2                	xor    %edx,%edx
   6be47:	4d 89 da             	mov    %r11,%r10
   6be4a:	4d 89 e3             	mov    %r12,%r11
   6be4d:	4c 8b 64 24 50       	mov    0x50(%rsp),%r12
   6be52:	48 3b 54 24 38       	cmp    0x38(%rsp),%rdx
   6be57:	48 8b 5c 24 18       	mov    0x18(%rsp),%rbx
   6be5c:	0f 84 d6 06 00 00    	je     6c538 <verified_tiff::lerc_image::decode_tiles+0x11d8>
   6be62:	e9 d4 07 00 00       	jmp    6c63b <verified_tiff::lerc_image::decode_tiles+0x12db>
   6be67:	48 85 c0             	test   %rax,%rax
   6be6a:	74 24                	je     6be90 <verified_tiff::lerc_image::decode_tiles+0xb30>
   6be6c:	01 c0                	add    %eax,%eax
   6be6e:	bd 08 00 00 00       	mov    $0x8,%ebp
   6be73:	29 c5                	sub    %eax,%ebp
   6be75:	eb 29                	jmp    6bea0 <verified_tiff::lerc_image::decode_tiles+0xb40>
   6be77:	31 ed                	xor    %ebp,%ebp
   6be79:	48 83 f8 01          	cmp    $0x1,%rax
   6be7d:	40 0f 94 c5          	sete   %bpl
   6be81:	ff c5                	inc    %ebp
   6be83:	48 85 c0             	test   %rax,%rax
   6be86:	b8 06 00 00 00       	mov    $0x6,%eax
   6be8b:	0f 44 e8             	cmove  %eax,%ebp
   6be8e:	eb 10                	jmp    6bea0 <verified_tiff::lerc_image::decode_tiles+0xb40>
   6be90:	bd 07 00 00 00       	mov    $0x7,%ebp
   6be95:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   6be9c:	00 00 00 00 
   6bea0:	41 89 e8             	mov    %ebp,%r8d
   6bea3:	49 ff c1             	inc    %r9
   6bea6:	48 8d bc 24 e0 00 00 	lea    0xe0(%rsp),%rdi
   6bead:	00 
   6beae:	48 8b 74 24 70       	mov    0x70(%rsp),%rsi
   6beb3:	4d 89 f4             	mov    %r14,%r12
   6beb6:	4c 89 f2             	mov    %r14,%rdx
   6beb9:	4d 89 ce             	mov    %r9,%r14
   6bebc:	4c 89 c9             	mov    %r9,%rcx
   6bebf:	e8 ac 36 02 00       	call   8f570 <verified_tiff::lerc_samples::read>
   6bec4:	80 bc 24 e0 00 00 00 	cmpb   $0x1,0xe0(%rsp)
   6becb:	01 
   6becc:	0f 84 8e 07 00 00    	je     6c660 <verified_tiff::lerc_image::decode_tiles+0x1300>
   6bed2:	48 8b 84 24 e8 00 00 	mov    0xe8(%rsp),%rax
   6bed9:	00 
   6beda:	48 89 44 24 48       	mov    %rax,0x48(%rsp)
   6bedf:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6bee6:	00 f0 7f 
   6bee9:	4c 21 c0             	and    %r8,%rax
   6beec:	4c 39 c0             	cmp    %r8,%rax
   6beef:	0f 84 73 07 00 00    	je     6c668 <verified_tiff::lerc_image::decode_tiles+0x1308>
   6bef5:	b9 01 00 00 00       	mov    $0x1,%ecx
   6befa:	83 fd 02             	cmp    $0x2,%ebp
   6befd:	72 1a                	jb     6bf19 <verified_tiff::lerc_image::decode_tiles+0xbb9>
   6beff:	b9 02 00 00 00       	mov    $0x2,%ecx
   6bf04:	83 fd 04             	cmp    $0x4,%ebp
   6bf07:	72 10                	jb     6bf19 <verified_tiff::lerc_image::decode_tiles+0xbb9>
   6bf09:	31 c0                	xor    %eax,%eax
   6bf0b:	83 fd 07             	cmp    $0x7,%ebp
   6bf0e:	0f 93 c0             	setae  %al
   6bf11:	48 8d 0c 85 04 00 00 	lea    0x4(,%rax,4),%rcx
   6bf18:	00 
   6bf19:	4c 01 f1             	add    %r14,%rcx
   6bf1c:	83 7c 24 40 03       	cmpl   $0x3,0x40(%rsp)
   6bf21:	0f 85 6a 06 00 00    	jne    6c591 <verified_tiff::lerc_image::decode_tiles+0x1231>
   6bf27:	48 89 4c 24 18       	mov    %rcx,0x18(%rsp)
   6bf2c:	48 8b 7c 24 68       	mov    0x68(%rsp),%rdi
   6bf31:	48 83 ff 3f          	cmp    $0x3f,%rdi
   6bf35:	4c 8b 54 24 30       	mov    0x30(%rsp),%r10
   6bf3a:	4c 8b 5c 24 28       	mov    0x28(%rsp),%r11
   6bf3f:	4c 8b 64 24 20       	mov    0x20(%rsp),%r12
   6bf44:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6bf49:	48 8b ac 24 b0 00 00 	mov    0xb0(%rsp),%rbp
   6bf50:	00 
   6bf51:	0f 87 bf 06 00 00    	ja     6c616 <verified_tiff::lerc_image::decode_tiles+0x12b6>
   6bf57:	48 8b 84 24 70 05 00 	mov    0x570(%rsp),%rax
   6bf5e:	00 
   6bf5f:	48 8b 04 f8          	mov    (%rax,%rdi,8),%rax
   6bf63:	48 89 04 24          	mov    %rax,(%rsp)
   6bf67:	b1 01                	mov    $0x1,%cl
   6bf69:	48 c7 44 24 38 00 00 	movq   $0x0,0x38(%rsp)
   6bf70:	00 00 
   6bf72:	48 8b 5c 24 50       	mov    0x50(%rsp),%rbx
   6bf77:	48 0f af dd          	imul   %rbp,%rbx
   6bf7b:	48 85 db             	test   %rbx,%rbx
   6bf7e:	89 8c 24 c4 00 00 00 	mov    %ecx,0xc4(%rsp)
   6bf85:	0f 84 75 05 00 00    	je     6c500 <verified_tiff::lerc_image::decode_tiles+0x11a0>
   6bf8b:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
   6bf90:	48 89 c2             	mov    %rax,%rdx
   6bf93:	4c 21 c2             	and    %r8,%rdx
   6bf96:	48 89 94 24 18 01 00 	mov    %rdx,0x118(%rsp)
   6bf9d:	00 
   6bf9e:	48 89 84 24 08 01 00 	mov    %rax,0x108(%rsp)
   6bfa5:	00 
   6bfa6:	89 c8                	mov    %ecx,%eax
   6bfa8:	34 01                	xor    $0x1,%al
   6bfaa:	0f b6 c0             	movzbl %al,%eax
   6bfad:	48 89 84 24 90 00 00 	mov    %rax,0x90(%rsp)
   6bfb4:	00 
   6bfb5:	45 31 ed             	xor    %r13d,%r13d
   6bfb8:	45 31 f6             	xor    %r14d,%r14d
   6bfbb:	48 c7 44 24 08 00 00 	movq   $0x0,0x8(%rsp)
   6bfc2:	00 00 
   6bfc4:	4d 89 d1             	mov    %r10,%r9
   6bfc7:	4d 89 da             	mov    %r11,%r10
   6bfca:	4d 89 e3             	mov    %r12,%r11
   6bfcd:	4c 8b 64 24 50       	mov    0x50(%rsp),%r12
   6bfd2:	eb 30                	jmp    6c004 <verified_tiff::lerc_image::decode_tiles+0xca4>
   6bfd4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   6bfdb:	00 00 00 00 00 
   6bfe0:	4c 89 f0             	mov    %r14,%rax
   6bfe3:	48 ff c0             	inc    %rax
   6bfe6:	31 c9                	xor    %ecx,%ecx
   6bfe8:	4c 39 e0             	cmp    %r12,%rax
   6bfeb:	0f 94 c1             	sete   %cl
   6bfee:	41 be 00 00 00 00    	mov    $0x0,%r14d
   6bff4:	4c 0f 45 f0          	cmovne %rax,%r14
   6bff8:	49 01 cd             	add    %rcx,%r13
   6bffb:	48 ff cb             	dec    %rbx
   6bffe:	0f 84 13 05 00 00    	je     6c517 <verified_tiff::lerc_image::decode_tiles+0x11b7>
   6c004:	4a 8d 2c 2e          	lea    (%rsi,%r13,1),%rbp
   6c008:	48 8b 44 24 60       	mov    0x60(%rsp),%rax
   6c00d:	4c 01 f0             	add    %r14,%rax
   6c010:	49 0f af eb          	imul   %r11,%rbp
   6c014:	48 01 c5             	add    %rax,%rbp
   6c017:	45 84 ff             	test   %r15b,%r15b
   6c01a:	74 24                	je     6c040 <verified_tiff::lerc_image::decode_tiles+0xce0>
   6c01c:	48 89 ef             	mov    %rbp,%rdi
   6c01f:	48 c1 ef 03          	shr    $0x3,%rdi
   6c023:	4c 39 cf             	cmp    %r9,%rdi
   6c026:	0f 83 70 06 00 00    	jae    6c69c <verified_tiff::lerc_image::decode_tiles+0x133c>
   6c02c:	89 e8                	mov    %ebp,%eax
   6c02e:	f6 d0                	not    %al
   6c030:	41 0f b6 0c 3a       	movzbl (%r10,%rdi,1),%ecx
   6c035:	0f b6 c0             	movzbl %al,%eax
   6c038:	83 e0 07             	and    $0x7,%eax
   6c03b:	0f a3 c1             	bt     %eax,%ecx
   6c03e:	73 a0                	jae    6bfe0 <verified_tiff::lerc_image::decode_tiles+0xc80>
   6c040:	48 8d 0d c9 bf 0d 00 	lea    0xdbfc9(%rip),%rcx        # 148010 <anon.7e66189caf5c456237a9eb57058ec56c.27.llvm.3294417352400817999+0x15b0>
   6c047:	48 8b 44 24 40       	mov    0x40(%rsp),%rax
   6c04c:	48 63 04 81          	movslq (%rcx,%rax,4),%rax
   6c050:	48 01 c8             	add    %rcx,%rax
   6c053:	48 8b 7c 24 48       	mov    0x48(%rsp),%rdi
   6c058:	ff e0                	jmp    *%rax
   6c05a:	48 8b 4c 24 58       	mov    0x58(%rsp),%rcx
   6c05f:	48 8b 7c 24 18       	mov    0x18(%rsp),%rdi
   6c064:	48 29 f9             	sub    %rdi,%rcx
   6c067:	0f 93 c2             	setae  %dl
   6c06a:	48 39 8c 24 b8 00 00 	cmp    %rcx,0xb8(%rsp)
   6c071:	00 
   6c072:	0f 96 c0             	setbe  %al
   6c075:	20 d0                	and    %dl,%al
   6c077:	48 8b b4 24 20 01 00 	mov    0x120(%rsp),%rsi
   6c07e:	00 
   6c07f:	48 8d 56 fe          	lea    -0x2(%rsi),%rdx
   6c083:	48 83 fa 02          	cmp    $0x2,%rdx
   6c087:	0f 83 6a 01 00 00    	jae    6c1f7 <verified_tiff::lerc_image::decode_tiles+0xe97>
   6c08d:	84 c0                	test   %al,%al
   6c08f:	0f 84 aa 05 00 00    	je     6c63f <verified_tiff::lerc_image::decode_tiles+0x12df>
   6c095:	48 83 f9 04          	cmp    $0x4,%rcx
   6c099:	0f 82 a0 05 00 00    	jb     6c63f <verified_tiff::lerc_image::decode_tiles+0x12df>
   6c09f:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
   6c0a4:	48 89 f0             	mov    %rsi,%rax
   6c0a7:	48 29 f8             	sub    %rdi,%rax
   6c0aa:	0f 86 92 06 00 00    	jbe    6c742 <verified_tiff::lerc_image::decode_tiles+0x13e2>
   6c0b0:	48 ff c7             	inc    %rdi
   6c0b3:	48 39 f7             	cmp    %rsi,%rdi
   6c0b6:	0f 83 93 06 00 00    	jae    6c74f <verified_tiff::lerc_image::decode_tiles+0x13ef>
   6c0bc:	48 83 c0 fe          	add    $0xfffffffffffffffe,%rax
   6c0c0:	48 83 f8 01          	cmp    $0x1,%rax
   6c0c4:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6c0c9:	0f 86 70 05 00 00    	jbe    6c63f <verified_tiff::lerc_image::decode_tiles+0x12df>
   6c0cf:	48 8b 44 24 70       	mov    0x70(%rsp),%rax
   6c0d4:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
   6c0d9:	8b 3c 08             	mov    (%rax,%rcx,1),%edi
   6c0dc:	83 bc 24 b8 00 00 00 	cmpl   $0x4,0xb8(%rsp)
   6c0e3:	04 
   6c0e4:	0f 84 06 02 00 00    	je     6c2f0 <verified_tiff::lerc_image::decode_tiles+0xf90>
   6c0ea:	48 8d 41 04          	lea    0x4(%rcx),%rax
   6c0ee:	31 d2                	xor    %edx,%edx
   6c0f0:	48 8b 4c 24 58       	mov    0x58(%rsp),%rcx
   6c0f5:	48 29 c1             	sub    %rax,%rcx
   6c0f8:	0f 82 43 05 00 00    	jb     6c641 <verified_tiff::lerc_image::decode_tiles+0x12e1>
   6c0fe:	48 83 f9 04          	cmp    $0x4,%rcx
   6c102:	0f 82 39 05 00 00    	jb     6c641 <verified_tiff::lerc_image::decode_tiles+0x12e1>
   6c108:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
   6c10d:	48 39 c6             	cmp    %rax,%rsi
   6c110:	0f 86 5d 06 00 00    	jbe    6c773 <verified_tiff::lerc_image::decode_tiles+0x1413>
   6c116:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   6c11b:	48 8d 4a 05          	lea    0x5(%rdx),%rcx
   6c11f:	48 39 f1             	cmp    %rsi,%rcx
   6c122:	0f 83 5b 06 00 00    	jae    6c783 <verified_tiff::lerc_image::decode_tiles+0x1423>
   6c128:	48 83 c2 06          	add    $0x6,%rdx
   6c12c:	48 29 d6             	sub    %rdx,%rsi
   6c12f:	48 83 fe 01          	cmp    $0x1,%rsi
   6c133:	0f 86 06 05 00 00    	jbe    6c63f <verified_tiff::lerc_image::decode_tiles+0x12df>
   6c139:	4c 8b 44 24 70       	mov    0x70(%rsp),%r8
   6c13e:	41 0f b6 34 00       	movzbl (%r8,%rax,1),%esi
   6c143:	41 0f b6 0c 08       	movzbl (%r8,%rcx,1),%ecx
   6c148:	41 0f b6 14 10       	movzbl (%r8,%rdx,1),%edx
   6c14d:	41 0f b6 44 00 03    	movzbl 0x3(%r8,%rax,1),%eax
   6c153:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6c15a:	00 f0 7f 
   6c15d:	48 c1 e0 38          	shl    $0x38,%rax
   6c161:	48 c1 e2 30          	shl    $0x30,%rdx
   6c165:	48 c1 e1 28          	shl    $0x28,%rcx
   6c169:	48 c1 e6 20          	shl    $0x20,%rsi
   6c16d:	48 09 ce             	or     %rcx,%rsi
   6c170:	48 09 d6             	or     %rdx,%rsi
   6c173:	48 09 c6             	or     %rax,%rsi
   6c176:	48 09 f7             	or     %rsi,%rdi
   6c179:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6c17e:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
   6c183:	e9 68 01 00 00       	jmp    6c2f0 <verified_tiff::lerc_image::decode_tiles+0xf90>
   6c188:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   6c18f:	00 
   6c190:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   6c195:	48 3b 7c 24 38       	cmp    0x38(%rsp),%rdi
   6c19a:	0f 83 9b 04 00 00    	jae    6c63b <verified_tiff::lerc_image::decode_tiles+0x12db>
   6c1a0:	48 81 ff 00 01 00 00 	cmp    $0x100,%rdi
   6c1a7:	0f 83 43 05 00 00    	jae    6c6f0 <verified_tiff::lerc_image::decode_tiles+0x1390>
   6c1ad:	8b b4 bc 28 01 00 00 	mov    0x128(%rsp,%rdi,4),%esi
   6c1b4:	4c 39 84 24 98 00 00 	cmp    %r8,0x98(%rsp)
   6c1bb:	00 
   6c1bc:	75 6a                	jne    6c228 <verified_tiff::lerc_image::decode_tiles+0xec8>
   6c1be:	48 8b bc 24 88 00 00 	mov    0x88(%rsp),%rdi
   6c1c5:	00 
   6c1c6:	e8 d5 e1 00 00       	call   7a3a0 <verified_tiff::lerc_numeric::scale_software>
   6c1cb:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6c1d2:	00 f0 7f 
   6c1d5:	4c 39 84 24 18 01 00 	cmp    %r8,0x118(%rsp)
   6c1dc:	00 
   6c1dd:	75 6c                	jne    6c24b <verified_tiff::lerc_image::decode_tiles+0xeeb>
   6c1df:	e9 87 00 00 00       	jmp    6c26b <verified_tiff::lerc_image::decode_tiles+0xf0b>
   6c1e4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   6c1eb:	00 00 00 00 00 
   6c1f0:	31 ff                	xor    %edi,%edi
   6c1f2:	e9 75 02 00 00       	jmp    6c46c <verified_tiff::lerc_image::decode_tiles+0x110c>
   6c1f7:	48 83 fe 01          	cmp    $0x1,%rsi
   6c1fb:	0f 85 cb 00 00 00    	jne    6c2cc <verified_tiff::lerc_image::decode_tiles+0xf6c>
   6c201:	48 83 f9 02          	cmp    $0x2,%rcx
   6c205:	0f 92 c1             	setb   %cl
   6c208:	34 01                	xor    $0x1,%al
   6c20a:	08 c8                	or     %cl,%al
   6c20c:	0f 85 2d 04 00 00    	jne    6c63f <verified_tiff::lerc_image::decode_tiles+0x12df>
   6c212:	48 8b 44 24 70       	mov    0x70(%rsp),%rax
   6c217:	48 89 f9             	mov    %rdi,%rcx
   6c21a:	0f b7 3c 38          	movzwl (%rax,%rdi,1),%edi
   6c21e:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6c223:	e9 c8 00 00 00       	jmp    6c2f0 <verified_tiff::lerc_image::decode_tiles+0xf90>
   6c228:	48 01 f6             	add    %rsi,%rsi
   6c22b:	0f 57 c0             	xorps  %xmm0,%xmm0
   6c22e:	f2 48 0f 2a c6       	cvtsi2sd %rsi,%xmm0
   6c233:	f2 0f 59 84 24 80 00 	mulsd  0x80(%rsp),%xmm0
   6c23a:	00 00 
   6c23c:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6c241:	4c 39 84 24 18 01 00 	cmp    %r8,0x118(%rsp)
   6c248:	00 
   6c249:	74 20                	je     6c26b <verified_tiff::lerc_image::decode_tiles+0xf0b>
   6c24b:	48 89 c1             	mov    %rax,%rcx
   6c24e:	4c 21 c1             	and    %r8,%rcx
   6c251:	4c 39 c1             	cmp    %r8,%rcx
   6c254:	74 15                	je     6c26b <verified_tiff::lerc_image::decode_tiles+0xf0b>
   6c256:	66 48 0f 6e c0       	movq   %rax,%xmm0
   6c25b:	f2 0f 58 84 24 08 01 	addsd  0x108(%rsp),%xmm0
   6c262:	00 00 
   6c264:	66 48 0f 7e c0       	movq   %xmm0,%rax
   6c269:	eb 17                	jmp    6c282 <verified_tiff::lerc_image::decode_tiles+0xf22>
   6c26b:	48 8b 7c 24 48       	mov    0x48(%rsp),%rdi
   6c270:	48 89 c6             	mov    %rax,%rsi
   6c273:	e8 88 da 00 00       	call   79d00 <verified_tiff::lerc_numeric::plus_software>
   6c278:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6c27f:	00 f0 7f 
   6c282:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6c287:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6c28c:	4c 8b 5c 24 20       	mov    0x20(%rsp),%r11
   6c291:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6c296:	48 89 c1             	mov    %rax,%rcx
   6c299:	48 8b 3c 24          	mov    (%rsp),%rdi
   6c29d:	48 09 f9             	or     %rdi,%rcx
   6c2a0:	48 ba ff ff ff ff ff 	movabs $0x7fffffffffffffff,%rdx
   6c2a7:	ff ff 7f 
   6c2aa:	48 85 d1             	test   %rdx,%rcx
   6c2ad:	74 61                	je     6c310 <verified_tiff::lerc_image::decode_tiles+0xfb0>
   6c2af:	48 89 c1             	mov    %rax,%rcx
   6c2b2:	48 33 0c 24          	xor    (%rsp),%rcx
   6c2b6:	78 4a                	js     6c302 <verified_tiff::lerc_image::decode_tiles+0xfa2>
   6c2b8:	48 89 c7             	mov    %rax,%rdi
   6c2bb:	48 85 c0             	test   %rax,%rax
   6c2be:	0f 88 d2 01 00 00    	js     6c496 <verified_tiff::lerc_image::decode_tiles+0x1136>
   6c2c4:	48 3b 04 24          	cmp    (%rsp),%rax
   6c2c8:	73 40                	jae    6c30a <verified_tiff::lerc_image::decode_tiles+0xfaa>
   6c2ca:	eb 44                	jmp    6c310 <verified_tiff::lerc_image::decode_tiles+0xfb0>
   6c2cc:	84 c0                	test   %al,%al
   6c2ce:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6c2d3:	0f 84 66 03 00 00    	je     6c63f <verified_tiff::lerc_image::decode_tiles+0x12df>
   6c2d9:	48 39 7c 24 58       	cmp    %rdi,0x58(%rsp)
   6c2de:	0f 86 7d 04 00 00    	jbe    6c761 <verified_tiff::lerc_image::decode_tiles+0x1401>
   6c2e4:	48 8b 44 24 70       	mov    0x70(%rsp),%rax
   6c2e9:	48 89 f9             	mov    %rdi,%rcx
   6c2ec:	0f b6 3c 38          	movzbl (%rax,%rdi,1),%edi
   6c2f0:	48 03 8c 24 b8 00 00 	add    0xb8(%rsp),%rcx
   6c2f7:	00 
   6c2f8:	48 89 4c 24 18       	mov    %rcx,0x18(%rsp)
   6c2fd:	e9 6a 01 00 00       	jmp    6c46c <verified_tiff::lerc_image::decode_tiles+0x110c>
   6c302:	48 89 c7             	mov    %rax,%rdi
   6c305:	48 85 c0             	test   %rax,%rax
   6c308:	78 06                	js     6c310 <verified_tiff::lerc_image::decode_tiles+0xfb0>
   6c30a:	48 8b 3c 24          	mov    (%rsp),%rdi
   6c30e:	66 90                	xchg   %ax,%ax
   6c310:	b2 01                	mov    $0x1,%dl
   6c312:	48 83 bc 24 a8 00 00 	cmpq   $0x7,0xa8(%rsp)
   6c319:	00 07 
   6c31b:	0f 87 20 03 00 00    	ja     6c641 <verified_tiff::lerc_image::decode_tiles+0x12e1>
   6c321:	0f 84 33 01 00 00    	je     6c45a <verified_tiff::lerc_image::decode_tiles+0x10fa>
   6c327:	48 83 bc 24 a8 00 00 	cmpq   $0x6,0xa8(%rsp)
   6c32e:	00 06 
   6c330:	75 37                	jne    6c369 <verified_tiff::lerc_image::decode_tiles+0x1009>
   6c332:	48 89 f8             	mov    %rdi,%rax
   6c335:	4c 21 c0             	and    %r8,%rax
   6c338:	4c 39 c0             	cmp    %r8,%rax
   6c33b:	0f 85 ca 00 00 00    	jne    6c40b <verified_tiff::lerc_image::decode_tiles+0x10ab>
   6c341:	e8 ca e2 00 00       	call   7a610 <verified_tiff::lerc_numeric::narrow_software>
   6c346:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6c34b:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6c352:	00 f0 7f 
   6c355:	4c 8b 5c 24 20       	mov    0x20(%rsp),%r11
   6c35a:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6c35f:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6c364:	e9 af 00 00 00       	jmp    6c418 <verified_tiff::lerc_image::decode_tiles+0x10b8>
   6c369:	48 89 fe             	mov    %rdi,%rsi
   6c36c:	48 c1 ee 34          	shr    $0x34,%rsi
   6c370:	48 81 e6 ff 07 00 00 	and    $0x7ff,%rsi
   6c377:	0f 84 b0 00 00 00    	je     6c42d <verified_tiff::lerc_image::decode_tiles+0x10cd>
   6c37d:	48 81 fe 53 04 00 00 	cmp    $0x453,%rsi
   6c384:	0f 87 b7 02 00 00    	ja     6c641 <verified_tiff::lerc_image::decode_tiles+0x12e1>
   6c38a:	48 89 f8             	mov    %rdi,%rax
   6c38d:	48 b9 ff ff ff ff ff 	movabs $0xfffffffffffff,%rcx
   6c394:	ff 0f 00 
   6c397:	48 21 c8             	and    %rcx,%rax
   6c39a:	48 01 c8             	add    %rcx,%rax
   6c39d:	48 ff c0             	inc    %rax
   6c3a0:	81 fe 32 04 00 00    	cmp    $0x432,%esi
   6c3a6:	76 79                	jbe    6c421 <verified_tiff::lerc_image::decode_tiles+0x10c1>
   6c3a8:	8d 8e cd fb ff ff    	lea    -0x433(%rsi),%ecx
   6c3ae:	31 f6                	xor    %esi,%esi
   6c3b0:	48 0f a5 c6          	shld   %cl,%rax,%rsi
   6c3b4:	48 d3 e0             	shl    %cl,%rax
   6c3b7:	f6 c1 40             	test   $0x40,%cl
   6c3ba:	48 0f 45 f0          	cmovne %rax,%rsi
   6c3be:	b9 00 00 00 00       	mov    $0x0,%ecx
   6c3c3:	48 0f 45 c1          	cmovne %rcx,%rax
   6c3c7:	f6 84 24 a8 00 00 00 	testb  $0x1,0xa8(%rsp)
   6c3ce:	01 
   6c3cf:	74 6a                	je     6c43b <verified_tiff::lerc_image::decode_tiles+0x10db>
   6c3d1:	48 3b 84 24 c8 00 00 	cmp    0xc8(%rsp),%rax
   6c3d8:	00 
   6c3d9:	48 89 f1             	mov    %rsi,%rcx
   6c3dc:	48 83 d9 00          	sbb    $0x0,%rcx
   6c3e0:	0f 92 c1             	setb   %cl
   6c3e3:	45 31 c0             	xor    %r8d,%r8d
   6c3e6:	48 09 c6             	or     %rax,%rsi
   6c3e9:	41 0f 94 c0          	sete   %r8b
   6c3ed:	48 85 ff             	test   %rdi,%rdi
   6c3f0:	0f b6 c9             	movzbl %cl,%ecx
   6c3f3:	44 0f 49 c1          	cmovns %ecx,%r8d
   6c3f7:	45 84 c0             	test   %r8b,%r8b
   6c3fa:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6c401:	00 f0 7f 
   6c404:	75 4c                	jne    6c452 <verified_tiff::lerc_image::decode_tiles+0x10f2>
   6c406:	e9 36 02 00 00       	jmp    6c641 <verified_tiff::lerc_image::decode_tiles+0x12e1>
   6c40b:	66 48 0f 6e c7       	movq   %rdi,%xmm0
   6c410:	f2 0f 5a c0          	cvtsd2ss %xmm0,%xmm0
   6c414:	66 0f 7e c0          	movd   %xmm0,%eax
   6c418:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   6c41d:	89 c7                	mov    %eax,%edi
   6c41f:	eb 3e                	jmp    6c45f <verified_tiff::lerc_image::decode_tiles+0x10ff>
   6c421:	81 fe b4 03 00 00    	cmp    $0x3b4,%esi
   6c427:	0f 83 9f 00 00 00    	jae    6c4cc <verified_tiff::lerc_image::decode_tiles+0x116c>
   6c42d:	31 c0                	xor    %eax,%eax
   6c42f:	31 f6                	xor    %esi,%esi
   6c431:	f6 84 24 a8 00 00 00 	testb  $0x1,0xa8(%rsp)
   6c438:	01 
   6c439:	75 96                	jne    6c3d1 <verified_tiff::lerc_image::decode_tiles+0x1071>
   6c43b:	48 85 ff             	test   %rdi,%rdi
   6c43e:	78 65                	js     6c4a5 <verified_tiff::lerc_image::decode_tiles+0x1145>
   6c440:	48 3b 84 24 00 01 00 	cmp    0x100(%rsp),%rax
   6c447:	00 
   6c448:	48 83 de 00          	sbb    $0x0,%rsi
   6c44c:	0f 83 ef 01 00 00    	jae    6c641 <verified_tiff::lerc_image::decode_tiles+0x12e1>
   6c452:	48 89 c7             	mov    %rax,%rdi
   6c455:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6c45a:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   6c45f:	48 03 8c 24 90 00 00 	add    0x90(%rsp),%rcx
   6c466:	00 
   6c467:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
   6c46c:	48 0f af 6c 24 78    	imul   0x78(%rsp),%rbp
   6c472:	48 03 6c 24 68       	add    0x68(%rsp),%rbp
   6c477:	48 3b ac 24 88 05 00 	cmp    0x588(%rsp),%rbp
   6c47e:	00 
   6c47f:	0f 83 27 02 00 00    	jae    6c6ac <verified_tiff::lerc_image::decode_tiles+0x134c>
   6c485:	48 8b 84 24 80 05 00 	mov    0x580(%rsp),%rax
   6c48c:	00 
   6c48d:	48 89 3c e8          	mov    %rdi,(%rax,%rbp,8)
   6c491:	e9 4a fb ff ff       	jmp    6bfe0 <verified_tiff::lerc_image::decode_tiles+0xc80>
   6c496:	48 3b 04 24          	cmp    (%rsp),%rax
   6c49a:	0f 86 6a fe ff ff    	jbe    6c30a <verified_tiff::lerc_image::decode_tiles+0xfaa>
   6c4a0:	e9 6b fe ff ff       	jmp    6c310 <verified_tiff::lerc_image::decode_tiles+0xfb0>
   6c4a5:	48 3b 84 24 f8 00 00 	cmp    0xf8(%rsp),%rax
   6c4ac:	00 
   6c4ad:	48 89 f1             	mov    %rsi,%rcx
   6c4b0:	48 83 d9 00          	sbb    $0x0,%rcx
   6c4b4:	0f 83 87 01 00 00    	jae    6c641 <verified_tiff::lerc_image::decode_tiles+0x12e1>
   6c4ba:	48 09 c6             	or     %rax,%rsi
   6c4bd:	74 93                	je     6c452 <verified_tiff::lerc_image::decode_tiles+0x10f2>
   6c4bf:	48 8b bc 24 c8 00 00 	mov    0xc8(%rsp),%rdi
   6c4c6:	00 
   6c4c7:	48 29 c7             	sub    %rax,%rdi
   6c4ca:	eb 89                	jmp    6c455 <verified_tiff::lerc_image::decode_tiles+0x10f5>
   6c4cc:	b1 33                	mov    $0x33,%cl
   6c4ce:	40 28 f1             	sub    %sil,%cl
   6c4d1:	31 f6                	xor    %esi,%esi
   6c4d3:	48 0f ad f0          	shrd   %cl,%rsi,%rax
   6c4d7:	f6 c1 40             	test   $0x40,%cl
   6c4da:	48 0f 45 c6          	cmovne %rsi,%rax
   6c4de:	31 f6                	xor    %esi,%esi
   6c4e0:	f6 84 24 a8 00 00 00 	testb  $0x1,0xa8(%rsp)
   6c4e7:	01 
   6c4e8:	0f 85 e3 fe ff ff    	jne    6c3d1 <verified_tiff::lerc_image::decode_tiles+0x1071>
   6c4ee:	e9 48 ff ff ff       	jmp    6c43b <verified_tiff::lerc_image::decode_tiles+0x10db>
   6c4f3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6c4fa:	84 00 00 00 00 00 
   6c500:	48 c7 44 24 08 00 00 	movq   $0x0,0x8(%rsp)
   6c507:	00 00 
   6c509:	4d 89 d1             	mov    %r10,%r9
   6c50c:	4d 89 da             	mov    %r11,%r10
   6c50f:	4d 89 e3             	mov    %r12,%r11
   6c512:	4c 8b 64 24 50       	mov    0x50(%rsp),%r12
   6c517:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   6c51c:	48 3b 44 24 38       	cmp    0x38(%rsp),%rax
   6c521:	0f 94 c0             	sete   %al
   6c524:	8b 8c 24 c4 00 00 00 	mov    0xc4(%rsp),%ecx
   6c52b:	08 c1                	or     %al,%cl
   6c52d:	48 8b 5c 24 18       	mov    0x18(%rsp),%rbx
   6c532:	0f 84 03 01 00 00    	je     6c63b <verified_tiff::lerc_image::decode_tiles+0x12db>
   6c538:	48 8b 7c 24 68       	mov    0x68(%rsp),%rdi
   6c53d:	48 ff c7             	inc    %rdi
   6c540:	48 3b 7c 24 78       	cmp    0x78(%rsp),%rdi
   6c545:	72 27                	jb     6c56e <verified_tiff::lerc_image::decode_tiles+0x120e>
   6c547:	4c 03 64 24 60       	add    0x60(%rsp),%r12
   6c54c:	4d 39 dc             	cmp    %r11,%r12
   6c54f:	b8 00 00 00 00       	mov    $0x0,%eax
   6c554:	4c 0f 43 e0          	cmovae %rax,%r12
   6c558:	48 8b 8c 24 b0 00 00 	mov    0xb0(%rsp),%rcx
   6c55f:	00 
   6c560:	48 0f 42 c8          	cmovb  %rax,%rcx
   6c564:	48 01 ce             	add    %rcx,%rsi
   6c567:	4c 89 64 24 60       	mov    %r12,0x60(%rsp)
   6c56c:	31 ff                	xor    %edi,%edi
   6c56e:	4c 8b 74 24 58       	mov    0x58(%rsp),%r14
   6c573:	4c 8b ac 24 d0 00 00 	mov    0xd0(%rsp),%r13
   6c57a:	00 
   6c57b:	4c 8b a4 24 d8 00 00 	mov    0xd8(%rsp),%r12
   6c582:	00 
   6c583:	4c 39 ee             	cmp    %r13,%rsi
   6c586:	0f 85 87 ef ff ff    	jne    6b513 <verified_tiff::lerc_image::decode_tiles+0x1b3>
   6c58c:	e9 9a 00 00 00       	jmp    6c62b <verified_tiff::lerc_image::decode_tiles+0x12cb>
   6c591:	48 8d bc 24 e0 00 00 	lea    0xe0(%rsp),%rdi
   6c598:	00 
   6c599:	48 8b 74 24 70       	mov    0x70(%rsp),%rsi
   6c59e:	4c 89 e2             	mov    %r12,%rdx
   6c5a1:	4c 8d 84 24 28 01 00 	lea    0x128(%rsp),%r8
   6c5a8:	00 
   6c5a9:	e8 42 1f 00 00       	call   6e4f0 <verified_tiff::lerc_bits::decode_into>
   6c5ae:	80 bc 24 e0 00 00 00 	cmpb   $0x1,0xe0(%rsp)
   6c5b5:	01 
   6c5b6:	48 8b 7c 24 68       	mov    0x68(%rsp),%rdi
   6c5bb:	48 8b ac 24 b0 00 00 	mov    0xb0(%rsp),%rbp
   6c5c2:	00 
   6c5c3:	0f 84 97 00 00 00    	je     6c660 <verified_tiff::lerc_image::decode_tiles+0x1300>
   6c5c9:	4c 8b 8c 24 e8 00 00 	mov    0xe8(%rsp),%r9
   6c5d0:	00 
   6c5d1:	48 8b 8c 24 f0 00 00 	mov    0xf0(%rsp),%rcx
   6c5d8:	00 
   6c5d9:	4c 89 8c 24 28 05 00 	mov    %r9,0x528(%rsp)
   6c5e0:	00 
   6c5e1:	48 89 4c 24 38       	mov    %rcx,0x38(%rsp)
   6c5e6:	48 89 8c 24 30 05 00 	mov    %rcx,0x530(%rsp)
   6c5ed:	00 
   6c5ee:	4c 8b 54 24 30       	mov    0x30(%rsp),%r10
   6c5f3:	4c 8b 5c 24 28       	mov    0x28(%rsp),%r11
   6c5f8:	4c 8b 64 24 20       	mov    0x20(%rsp),%r12
   6c5fd:	49 b8 00 00 00 00 00 	movabs $0x7ff0000000000000,%r8
   6c604:	00 f0 7f 
   6c607:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   6c60c:	48 83 ff 40          	cmp    $0x40,%rdi
   6c610:	0f 82 cd ef ff ff    	jb     6b5e3 <verified_tiff::lerc_image::decode_tiles+0x283>
   6c616:	48 8d 15 73 40 12 00 	lea    0x124073(%rip),%rdx        # 190690 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x218>
   6c61d:	be 40 00 00 00       	mov    $0x40,%esi
   6c622:	ff 15 98 d0 12 00    	call   *0x12d098(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c628:	48 89 eb             	mov    %rbp,%rbx
   6c62b:	48 8b 84 24 a0 00 00 	mov    0xa0(%rsp),%rax
   6c632:	00 
   6c633:	48 89 58 08          	mov    %rbx,0x8(%rax)
   6c637:	31 c9                	xor    %ecx,%ecx
   6c639:	eb 3a                	jmp    6c675 <verified_tiff::lerc_image::decode_tiles+0x1315>
   6c63b:	b2 01                	mov    $0x1,%dl
   6c63d:	eb 02                	jmp    6c641 <verified_tiff::lerc_image::decode_tiles+0x12e1>
   6c63f:	31 d2                	xor    %edx,%edx
   6c641:	48 8b 84 24 a0 00 00 	mov    0xa0(%rsp),%rax
   6c648:	00 
   6c649:	88 50 01             	mov    %dl,0x1(%rax)
   6c64c:	eb 25                	jmp    6c673 <verified_tiff::lerc_image::decode_tiles+0x1313>
   6c64e:	48 8b 84 24 a0 00 00 	mov    0xa0(%rsp),%rax
   6c655:	00 
   6c656:	c6 40 01 01          	movb   $0x1,0x1(%rax)
   6c65a:	eb 17                	jmp    6c673 <verified_tiff::lerc_image::decode_tiles+0x1313>
   6c65c:	31 db                	xor    %ebx,%ebx
   6c65e:	eb 08                	jmp    6c668 <verified_tiff::lerc_image::decode_tiles+0x1308>
   6c660:	0f b6 9c 24 e1 00 00 	movzbl 0xe1(%rsp),%ebx
   6c667:	00 
   6c668:	48 8b 84 24 a0 00 00 	mov    0xa0(%rsp),%rax
   6c66f:	00 
   6c670:	88 58 01             	mov    %bl,0x1(%rax)
   6c673:	b1 01                	mov    $0x1,%cl
   6c675:	88 08                	mov    %cl,(%rax)
   6c677:	48 81 c4 38 05 00 00 	add    $0x538,%rsp
   6c67e:	5b                   	pop    %rbx
   6c67f:	41 5c                	pop    %r12
   6c681:	41 5d                	pop    %r13
   6c683:	41 5e                	pop    %r14
   6c685:	41 5f                	pop    %r15
   6c687:	5d                   	pop    %rbp
   6c688:	c3                   	ret
   6c689:	48 8d 15 e8 3f 12 00 	lea    0x123fe8(%rip),%rdx        # 190678 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x200>
   6c690:	4c 89 cf             	mov    %r9,%rdi
   6c693:	4c 89 ce             	mov    %r9,%rsi
   6c696:	ff 15 24 d0 12 00    	call   *0x12d024(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c69c:	48 8d 15 fd 41 12 00 	lea    0x1241fd(%rip),%rdx        # 1908a0 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569>
   6c6a3:	4c 89 ce             	mov    %r9,%rsi
   6c6a6:	ff 15 14 d0 12 00    	call   *0x12d014(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c6ac:	48 8d 15 65 3f 12 00 	lea    0x123f65(%rip),%rdx        # 190618 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x1a0>
   6c6b3:	48 89 ef             	mov    %rbp,%rdi
   6c6b6:	48 8b b4 24 88 05 00 	mov    0x588(%rsp),%rsi
   6c6bd:	00 
   6c6be:	ff 15 fc cf 12 00    	call   *0x12cffc(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c6c4:	48 8d 15 7d 3f 12 00 	lea    0x123f7d(%rip),%rdx        # 190648 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x1d0>
   6c6cb:	48 8b b4 24 88 05 00 	mov    0x588(%rsp),%rsi
   6c6d2:	00 
   6c6d3:	ff 15 e7 cf 12 00    	call   *0x12cfe7(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c6d9:	48 8d 15 50 3f 12 00 	lea    0x123f50(%rip),%rdx        # 190630 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x1b8>
   6c6e0:	bf 00 01 00 00       	mov    $0x100,%edi
   6c6e5:	be 00 01 00 00       	mov    $0x100,%esi
   6c6ea:	ff 15 d0 cf 12 00    	call   *0x12cfd0(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c6f0:	48 8d 15 09 3f 12 00 	lea    0x123f09(%rip),%rdx        # 190600 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x188>
   6c6f7:	be 00 01 00 00       	mov    $0x100,%esi
   6c6fc:	ff 15 be cf 12 00    	call   *0x12cfbe(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c702:	48 8d 15 57 3f 12 00 	lea    0x123f57(%rip),%rdx        # 190660 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x1e8>
   6c709:	48 89 df             	mov    %rbx,%rdi
   6c70c:	4c 89 f6             	mov    %r14,%rsi
   6c70f:	ff 15 ab cf 12 00    	call   *0x12cfab(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c715:	48 89 d7             	mov    %rdx,%rdi
   6c718:	48 8d 15 11 3f 12 00 	lea    0x123f11(%rip),%rdx        # 190630 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x1b8>
   6c71f:	be 00 01 00 00       	mov    $0x100,%esi
   6c724:	ff 15 96 cf 12 00    	call   *0x12cf96(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c72a:	48 8d 15 17 3f 12 00 	lea    0x123f17(%rip),%rdx        # 190648 <anon.e33870e3aff4ecfcb9b6e2c3e3b380eb.5.llvm.7528104188100526393+0x1d0>
   6c731:	48 89 ef             	mov    %rbp,%rdi
   6c734:	48 8b b4 24 88 05 00 	mov    0x588(%rsp),%rsi
   6c73b:	00 
   6c73c:	ff 15 7e cf 12 00    	call   *0x12cf7e(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c742:	48 8d 15 4f 38 12 00 	lea    0x12384f(%rip),%rdx        # 18ff98 <anon.7e66189caf5c456237a9eb57058ec56c.20.llvm.3294417352400817999>
   6c749:	ff 15 71 cf 12 00    	call   *0x12cf71(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c74f:	48 8d 15 5a 38 12 00 	lea    0x12385a(%rip),%rdx        # 18ffb0 <anon.7e66189caf5c456237a9eb57058ec56c.21.llvm.3294417352400817999>
   6c756:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
   6c75b:	ff 15 5f cf 12 00    	call   *0x12cf5f(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c761:	48 8d 15 90 40 12 00 	lea    0x124090(%rip),%rdx        # 1907f8 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.23.llvm.8789668757449324569+0xd8>
   6c768:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
   6c76d:	ff 15 4d cf 12 00    	call   *0x12cf4d(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c773:	48 8d 15 1e 38 12 00 	lea    0x12381e(%rip),%rdx        # 18ff98 <anon.7e66189caf5c456237a9eb57058ec56c.20.llvm.3294417352400817999>
   6c77a:	48 89 c7             	mov    %rax,%rdi
   6c77d:	ff 15 3d cf 12 00    	call   *0x12cf3d(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6c783:	48 8d 15 26 38 12 00 	lea    0x123826(%rip),%rdx        # 18ffb0 <anon.7e66189caf5c456237a9eb57058ec56c.21.llvm.3294417352400817999>
   6c78a:	48 89 cf             	mov    %rcx,%rdi
   6c78d:	ff 15 2d cf 12 00    	call   *0x12cf2d(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
