
target/release/examples/benchmark:     file format elf64-x86-64


Disassembly of section .text:

000000000002a3e0 <<verified_tiff::deflate::Cursor>::step>:
   2a3e0:	55                   	push   %rbp
   2a3e1:	41 57                	push   %r15
   2a3e3:	41 56                	push   %r14
   2a3e5:	41 55                	push   %r13
   2a3e7:	41 54                	push   %r12
   2a3e9:	53                   	push   %rbx
   2a3ea:	48 81 ec 00 10 00 00 	sub    $0x1000,%rsp
   2a3f1:	48 c7 04 24 00 00 00 	movq   $0x0,(%rsp)
   2a3f8:	00 
   2a3f9:	48 81 ec 68 0a 00 00 	sub    $0xa68,%rsp
   2a400:	0f b6 87 08 89 00 00 	movzbl 0x8908(%rdi),%eax
   2a407:	48 83 f8 03          	cmp    $0x3,%rax
   2a40b:	0f 87 7b 03 00 00    	ja     2a78c <<verified_tiff::deflate::Cursor>::step+0x3ac>
   2a411:	4c 8d 05 08 fd fd ff 	lea    -0x202f8(%rip),%r8        # a120 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x76d>
   2a418:	49 63 04 80          	movslq (%r8,%rax,4),%rax
   2a41c:	4c 01 c0             	add    %r8,%rax
   2a41f:	ff e0                	jmp    *%rax
   2a421:	48 8b 8f c0 88 00 00 	mov    0x88c0(%rdi),%rcx
   2a428:	4c 8b 8f d0 88 00 00 	mov    0x88d0(%rdi),%r9
   2a42f:	66 bd 02 00          	mov    $0x2,%bp
   2a433:	4c 39 c9             	cmp    %r9,%rcx
   2a436:	0f 84 ec 0a 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a43c:	48 39 d1             	cmp    %rdx,%rcx
   2a43f:	0f 83 aa 0b 00 00    	jae    2afef <<verified_tiff::deflate::Cursor>::step+0xc0f>
   2a445:	48 8d 9f c0 88 00 00 	lea    0x88c0(%rdi),%rbx
   2a44c:	4c 8b 87 c8 88 00 00 	mov    0x88c8(%rdi),%r8
   2a453:	49 83 f8 07          	cmp    $0x7,%r8
   2a457:	41 bd 07 00 00 00    	mov    $0x7,%r13d
   2a45d:	4d 0f 42 e8          	cmovb  %r8,%r13
   2a461:	0f 85 e0 03 00 00    	jne    2a847 <<verified_tiff::deflate::Cursor>::step+0x467>
   2a467:	48 8d 41 01          	lea    0x1(%rcx),%rax
   2a46b:	48 89 03             	mov    %rax,(%rbx)
   2a46e:	45 31 c0             	xor    %r8d,%r8d
   2a471:	44 0f b6 34 0e       	movzbl (%rsi,%rcx,1),%r14d
   2a476:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   2a47d:	4c 39 c8             	cmp    %r9,%rax
   2a480:	0f 84 a2 0a 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a486:	e9 d7 03 00 00       	jmp    2a862 <<verified_tiff::deflate::Cursor>::step+0x482>
   2a48b:	48 8d 9f c0 88 00 00 	lea    0x88c0(%rdi),%rbx
   2a492:	4c 8b 8f d0 88 00 00 	mov    0x88d0(%rdi),%r9
   2a499:	48 8b 87 c0 88 00 00 	mov    0x88c0(%rdi),%rax
   2a4a0:	4c 8b b7 c8 88 00 00 	mov    0x88c8(%rdi),%r14
   2a4a7:	49 c7 c2 f1 ff ff ff 	mov    $0xfffffffffffffff1,%r10
   2a4ae:	45 31 db             	xor    %r11d,%r11d
   2a4b1:	45 31 ff             	xor    %r15d,%r15d
   2a4b4:	eb 1c                	jmp    2a4d2 <<verified_tiff::deflate::Cursor>::step+0xf2>
   2a4b6:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   2a4bd:	00 00 00 
   2a4c0:	4d 01 fc             	add    %r15,%r12
   2a4c3:	4d 01 e4             	add    %r12,%r12
   2a4c6:	4d 89 e7             	mov    %r12,%r15
   2a4c9:	49 ff c2             	inc    %r10
   2a4cc:	0f 84 6d 03 00 00    	je     2a83f <<verified_tiff::deflate::Cursor>::step+0x45f>
   2a4d2:	66 bd 02 00          	mov    $0x2,%bp
   2a4d6:	4c 39 c8             	cmp    %r9,%rax
   2a4d9:	0f 84 49 0a 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a4df:	48 39 d0             	cmp    %rdx,%rax
   2a4e2:	0f 83 de 0a 00 00    	jae    2afc6 <<verified_tiff::deflate::Cursor>::step+0xbe6>
   2a4e8:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   2a4ed:	49 83 fe 07          	cmp    $0x7,%r14
   2a4f1:	41 bd 07 00 00 00    	mov    $0x7,%r13d
   2a4f7:	4d 0f 42 ee          	cmovb  %r14,%r13
   2a4fb:	45 31 c0             	xor    %r8d,%r8d
   2a4fe:	4d 0f a3 ec          	bt     %r13,%r12
   2a502:	41 0f 92 c4          	setb   %r12b
   2a506:	49 83 fe 07          	cmp    $0x7,%r14
   2a50a:	74 14                	je     2a520 <<verified_tiff::deflate::Cursor>::step+0x140>
   2a50c:	49 ff c6             	inc    %r14
   2a50f:	eb 18                	jmp    2a529 <<verified_tiff::deflate::Cursor>::step+0x149>
   2a511:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2a518:	0f 1f 84 00 00 00 00 
   2a51f:	00 
   2a520:	48 ff c0             	inc    %rax
   2a523:	48 89 03             	mov    %rax,(%rbx)
   2a526:	45 31 f6             	xor    %r14d,%r14d
   2a529:	45 88 e0             	mov    %r12b,%r8b
   2a52c:	4c 89 b7 c8 88 00 00 	mov    %r14,0x88c8(%rdi)
   2a533:	4f 8d 1c 58          	lea    (%r8,%r11,2),%r11
   2a537:	4e 8b a4 d7 80 80 00 	mov    0x8080(%rdi,%r10,8),%r12
   2a53e:	00 
   2a53f:	4d 89 d8             	mov    %r11,%r8
   2a542:	4d 29 f8             	sub    %r15,%r8
   2a545:	0f 82 75 ff ff ff    	jb     2a4c0 <<verified_tiff::deflate::Cursor>::step+0xe0>
   2a54b:	4d 39 e0             	cmp    %r12,%r8
   2a54e:	0f 83 6c ff ff ff    	jae    2a4c0 <<verified_tiff::deflate::Cursor>::step+0xe0>
   2a554:	4e 03 84 d7 00 81 00 	add    0x8100(%rdi,%r10,8),%r8
   2a55b:	00 
   2a55c:	49 81 f8 20 01 00 00 	cmp    $0x120,%r8
   2a563:	0f 83 9c 0a 00 00    	jae    2b005 <<verified_tiff::deflate::Cursor>::step+0xc25>
   2a569:	46 0f b7 8c 47 00 81 	movzwl 0x8100(%rdi,%r8,2),%r9d
   2a570:	00 00 
   2a572:	49 81 f9 00 01 00 00 	cmp    $0x100,%r9
   2a579:	0f 83 3c 03 00 00    	jae    2a8bb <<verified_tiff::deflate::Cursor>::step+0x4db>
   2a57f:	48 8b b7 e8 88 00 00 	mov    0x88e8(%rdi),%rsi
   2a586:	41 b4 01             	mov    $0x1,%r12b
   2a589:	48 39 ce             	cmp    %rcx,%rsi
   2a58c:	0f 84 99 09 00 00    	je     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a592:	89 f0                	mov    %esi,%eax
   2a594:	25 ff 7f 00 00       	and    $0x7fff,%eax
   2a599:	44 88 0c 07          	mov    %r9b,(%rdi,%rax,1)
   2a59d:	4c 89 c9             	mov    %r9,%rcx
   2a5a0:	48 03 8f f8 88 00 00 	add    0x88f8(%rdi),%rcx
   2a5a7:	49 b8 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r8
   2a5ae:	00 0f 00 
   2a5b1:	48 89 c8             	mov    %rcx,%rax
   2a5b4:	49 f7 e0             	mul    %r8
   2a5b7:	48 89 c8             	mov    %rcx,%rax
   2a5ba:	48 29 d0             	sub    %rdx,%rax
   2a5bd:	48 d1 e8             	shr    $1,%rax
   2a5c0:	48 01 d0             	add    %rdx,%rax
   2a5c3:	48 c1 e8 0f          	shr    $0xf,%rax
   2a5c7:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2a5ce:	48 29 c1             	sub    %rax,%rcx
   2a5d1:	48 89 8f f8 88 00 00 	mov    %rcx,0x88f8(%rdi)
   2a5d8:	48 03 8f 00 89 00 00 	add    0x8900(%rdi),%rcx
   2a5df:	48 89 c8             	mov    %rcx,%rax
   2a5e2:	49 f7 e0             	mul    %r8
   2a5e5:	48 89 c8             	mov    %rcx,%rax
   2a5e8:	48 29 d0             	sub    %rdx,%rax
   2a5eb:	48 d1 e8             	shr    $1,%rax
   2a5ee:	48 01 d0             	add    %rdx,%rax
   2a5f1:	48 c1 e8 0f          	shr    $0xf,%rax
   2a5f5:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2a5fc:	48 29 c1             	sub    %rax,%rcx
   2a5ff:	48 89 8f 00 89 00 00 	mov    %rcx,0x8900(%rdi)
   2a606:	48 ff c6             	inc    %rsi
   2a609:	48 89 b7 e8 88 00 00 	mov    %rsi,0x88e8(%rdi)
   2a610:	66 bd 01 00          	mov    $0x1,%bp
   2a614:	45 89 cc             	mov    %r9d,%r12d
   2a617:	e9 0f 09 00 00       	jmp    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a61c:	48 8b 87 d8 88 00 00 	mov    0x88d8(%rdi),%rax
   2a623:	48 85 c0             	test   %rax,%rax
   2a626:	0f 84 07 02 00 00    	je     2a833 <<verified_tiff::deflate::Cursor>::step+0x453>
   2a62c:	48 8b 97 e0 88 00 00 	mov    0x88e0(%rdi),%rdx
   2a633:	66 bd 02 00          	mov    $0x2,%bp
   2a637:	41 b4 01             	mov    $0x1,%r12b
   2a63a:	48 85 d2             	test   %rdx,%rdx
   2a63d:	0f 84 e8 08 00 00    	je     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a643:	48 8b b7 e8 88 00 00 	mov    0x88e8(%rdi),%rsi
   2a64a:	48 39 d6             	cmp    %rdx,%rsi
   2a64d:	0f 82 d8 08 00 00    	jb     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a653:	48 3b 97 f0 88 00 00 	cmp    0x88f0(%rdi),%rdx
   2a65a:	0f 87 cb 08 00 00    	ja     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a660:	41 89 f0             	mov    %esi,%r8d
   2a663:	41 29 d0             	sub    %edx,%r8d
   2a666:	41 81 e0 ff 7f 00 00 	and    $0x7fff,%r8d
   2a66d:	46 0f b6 04 07       	movzbl (%rdi,%r8,1),%r8d
   2a672:	48 ff c8             	dec    %rax
   2a675:	48 89 87 d8 88 00 00 	mov    %rax,0x88d8(%rdi)
   2a67c:	48 39 ce             	cmp    %rcx,%rsi
   2a67f:	0f 84 a6 08 00 00    	je     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a685:	41 0f b6 c0          	movzbl %r8b,%eax
   2a689:	89 f1                	mov    %esi,%ecx
   2a68b:	81 e1 ff 7f 00 00    	and    $0x7fff,%ecx
   2a691:	88 04 0f             	mov    %al,(%rdi,%rcx,1)
   2a694:	48 8b 8f f8 88 00 00 	mov    0x88f8(%rdi),%rcx
   2a69b:	48 01 c1             	add    %rax,%rcx
   2a69e:	49 b9 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r9
   2a6a5:	00 0f 00 
   2a6a8:	48 89 c8             	mov    %rcx,%rax
   2a6ab:	49 f7 e1             	mul    %r9
   2a6ae:	48 89 c8             	mov    %rcx,%rax
   2a6b1:	48 29 d0             	sub    %rdx,%rax
   2a6b4:	48 d1 e8             	shr    $1,%rax
   2a6b7:	48 01 d0             	add    %rdx,%rax
   2a6ba:	48 c1 e8 0f          	shr    $0xf,%rax
   2a6be:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2a6c5:	48 29 c1             	sub    %rax,%rcx
   2a6c8:	48 89 8f f8 88 00 00 	mov    %rcx,0x88f8(%rdi)
   2a6cf:	48 03 8f 00 89 00 00 	add    0x8900(%rdi),%rcx
   2a6d6:	48 89 c8             	mov    %rcx,%rax
   2a6d9:	49 f7 e1             	mul    %r9
   2a6dc:	48 89 c8             	mov    %rcx,%rax
   2a6df:	48 29 d0             	sub    %rdx,%rax
   2a6e2:	48 d1 e8             	shr    $1,%rax
   2a6e5:	48 01 d0             	add    %rdx,%rax
   2a6e8:	48 c1 e8 0f          	shr    $0xf,%rax
   2a6ec:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2a6f3:	48 29 c1             	sub    %rax,%rcx
   2a6f6:	48 89 8f 00 89 00 00 	mov    %rcx,0x8900(%rdi)
   2a6fd:	48 ff c6             	inc    %rsi
   2a700:	48 89 b7 e8 88 00 00 	mov    %rsi,0x88e8(%rdi)
   2a707:	66 bd 01 00          	mov    $0x1,%bp
   2a70b:	45 89 c4             	mov    %r8d,%r12d
   2a70e:	e9 18 08 00 00       	jmp    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a713:	4c 8b 87 d8 88 00 00 	mov    0x88d8(%rdi),%r8
   2a71a:	4d 85 c0             	test   %r8,%r8
   2a71d:	0f 84 a5 01 00 00    	je     2a8c8 <<verified_tiff::deflate::Cursor>::step+0x4e8>
   2a723:	48 8b 87 c0 88 00 00 	mov    0x88c0(%rdi),%rax
   2a72a:	4c 8b 8f c8 88 00 00 	mov    0x88c8(%rdi),%r9
   2a731:	4c 8b 9f d0 88 00 00 	mov    0x88d0(%rdi),%r11
   2a738:	4d 85 c9             	test   %r9,%r9
   2a73b:	0f 84 65 01 00 00    	je     2a8a6 <<verified_tiff::deflate::Cursor>::step+0x4c6>
   2a741:	66 bd 02 00          	mov    $0x2,%bp
   2a745:	4c 39 d8             	cmp    %r11,%rax
   2a748:	0f 84 da 07 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a74e:	48 39 d0             	cmp    %rdx,%rax
   2a751:	0f 83 85 08 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2a757:	49 83 f9 07          	cmp    $0x7,%r9
   2a75b:	41 ba 07 00 00 00    	mov    $0x7,%r10d
   2a761:	4d 0f 42 d1          	cmovb  %r9,%r10
   2a765:	0f b6 1c 06          	movzbl (%rsi,%rax,1),%ebx
   2a769:	44 0f a3 d3          	bt     %r10d,%ebx
   2a76d:	0f 92 c3             	setb   %bl
   2a770:	49 83 f9 07          	cmp    $0x7,%r9
   2a774:	0f 85 0f 02 00 00    	jne    2a989 <<verified_tiff::deflate::Cursor>::step+0x5a9>
   2a77a:	48 ff c0             	inc    %rax
   2a77d:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2a784:	45 31 c9             	xor    %r9d,%r9d
   2a787:	e9 00 02 00 00       	jmp    2a98c <<verified_tiff::deflate::Cursor>::step+0x5ac>
   2a78c:	48 8b 8f c0 88 00 00 	mov    0x88c0(%rdi),%rcx
   2a793:	48 8b 87 d0 88 00 00 	mov    0x88d0(%rdi),%rax
   2a79a:	49 89 c0             	mov    %rax,%r8
   2a79d:	66 bd 02 00          	mov    $0x2,%bp
   2a7a1:	41 b4 01             	mov    $0x1,%r12b
   2a7a4:	49 29 c8             	sub    %rcx,%r8
   2a7a7:	74 0a                	je     2a7b3 <<verified_tiff::deflate::Cursor>::step+0x3d3>
   2a7a9:	49 83 f8 01          	cmp    $0x1,%r8
   2a7ad:	0f 85 78 07 00 00    	jne    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a7b3:	48 39 c8             	cmp    %rcx,%rax
   2a7b6:	74 0e                	je     2a7c6 <<verified_tiff::deflate::Cursor>::step+0x3e6>
   2a7b8:	48 83 bf c8 88 00 00 	cmpq   $0x0,0x88c8(%rdi)
   2a7bf:	00 
   2a7c0:	0f 84 65 07 00 00    	je     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a7c6:	48 89 d1             	mov    %rdx,%rcx
   2a7c9:	48 29 c1             	sub    %rax,%rcx
   2a7cc:	41 0f 92 c0          	setb   %r8b
   2a7d0:	48 83 f9 04          	cmp    $0x4,%rcx
   2a7d4:	0f 92 c1             	setb   %cl
   2a7d7:	44 08 c1             	or     %r8b,%cl
   2a7da:	0f 85 48 07 00 00    	jne    2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a7e0:	49 89 d0             	mov    %rdx,%r8
   2a7e3:	49 29 c0             	sub    %rax,%r8
   2a7e6:	0f 86 2e 08 00 00    	jbe    2b01a <<verified_tiff::deflate::Cursor>::step+0xc3a>
   2a7ec:	48 8d 48 01          	lea    0x1(%rax),%rcx
   2a7f0:	48 39 d1             	cmp    %rdx,%rcx
   2a7f3:	0f 83 37 08 00 00    	jae    2b030 <<verified_tiff::deflate::Cursor>::step+0xc50>
   2a7f9:	49 83 c0 fe          	add    $0xfffffffffffffffe,%r8
   2a7fd:	49 83 f8 01          	cmp    $0x1,%r8
   2a801:	0f 86 21 07 00 00    	jbe    2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a807:	8b 04 06             	mov    (%rsi,%rax,1),%eax
   2a80a:	0f c8                	bswap  %eax
   2a80c:	48 8b 8f 00 89 00 00 	mov    0x8900(%rdi),%rcx
   2a813:	48 c1 e1 10          	shl    $0x10,%rcx
   2a817:	48 03 8f f8 88 00 00 	add    0x88f8(%rdi),%rcx
   2a81e:	48 39 c8             	cmp    %rcx,%rax
   2a821:	0f 85 04 07 00 00    	jne    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a827:	c6 87 08 89 00 00 05 	movb   $0x5,0x8908(%rdi)
   2a82e:	e9 a5 00 00 00       	jmp    2a8d8 <<verified_tiff::deflate::Cursor>::step+0x4f8>
   2a833:	c6 87 08 89 00 00 02 	movb   $0x2,0x8908(%rdi)
   2a83a:	e9 99 00 00 00       	jmp    2a8d8 <<verified_tiff::deflate::Cursor>::step+0x4f8>
   2a83f:	41 b4 01             	mov    $0x1,%r12b
   2a842:	e9 e4 06 00 00       	jmp    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a847:	49 ff c0             	inc    %r8
   2a84a:	48 89 c8             	mov    %rcx,%rax
   2a84d:	44 0f b6 34 0e       	movzbl (%rsi,%rcx,1),%r14d
   2a852:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   2a859:	4c 39 c8             	cmp    %r9,%rax
   2a85c:	0f 84 c6 06 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a862:	48 39 d0             	cmp    %rdx,%rax
   2a865:	0f 83 71 07 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2a86b:	49 83 f8 07          	cmp    $0x7,%r8
   2a86f:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a874:	49 0f 42 c8          	cmovb  %r8,%rcx
   2a878:	44 0f b6 14 06       	movzbl (%rsi,%rax,1),%r10d
   2a87d:	41 0f a3 ca          	bt     %ecx,%r10d
   2a881:	41 0f 92 c2          	setb   %r10b
   2a885:	49 83 f8 07          	cmp    $0x7,%r8
   2a889:	75 54                	jne    2a8df <<verified_tiff::deflate::Cursor>::step+0x4ff>
   2a88b:	48 ff c0             	inc    %rax
   2a88e:	48 89 03             	mov    %rax,(%rbx)
   2a891:	45 31 c0             	xor    %r8d,%r8d
   2a894:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   2a89b:	4c 39 c8             	cmp    %r9,%rax
   2a89e:	0f 84 84 06 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a8a4:	eb 4c                	jmp    2a8f2 <<verified_tiff::deflate::Cursor>::step+0x512>
   2a8a6:	4c 39 d8             	cmp    %r11,%rax
   2a8a9:	0f 85 9c 00 00 00    	jne    2a94b <<verified_tiff::deflate::Cursor>::step+0x56b>
   2a8af:	66 bd 02 00          	mov    $0x2,%bp
   2a8b3:	45 31 e4             	xor    %r12d,%r12d
   2a8b6:	e9 70 06 00 00       	jmp    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a8bb:	41 81 f9 00 01 00 00 	cmp    $0x100,%r9d
   2a8c2:	0f 85 11 01 00 00    	jne    2a9d9 <<verified_tiff::deflate::Cursor>::step+0x5f9>
   2a8c8:	0f b6 87 09 89 00 00 	movzbl 0x8909(%rdi),%eax
   2a8cf:	c0 e0 02             	shl    $0x2,%al
   2a8d2:	88 87 08 89 00 00    	mov    %al,0x8908(%rdi)
   2a8d8:	31 ed                	xor    %ebp,%ebp
   2a8da:	e9 4c 06 00 00       	jmp    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a8df:	49 ff c0             	inc    %r8
   2a8e2:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   2a8e9:	4c 39 c8             	cmp    %r9,%rax
   2a8ec:	0f 84 36 06 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a8f2:	48 39 d0             	cmp    %rdx,%rax
   2a8f5:	0f 83 e1 06 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2a8fb:	44 0f b6 0c 06       	movzbl (%rsi,%rax,1),%r9d
   2a900:	49 83 f8 07          	cmp    $0x7,%r8
   2a904:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a909:	49 0f 42 c8          	cmovb  %r8,%rcx
   2a90d:	41 d2 e9             	shr    %cl,%r9b
   2a910:	45 00 c9             	add    %r9b,%r9b
   2a913:	41 80 e1 02          	and    $0x2,%r9b
   2a917:	45 08 d1             	or     %r10b,%r9b
   2a91a:	49 83 f8 07          	cmp    $0x7,%r8
   2a91e:	0f 85 fc 01 00 00    	jne    2ab20 <<verified_tiff::deflate::Cursor>::step+0x740>
   2a924:	48 ff c0             	inc    %rax
   2a927:	48 89 03             	mov    %rax,(%rbx)
   2a92a:	45 31 c0             	xor    %r8d,%r8d
   2a92d:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   2a934:	41 b4 01             	mov    $0x1,%r12b
   2a937:	41 0f b6 c9          	movzbl %r9b,%ecx
   2a93b:	4c 8d 0d ee f7 fd ff 	lea    -0x20812(%rip),%r9        # a130 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x77d>
   2a942:	49 63 0c 89          	movslq (%r9,%rcx,4),%rcx
   2a946:	4c 01 c9             	add    %r9,%rcx
   2a949:	ff e1                	jmp    *%rcx
   2a94b:	48 39 d0             	cmp    %rdx,%rax
   2a94e:	0f 83 f2 06 00 00    	jae    2b046 <<verified_tiff::deflate::Cursor>::step+0xc66>
   2a954:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   2a959:	48 ff c0             	inc    %rax
   2a95c:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2a963:	49 ff c8             	dec    %r8
   2a966:	4c 89 87 d8 88 00 00 	mov    %r8,0x88d8(%rdi)
   2a96d:	48 8b b7 e8 88 00 00 	mov    0x88e8(%rdi),%rsi
   2a974:	48 39 ce             	cmp    %rcx,%rsi
   2a977:	0f 85 68 02 00 00    	jne    2abe5 <<verified_tiff::deflate::Cursor>::step+0x805>
   2a97d:	66 bd 02 00          	mov    $0x2,%bp
   2a981:	41 b4 01             	mov    $0x1,%r12b
   2a984:	e9 a2 05 00 00       	jmp    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a989:	49 ff c1             	inc    %r9
   2a98c:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2a993:	4c 39 d8             	cmp    %r11,%rax
   2a996:	0f 84 8c 05 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a99c:	48 39 d0             	cmp    %rdx,%rax
   2a99f:	0f 83 37 06 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2a9a5:	49 89 ca             	mov    %rcx,%r10
   2a9a8:	44 0f b6 34 06       	movzbl (%rsi,%rax,1),%r14d
   2a9ad:	49 83 f9 07          	cmp    $0x7,%r9
   2a9b1:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a9b6:	49 0f 42 c9          	cmovb  %r9,%rcx
   2a9ba:	41 d2 ee             	shr    %cl,%r14b
   2a9bd:	49 83 f9 07          	cmp    $0x7,%r9
   2a9c1:	0f 85 a9 02 00 00    	jne    2ac70 <<verified_tiff::deflate::Cursor>::step+0x890>
   2a9c7:	48 ff c0             	inc    %rax
   2a9ca:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2a9d1:	45 31 c9             	xor    %r9d,%r9d
   2a9d4:	e9 9a 02 00 00       	jmp    2ac73 <<verified_tiff::deflate::Cursor>::step+0x893>
   2a9d9:	41 b4 01             	mov    $0x1,%r12b
   2a9dc:	41 81 f9 1d 01 00 00 	cmp    $0x11d,%r9d
   2a9e3:	0f 87 42 05 00 00    	ja     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2a9e9:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
   2a9ee:	48 89 3c 24          	mov    %rdi,(%rsp)
   2a9f2:	48 8d 05 47 f7 fd ff 	lea    -0x208b9(%rip),%rax        # a140 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x78d>
   2a9f9:	4d 8d 81 fb fe ff ff 	lea    -0x105(%r9),%r8
   2aa00:	49 c1 e8 02          	shr    $0x2,%r8
   2aa04:	31 c9                	xor    %ecx,%ecx
   2aa06:	49 81 f9 1d 01 00 00 	cmp    $0x11d,%r9
   2aa0d:	4c 0f 44 c1          	cmove  %rcx,%r8
   2aa11:	4e 8b ac c8 f8 f7 ff 	mov    -0x808(%rax,%r9,8),%r13
   2aa18:	ff 
   2aa19:	41 81 f9 09 01 00 00 	cmp    $0x109,%r9d
   2aa20:	4c 0f 42 c1          	cmovb  %rcx,%r8
   2aa24:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   2aa29:	48 89 f0             	mov    %rsi,%rax
   2aa2c:	48 89 de             	mov    %rbx,%rsi
   2aa2f:	48 89 d1             	mov    %rdx,%rcx
   2aa32:	49 89 c7             	mov    %rax,%r15
   2aa35:	48 89 c2             	mov    %rax,%rdx
   2aa38:	49 89 ce             	mov    %rcx,%r14
   2aa3b:	e8 30 f5 ff ff       	call   29f70 <<verified_tiff::deflate_bits::Bits>::read>
   2aa40:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   2aa45:	0f 84 00 03 00 00    	je     2ad4b <<verified_tiff::deflate::Cursor>::step+0x96b>
   2aa4b:	4c 03 6c 24 20       	add    0x20(%rsp),%r13
   2aa50:	48 8b 04 24          	mov    (%rsp),%rax
   2aa54:	48 8d b0 60 84 00 00 	lea    0x8460(%rax),%rsi
   2aa5b:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   2aa60:	4c 89 fa             	mov    %r15,%rdx
   2aa63:	4c 89 f1             	mov    %r14,%rcx
   2aa66:	49 89 d8             	mov    %rbx,%r8
   2aa69:	e8 92 0f 00 00       	call   2ba00 <<verified_tiff::deflate_bits::Huff>::read>
   2aa6e:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   2aa73:	0f 84 d2 02 00 00    	je     2ad4b <<verified_tiff::deflate::Cursor>::step+0x96b>
   2aa79:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   2aa7e:	48 83 f8 1d          	cmp    $0x1d,%rax
   2aa82:	0f 87 a3 04 00 00    	ja     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2aa88:	48 8d 0d 99 f7 fd ff 	lea    -0x20867(%rip),%rcx        # a228 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x875>
   2aa8f:	48 8b 0c c1          	mov    (%rcx,%rax,8),%rcx
   2aa93:	48 89 4c 24 10       	mov    %rcx,0x10(%rsp)
   2aa98:	48 89 c1             	mov    %rax,%rcx
   2aa9b:	48 d1 e9             	shr    $1,%rcx
   2aa9e:	48 ff c9             	dec    %rcx
   2aaa1:	45 31 c0             	xor    %r8d,%r8d
   2aaa4:	48 83 f8 04          	cmp    $0x4,%rax
   2aaa8:	4c 0f 43 c1          	cmovae %rcx,%r8
   2aaac:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   2aab1:	48 89 de             	mov    %rbx,%rsi
   2aab4:	4c 89 fa             	mov    %r15,%rdx
   2aab7:	4c 89 f1             	mov    %r14,%rcx
   2aaba:	e8 b1 f4 ff ff       	call   29f70 <<verified_tiff::deflate_bits::Bits>::read>
   2aabf:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   2aac4:	0f 84 81 02 00 00    	je     2ad4b <<verified_tiff::deflate::Cursor>::step+0x96b>
   2aaca:	48 8b 04 24          	mov    (%rsp),%rax
   2aace:	48 8b 88 e8 88 00 00 	mov    0x88e8(%rax),%rcx
   2aad5:	48 8b 54 24 08       	mov    0x8(%rsp),%rdx
   2aada:	48 29 ca             	sub    %rcx,%rdx
   2aadd:	49 39 d5             	cmp    %rdx,%r13
   2aae0:	0f 87 45 04 00 00    	ja     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2aae6:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
   2aaeb:	48 03 54 24 20       	add    0x20(%rsp),%rdx
   2aaf0:	48 39 ca             	cmp    %rcx,%rdx
   2aaf3:	0f 87 32 04 00 00    	ja     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2aaf9:	48 3b 90 f0 88 00 00 	cmp    0x88f0(%rax),%rdx
   2ab00:	0f 87 25 04 00 00    	ja     2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2ab06:	48 89 90 e0 88 00 00 	mov    %rdx,0x88e0(%rax)
   2ab0d:	4c 89 a8 d8 88 00 00 	mov    %r13,0x88d8(%rax)
   2ab14:	c6 80 08 89 00 00 03 	movb   $0x3,0x8908(%rax)
   2ab1b:	e9 b8 fd ff ff       	jmp    2a8d8 <<verified_tiff::deflate::Cursor>::step+0x4f8>
   2ab20:	49 ff c0             	inc    %r8
   2ab23:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   2ab2a:	41 b4 01             	mov    $0x1,%r12b
   2ab2d:	41 0f b6 c9          	movzbl %r9b,%ecx
   2ab31:	4c 8d 0d f8 f5 fd ff 	lea    -0x20a08(%rip),%r9        # a130 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x77d>
   2ab38:	49 63 0c 89          	movslq (%r9,%rcx,4),%rcx
   2ab3c:	4c 01 c9             	add    %r9,%rcx
   2ab3f:	ff e1                	jmp    *%rcx
   2ab41:	49 89 f7             	mov    %rsi,%r15
   2ab44:	49 89 d4             	mov    %rdx,%r12
   2ab47:	4d 85 c0             	test   %r8,%r8
   2ab4a:	74 15                	je     2ab61 <<verified_tiff::deflate::Cursor>::step+0x781>
   2ab4c:	48 ff c0             	inc    %rax
   2ab4f:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2ab56:	48 c7 87 c8 88 00 00 	movq   $0x0,0x88c8(%rdi)
   2ab5d:	00 00 00 00 
   2ab61:	48 89 3c 24          	mov    %rdi,(%rsp)
   2ab65:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   2ab6a:	41 b8 10 00 00 00    	mov    $0x10,%r8d
   2ab70:	48 89 de             	mov    %rbx,%rsi
   2ab73:	4c 89 fa             	mov    %r15,%rdx
   2ab76:	4c 89 e1             	mov    %r12,%rcx
   2ab79:	e8 f2 f3 ff ff       	call   29f70 <<verified_tiff::deflate_bits::Bits>::read>
   2ab7e:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   2ab83:	0f 84 c2 01 00 00    	je     2ad4b <<verified_tiff::deflate::Cursor>::step+0x96b>
   2ab89:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   2ab8e:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   2ab93:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   2ab98:	41 b8 10 00 00 00    	mov    $0x10,%r8d
   2ab9e:	48 89 de             	mov    %rbx,%rsi
   2aba1:	4c 89 fa             	mov    %r15,%rdx
   2aba4:	4c 89 e1             	mov    %r12,%rcx
   2aba7:	e8 c4 f3 ff ff       	call   29f70 <<verified_tiff::deflate_bits::Bits>::read>
   2abac:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   2abb1:	0f 84 94 01 00 00    	je     2ad4b <<verified_tiff::deflate::Cursor>::step+0x96b>
   2abb7:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   2abbc:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2abc1:	48 01 c8             	add    %rcx,%rax
   2abc4:	41 b4 01             	mov    $0x1,%r12b
   2abc7:	48 3d ff ff 00 00    	cmp    $0xffff,%rax
   2abcd:	4c 8b 3c 24          	mov    (%rsp),%r15
   2abd1:	0f 85 54 03 00 00    	jne    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2abd7:	49 89 8f d8 88 00 00 	mov    %rcx,0x88d8(%r15)
   2abde:	b1 01                	mov    $0x1,%cl
   2abe0:	e9 34 02 00 00       	jmp    2ae19 <<verified_tiff::deflate::Cursor>::step+0xa39>
   2abe5:	41 0f b6 c4          	movzbl %r12b,%eax
   2abe9:	89 f1                	mov    %esi,%ecx
   2abeb:	81 e1 ff 7f 00 00    	and    $0x7fff,%ecx
   2abf1:	88 04 0f             	mov    %al,(%rdi,%rcx,1)
   2abf4:	48 8b 8f f8 88 00 00 	mov    0x88f8(%rdi),%rcx
   2abfb:	48 01 c1             	add    %rax,%rcx
   2abfe:	49 b8 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r8
   2ac05:	00 0f 00 
   2ac08:	48 89 c8             	mov    %rcx,%rax
   2ac0b:	49 f7 e0             	mul    %r8
   2ac0e:	48 89 c8             	mov    %rcx,%rax
   2ac11:	48 29 d0             	sub    %rdx,%rax
   2ac14:	48 d1 e8             	shr    $1,%rax
   2ac17:	48 01 d0             	add    %rdx,%rax
   2ac1a:	48 c1 e8 0f          	shr    $0xf,%rax
   2ac1e:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2ac25:	48 29 c1             	sub    %rax,%rcx
   2ac28:	48 89 8f f8 88 00 00 	mov    %rcx,0x88f8(%rdi)
   2ac2f:	48 03 8f 00 89 00 00 	add    0x8900(%rdi),%rcx
   2ac36:	48 89 c8             	mov    %rcx,%rax
   2ac39:	49 f7 e0             	mul    %r8
   2ac3c:	48 89 c8             	mov    %rcx,%rax
   2ac3f:	48 29 d0             	sub    %rdx,%rax
   2ac42:	48 d1 e8             	shr    $1,%rax
   2ac45:	48 01 d0             	add    %rdx,%rax
   2ac48:	48 c1 e8 0f          	shr    $0xf,%rax
   2ac4c:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2ac53:	48 29 c1             	sub    %rax,%rcx
   2ac56:	48 89 8f 00 89 00 00 	mov    %rcx,0x8900(%rdi)
   2ac5d:	48 ff c6             	inc    %rsi
   2ac60:	48 89 b7 e8 88 00 00 	mov    %rsi,0x88e8(%rdi)
   2ac67:	66 bd 01 00          	mov    $0x1,%bp
   2ac6b:	e9 bb 02 00 00       	jmp    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2ac70:	49 ff c1             	inc    %r9
   2ac73:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2ac7a:	4c 39 d8             	cmp    %r11,%rax
   2ac7d:	0f 84 a5 02 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2ac83:	48 39 d0             	cmp    %rdx,%rax
   2ac86:	0f 83 50 03 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2ac8c:	44 0f b6 3c 06       	movzbl (%rsi,%rax,1),%r15d
   2ac91:	49 83 f9 07          	cmp    $0x7,%r9
   2ac95:	b9 07 00 00 00       	mov    $0x7,%ecx
   2ac9a:	49 0f 42 c9          	cmovb  %r9,%rcx
   2ac9e:	41 d2 ef             	shr    %cl,%r15b
   2aca1:	49 83 f9 07          	cmp    $0x7,%r9
   2aca5:	0f 85 ab 00 00 00    	jne    2ad56 <<verified_tiff::deflate::Cursor>::step+0x976>
   2acab:	48 ff c0             	inc    %rax
   2acae:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2acb5:	45 31 c9             	xor    %r9d,%r9d
   2acb8:	e9 9c 00 00 00       	jmp    2ad59 <<verified_tiff::deflate::Cursor>::step+0x979>
   2acbd:	49 89 ff             	mov    %rdi,%r15
   2acc0:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   2acc5:	31 f6                	xor    %esi,%esi
   2acc7:	e8 44 cf 00 00       	call   37c10 <verified_tiff::deflate_header::fixed>
   2accc:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   2acd1:	74 78                	je     2ad4b <<verified_tiff::deflate::Cursor>::step+0x96b>
   2acd3:	48 8d 74 24 20       	lea    0x20(%rsp),%rsi
   2acd8:	48 8d 9c 24 e8 08 00 	lea    0x8e8(%rsp),%rbx
   2acdf:	00 
   2ace0:	4c 8b 25 19 f9 05 00 	mov    0x5f919(%rip),%r12        # 8a600 <memcpy@GLIBC_2.14>
   2ace7:	ba 60 04 00 00       	mov    $0x460,%edx
   2acec:	48 89 df             	mov    %rbx,%rdi
   2acef:	41 ff d4             	call   *%r12
   2acf2:	49 8d bf 00 80 00 00 	lea    0x8000(%r15),%rdi
   2acf9:	ba 60 04 00 00       	mov    $0x460,%edx
   2acfe:	48 89 de             	mov    %rbx,%rsi
   2ad01:	41 ff d4             	call   *%r12
   2ad04:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   2ad09:	be 01 00 00 00       	mov    $0x1,%esi
   2ad0e:	e8 fd ce 00 00       	call   37c10 <verified_tiff::deflate_header::fixed>
   2ad13:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   2ad18:	74 31                	je     2ad4b <<verified_tiff::deflate::Cursor>::step+0x96b>
   2ad1a:	48 8d 74 24 20       	lea    0x20(%rsp),%rsi
   2ad1f:	48 8d 9c 24 e8 08 00 	lea    0x8e8(%rsp),%rbx
   2ad26:	00 
   2ad27:	ba 60 04 00 00       	mov    $0x460,%edx
   2ad2c:	48 89 df             	mov    %rbx,%rdi
   2ad2f:	e9 ce 00 00 00       	jmp    2ae02 <<verified_tiff::deflate::Cursor>::step+0xa22>
   2ad34:	49 89 ff             	mov    %rdi,%r15
   2ad37:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   2ad3c:	48 89 d9             	mov    %rbx,%rcx
   2ad3f:	e8 8c d1 00 00       	call   37ed0 <verified_tiff::deflate_header::dynamic>
   2ad44:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   2ad49:	75 58                	jne    2ada3 <<verified_tiff::deflate::Cursor>::step+0x9c3>
   2ad4b:	44 0f b6 64 24 19    	movzbl 0x19(%rsp),%r12d
   2ad51:	e9 d5 01 00 00       	jmp    2af2b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2ad56:	49 ff c1             	inc    %r9
   2ad59:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2ad60:	4c 39 d8             	cmp    %r11,%rax
   2ad63:	0f 84 bf 01 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2ad69:	48 39 d0             	cmp    %rdx,%rax
   2ad6c:	0f 83 6a 02 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2ad72:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   2ad77:	49 83 f9 07          	cmp    $0x7,%r9
   2ad7b:	b9 07 00 00 00       	mov    $0x7,%ecx
   2ad80:	49 0f 42 c9          	cmovb  %r9,%rcx
   2ad84:	41 d2 ec             	shr    %cl,%r12b
   2ad87:	49 83 f9 07          	cmp    $0x7,%r9
   2ad8b:	0f 85 a7 00 00 00    	jne    2ae38 <<verified_tiff::deflate::Cursor>::step+0xa58>
   2ad91:	48 ff c0             	inc    %rax
   2ad94:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2ad9b:	45 31 c9             	xor    %r9d,%r9d
   2ad9e:	e9 98 00 00 00       	jmp    2ae3b <<verified_tiff::deflate::Cursor>::step+0xa5b>
   2ada3:	48 8d 74 24 20       	lea    0x20(%rsp),%rsi
   2ada8:	48 8d 9c 24 e8 08 00 	lea    0x8e8(%rsp),%rbx
   2adaf:	00 
   2adb0:	4c 8b 25 49 f8 05 00 	mov    0x5f849(%rip),%r12        # 8a600 <memcpy@GLIBC_2.14>
   2adb7:	ba c0 08 00 00       	mov    $0x8c0,%edx
   2adbc:	48 89 df             	mov    %rbx,%rdi
   2adbf:	41 ff d4             	call   *%r12
   2adc2:	48 8d ac 24 a8 11 00 	lea    0x11a8(%rsp),%rbp
   2adc9:	00 
   2adca:	ba 60 04 00 00       	mov    $0x460,%edx
   2adcf:	48 89 ef             	mov    %rbp,%rdi
   2add2:	48 89 de             	mov    %rbx,%rsi
   2add5:	41 ff d4             	call   *%r12
   2add8:	48 8d b4 24 48 0d 00 	lea    0xd48(%rsp),%rsi
   2addf:	00 
   2ade0:	48 8d 9c 24 08 16 00 	lea    0x1608(%rsp),%rbx
   2ade7:	00 
   2ade8:	ba 60 04 00 00       	mov    $0x460,%edx
   2aded:	48 89 df             	mov    %rbx,%rdi
   2adf0:	41 ff d4             	call   *%r12
   2adf3:	49 8d bf 00 80 00 00 	lea    0x8000(%r15),%rdi
   2adfa:	ba 60 04 00 00       	mov    $0x460,%edx
   2adff:	48 89 ee             	mov    %rbp,%rsi
   2ae02:	41 ff d4             	call   *%r12
   2ae05:	49 8d bf 60 84 00 00 	lea    0x8460(%r15),%rdi
   2ae0c:	ba 60 04 00 00       	mov    $0x460,%edx
   2ae11:	48 89 de             	mov    %rbx,%rsi
   2ae14:	41 ff d4             	call   *%r12
   2ae17:	b1 02                	mov    $0x2,%cl
   2ae19:	41 88 8f 08 89 00 00 	mov    %cl,0x8908(%r15)
   2ae20:	41 0f b6 cd          	movzbl %r13b,%ecx
   2ae24:	41 0f b6 d6          	movzbl %r14b,%edx
   2ae28:	0f a3 ca             	bt     %ecx,%edx
   2ae2b:	41 0f 92 87 09 89 00 	setb   0x8909(%r15)
   2ae32:	00 
   2ae33:	e9 a0 fa ff ff       	jmp    2a8d8 <<verified_tiff::deflate::Cursor>::step+0x4f8>
   2ae38:	49 ff c1             	inc    %r9
   2ae3b:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2ae42:	4c 39 d8             	cmp    %r11,%rax
   2ae45:	0f 84 dd 00 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2ae4b:	48 39 d0             	cmp    %rdx,%rax
   2ae4e:	0f 83 88 01 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2ae54:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2ae59:	49 83 f9 07          	cmp    $0x7,%r9
   2ae5d:	b9 07 00 00 00       	mov    $0x7,%ecx
   2ae62:	49 0f 42 c9          	cmovb  %r9,%rcx
   2ae66:	41 d2 ed             	shr    %cl,%r13b
   2ae69:	49 83 f9 07          	cmp    $0x7,%r9
   2ae6d:	75 0f                	jne    2ae7e <<verified_tiff::deflate::Cursor>::step+0xa9e>
   2ae6f:	48 ff c0             	inc    %rax
   2ae72:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2ae79:	45 31 c9             	xor    %r9d,%r9d
   2ae7c:	eb 03                	jmp    2ae81 <<verified_tiff::deflate::Cursor>::step+0xaa1>
   2ae7e:	49 ff c1             	inc    %r9
   2ae81:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2ae88:	4c 39 d8             	cmp    %r11,%rax
   2ae8b:	0f 84 97 00 00 00    	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2ae91:	44 88 2c 24          	mov    %r13b,(%rsp)
   2ae95:	48 39 d0             	cmp    %rdx,%rax
   2ae98:	0f 83 3e 01 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2ae9e:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2aea3:	49 83 f9 07          	cmp    $0x7,%r9
   2aea7:	b9 07 00 00 00       	mov    $0x7,%ecx
   2aeac:	49 0f 42 c9          	cmovb  %r9,%rcx
   2aeb0:	41 d2 ed             	shr    %cl,%r13b
   2aeb3:	44 88 6c 24 08       	mov    %r13b,0x8(%rsp)
   2aeb8:	49 83 f9 07          	cmp    $0x7,%r9
   2aebc:	75 0f                	jne    2aecd <<verified_tiff::deflate::Cursor>::step+0xaed>
   2aebe:	48 ff c0             	inc    %rax
   2aec1:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2aec8:	45 31 c9             	xor    %r9d,%r9d
   2aecb:	eb 03                	jmp    2aed0 <<verified_tiff::deflate::Cursor>::step+0xaf0>
   2aecd:	49 ff c1             	inc    %r9
   2aed0:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2aed7:	4c 39 d8             	cmp    %r11,%rax
   2aeda:	74 4c                	je     2af28 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2aedc:	48 39 d0             	cmp    %rdx,%rax
   2aedf:	0f 83 f7 00 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2aee5:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2aeea:	49 83 f9 07          	cmp    $0x7,%r9
   2aeee:	b9 07 00 00 00       	mov    $0x7,%ecx
   2aef3:	49 0f 42 c9          	cmovb  %r9,%rcx
   2aef7:	41 d2 ed             	shr    %cl,%r13b
   2aefa:	44 88 6c 24 10       	mov    %r13b,0x10(%rsp)
   2aeff:	49 83 f9 07          	cmp    $0x7,%r9
   2af03:	75 0f                	jne    2af14 <<verified_tiff::deflate::Cursor>::step+0xb34>
   2af05:	48 ff c0             	inc    %rax
   2af08:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2af0f:	45 31 c9             	xor    %r9d,%r9d
   2af12:	eb 03                	jmp    2af17 <<verified_tiff::deflate::Cursor>::step+0xb37>
   2af14:	49 ff c1             	inc    %r9
   2af17:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2af1e:	4c 39 d8             	cmp    %r11,%rax
   2af21:	44 0f b6 1c 24       	movzbl (%rsp),%r11d
   2af26:	75 21                	jne    2af49 <<verified_tiff::deflate::Cursor>::step+0xb69>
   2af28:	45 31 e4             	xor    %r12d,%r12d
   2af2b:	41 0f b6 cc          	movzbl %r12b,%ecx
   2af2f:	c1 e1 08             	shl    $0x8,%ecx
   2af32:	0f b7 c5             	movzwl %bp,%eax
   2af35:	09 c8                	or     %ecx,%eax
   2af37:	48 81 c4 68 1a 00 00 	add    $0x1a68,%rsp
   2af3e:	5b                   	pop    %rbx
   2af3f:	41 5c                	pop    %r12
   2af41:	41 5d                	pop    %r13
   2af43:	41 5e                	pop    %r14
   2af45:	41 5f                	pop    %r15
   2af47:	5d                   	pop    %rbp
   2af48:	c3                   	ret
   2af49:	48 39 d0             	cmp    %rdx,%rax
   2af4c:	0f 83 8a 00 00 00    	jae    2afdc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2af52:	45 00 f6             	add    %r14b,%r14b
   2af55:	41 80 e6 02          	and    $0x2,%r14b
   2af59:	44 08 f3             	or     %r14b,%bl
   2af5c:	41 c0 e7 02          	shl    $0x2,%r15b
   2af60:	41 80 e7 04          	and    $0x4,%r15b
   2af64:	41 08 df             	or     %bl,%r15b
   2af67:	41 c0 e4 03          	shl    $0x3,%r12b
   2af6b:	41 80 e4 08          	and    $0x8,%r12b
   2af6f:	45 08 fc             	or     %r15b,%r12b
   2af72:	41 c0 e3 04          	shl    $0x4,%r11b
   2af76:	41 80 e3 10          	and    $0x10,%r11b
   2af7a:	45 08 e3             	or     %r12b,%r11b
   2af7d:	0f b6 4c 24 08       	movzbl 0x8(%rsp),%ecx
   2af82:	c0 e1 05             	shl    $0x5,%cl
   2af85:	80 e1 20             	and    $0x20,%cl
   2af88:	44 08 d9             	or     %r11b,%cl
   2af8b:	0f b6 54 24 10       	movzbl 0x10(%rsp),%edx
   2af90:	c0 e2 06             	shl    $0x6,%dl
   2af93:	80 e2 40             	and    $0x40,%dl
   2af96:	08 ca                	or     %cl,%dl
   2af98:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   2af9d:	49 83 f9 07          	cmp    $0x7,%r9
   2afa1:	b9 07 00 00 00       	mov    $0x7,%ecx
   2afa6:	49 0f 42 c9          	cmovb  %r9,%rcx
   2afaa:	41 d2 ec             	shr    %cl,%r12b
   2afad:	41 c0 e4 07          	shl    $0x7,%r12b
   2afb1:	41 08 d4             	or     %dl,%r12b
   2afb4:	49 ff c1             	inc    %r9
   2afb7:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2afbe:	4c 89 d1             	mov    %r10,%rcx
   2afc1:	e9 9d f9 ff ff       	jmp    2a963 <<verified_tiff::deflate::Cursor>::step+0x583>
   2afc6:	48 8d 0d 13 c0 05 00 	lea    0x5c013(%rip),%rcx        # 86fe0 <anon.7d3d7a5a00fe104142489088ec6a6e3f.2.llvm.691752062989596110>
   2afcd:	48 89 c7             	mov    %rax,%rdi
   2afd0:	48 89 d6             	mov    %rdx,%rsi
   2afd3:	48 89 ca             	mov    %rcx,%rdx
   2afd6:	ff 15 24 f7 05 00    	call   *0x5f724(%rip)        # 8a700 <_DYNAMIC+0x350>
   2afdc:	48 89 d6             	mov    %rdx,%rsi
   2afdf:	48 8d 15 fa bf 05 00 	lea    0x5bffa(%rip),%rdx        # 86fe0 <anon.7d3d7a5a00fe104142489088ec6a6e3f.2.llvm.691752062989596110>
   2afe6:	48 89 c7             	mov    %rax,%rdi
   2afe9:	ff 15 11 f7 05 00    	call   *0x5f711(%rip)        # 8a700 <_DYNAMIC+0x350>
   2afef:	48 8d 05 ea bf 05 00 	lea    0x5bfea(%rip),%rax        # 86fe0 <anon.7d3d7a5a00fe104142489088ec6a6e3f.2.llvm.691752062989596110>
   2aff6:	48 89 cf             	mov    %rcx,%rdi
   2aff9:	48 89 d6             	mov    %rdx,%rsi
   2affc:	48 89 c2             	mov    %rax,%rdx
   2afff:	ff 15 fb f6 05 00    	call   *0x5f6fb(%rip)        # 8a700 <_DYNAMIC+0x350>
   2b005:	48 8d 15 84 c1 05 00 	lea    0x5c184(%rip),%rdx        # 87190 <anon.7d3d7a5a00fe104142489088ec6a6e3f.26.llvm.691752062989596110>
   2b00c:	be 20 01 00 00       	mov    $0x120,%esi
   2b011:	4c 89 c7             	mov    %r8,%rdi
   2b014:	ff 15 e6 f6 05 00    	call   *0x5f6e6(%rip)        # 8a700 <_DYNAMIC+0x350>
   2b01a:	48 8d 0d 3f be 05 00 	lea    0x5be3f(%rip),%rcx        # 86e60 <anon.35f1884359c0d9048fdad1d4cf8af759.18.llvm.1848552889070027181>
   2b021:	48 89 c7             	mov    %rax,%rdi
   2b024:	48 89 d6             	mov    %rdx,%rsi
   2b027:	48 89 ca             	mov    %rcx,%rdx
   2b02a:	ff 15 d0 f6 05 00    	call   *0x5f6d0(%rip)        # 8a700 <_DYNAMIC+0x350>
   2b030:	48 8d 05 41 be 05 00 	lea    0x5be41(%rip),%rax        # 86e78 <anon.35f1884359c0d9048fdad1d4cf8af759.19.llvm.1848552889070027181>
   2b037:	48 89 cf             	mov    %rcx,%rdi
   2b03a:	48 89 d6             	mov    %rdx,%rsi
   2b03d:	48 89 c2             	mov    %rax,%rdx
   2b040:	ff 15 ba f6 05 00    	call   *0x5f6ba(%rip)        # 8a700 <_DYNAMIC+0x350>
   2b046:	48 8d 0d 7b bf 05 00 	lea    0x5bf7b(%rip),%rcx        # 86fc8 <anon.35f1884359c0d9048fdad1d4cf8af759.24.llvm.1848552889070027181+0xd8>
   2b04d:	48 89 c7             	mov    %rax,%rdi
   2b050:	48 89 d6             	mov    %rdx,%rsi
   2b053:	48 89 ca             	mov    %rcx,%rdx
   2b056:	ff 15 a4 f6 05 00    	call   *0x5f6a4(%rip)        # 8a700 <_DYNAMIC+0x350>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
