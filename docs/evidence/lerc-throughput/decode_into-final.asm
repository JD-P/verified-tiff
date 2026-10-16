
target/compat/libtiff.so.6:     file format elf64-x86-64


Disassembly of section .text:

000000000006e4f0 <verified_tiff::lerc_bits::decode_into>:
   6e4f0:	55                   	push   %rbp
   6e4f1:	41 57                	push   %r15
   6e4f3:	41 56                	push   %r14
   6e4f5:	41 55                	push   %r13
   6e4f7:	41 54                	push   %r12
   6e4f9:	53                   	push   %rbx
   6e4fa:	48 83 ec 58          	sub    $0x58,%rsp
   6e4fe:	48 39 d1             	cmp    %rdx,%rcx
   6e501:	0f 93 c0             	setae  %al
   6e504:	49 89 d1             	mov    %rdx,%r9
   6e507:	49 c1 e9 3d          	shr    $0x3d,%r9
   6e50b:	41 0f 95 c1          	setne  %r9b
   6e50f:	41 08 c1             	or     %al,%r9b
   6e512:	75 29                	jne    6e53d <verified_tiff::lerc_bits::decode_into+0x4d>
   6e514:	0f b6 2c 0e          	movzbl (%rsi,%rcx,1),%ebp
   6e518:	41 89 ed             	mov    %ebp,%r13d
   6e51b:	41 c1 ed 06          	shr    $0x6,%r13d
   6e51f:	74 08                	je     6e529 <verified_tiff::lerc_bits::decode_into+0x39>
   6e521:	49 83 f5 03          	xor    $0x3,%r13
   6e525:	75 08                	jne    6e52f <verified_tiff::lerc_bits::decode_into+0x3f>
   6e527:	eb 14                	jmp    6e53d <verified_tiff::lerc_bits::decode_into+0x4d>
   6e529:	41 bd 04 00 00 00    	mov    $0x4,%r13d
   6e52f:	48 89 c8             	mov    %rcx,%rax
   6e532:	48 f7 d0             	not    %rax
   6e535:	48 01 d0             	add    %rdx,%rax
   6e538:	49 39 c5             	cmp    %rax,%r13
   6e53b:	76 18                	jbe    6e555 <verified_tiff::lerc_bits::decode_into+0x65>
   6e53d:	31 db                	xor    %ebx,%ebx
   6e53f:	88 5f 01             	mov    %bl,0x1(%rdi)
   6e542:	b0 01                	mov    $0x1,%al
   6e544:	88 07                	mov    %al,(%rdi)
   6e546:	48 83 c4 58          	add    $0x58,%rsp
   6e54a:	5b                   	pop    %rbx
   6e54b:	41 5c                	pop    %r12
   6e54d:	41 5d                	pop    %r13
   6e54f:	41 5e                	pop    %r14
   6e551:	41 5f                	pop    %r15
   6e553:	5d                   	pop    %rbp
   6e554:	c3                   	ret
   6e555:	41 8d 45 ff          	lea    -0x1(%r13),%eax
   6e559:	b3 02                	mov    $0x2,%bl
   6e55b:	41 85 c5             	test   %eax,%r13d
   6e55e:	75 df                	jne    6e53f <verified_tiff::lerc_bits::decode_into+0x4f>
   6e560:	4c 8d 71 01          	lea    0x1(%rcx),%r14
   6e564:	f3 4d 0f bc cd       	tzcnt  %r13,%r9
   6e569:	48 89 d0             	mov    %rdx,%rax
   6e56c:	4c 29 f0             	sub    %r14,%rax
   6e56f:	49 83 f9 02          	cmp    $0x2,%r9
   6e573:	74 40                	je     6e5b5 <verified_tiff::lerc_bits::decode_into+0xc5>
   6e575:	41 83 f9 01          	cmp    $0x1,%r9d
   6e579:	0f 85 8f 00 00 00    	jne    6e60e <verified_tiff::lerc_bits::decode_into+0x11e>
   6e57f:	49 39 c5             	cmp    %rax,%r13
   6e582:	77 b9                	ja     6e53d <verified_tiff::lerc_bits::decode_into+0x4d>
   6e584:	4d 89 c4             	mov    %r8,%r12
   6e587:	49 89 ff             	mov    %rdi,%r15
   6e58a:	48 89 74 24 18       	mov    %rsi,0x18(%rsp)
   6e58f:	48 89 f7             	mov    %rsi,%rdi
   6e592:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
   6e597:	48 89 d6             	mov    %rdx,%rsi
   6e59a:	4c 89 f2             	mov    %r14,%rdx
   6e59d:	e8 1e c1 ff ff       	call   6a6c0 <verified_tiff::read16>
   6e5a2:	89 c3                	mov    %eax,%ebx
   6e5a4:	f6 c3 01             	test   $0x1,%bl
   6e5a7:	74 7e                	je     6e627 <verified_tiff::lerc_bits::decode_into+0x137>
   6e5a9:	c1 eb 08             	shr    $0x8,%ebx
   6e5ac:	4c 89 ff             	mov    %r15,%rdi
   6e5af:	41 88 5f 01          	mov    %bl,0x1(%r15)
   6e5b3:	eb 8d                	jmp    6e542 <verified_tiff::lerc_bits::decode_into+0x52>
   6e5b5:	49 39 c5             	cmp    %rax,%r13
   6e5b8:	77 83                	ja     6e53d <verified_tiff::lerc_bits::decode_into+0x4d>
   6e5ba:	49 89 cf             	mov    %rcx,%r15
   6e5bd:	4c 89 44 24 10       	mov    %r8,0x10(%rsp)
   6e5c2:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
   6e5c7:	48 89 74 24 18       	mov    %rsi,0x18(%rsp)
   6e5cc:	48 89 f7             	mov    %rsi,%rdi
   6e5cf:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
   6e5d4:	48 89 d6             	mov    %rdx,%rsi
   6e5d7:	4c 89 f2             	mov    %r14,%rdx
   6e5da:	b9 01 00 00 00       	mov    $0x1,%ecx
   6e5df:	ff 15 83 b7 12 00    	call   *0x12b783(%rip)        # 199d68 <_GLOBAL_OFFSET_TABLE_+0x1208>
   6e5e5:	48 89 c3             	mov    %rax,%rbx
   6e5e8:	41 83 fd 04          	cmp    $0x4,%r13d
   6e5ec:	0f 85 40 04 00 00    	jne    6ea32 <verified_tiff::lerc_bits::decode_into+0x542>
   6e5f2:	f6 c3 01             	test   $0x1,%bl
   6e5f5:	4c 8b 44 24 10       	mov    0x10(%rsp),%r8
   6e5fa:	0f 85 37 04 00 00    	jne    6ea37 <verified_tiff::lerc_bits::decode_into+0x547>
   6e600:	48 c1 eb 20          	shr    $0x20,%rbx
   6e604:	48 89 d8             	mov    %rbx,%rax
   6e607:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   6e60c:	eb 25                	jmp    6e633 <verified_tiff::lerc_bits::decode_into+0x143>
   6e60e:	49 39 c5             	cmp    %rax,%r13
   6e611:	0f 87 26 ff ff ff    	ja     6e53d <verified_tiff::lerc_bits::decode_into+0x4d>
   6e617:	4c 39 f2             	cmp    %r14,%rdx
   6e61a:	0f 86 23 05 00 00    	jbe    6eb43 <verified_tiff::lerc_bits::decode_into+0x653>
   6e620:	42 0f b6 04 36       	movzbl (%rsi,%r14,1),%eax
   6e625:	eb 24                	jmp    6e64b <verified_tiff::lerc_bits::decode_into+0x15b>
   6e627:	c1 eb 10             	shr    $0x10,%ebx
   6e62a:	48 89 d8             	mov    %rbx,%rax
   6e62d:	4c 89 ff             	mov    %r15,%rdi
   6e630:	4d 89 e0             	mov    %r12,%r8
   6e633:	b3 01                	mov    $0x1,%bl
   6e635:	48 3d 00 01 00 00    	cmp    $0x100,%rax
   6e63b:	48 8b 54 24 20       	mov    0x20(%rsp),%rdx
   6e640:	48 8b 74 24 18       	mov    0x18(%rsp),%rsi
   6e645:	0f 87 f4 fe ff ff    	ja     6e53f <verified_tiff::lerc_bits::decode_into+0x4f>
   6e64b:	41 89 ef             	mov    %ebp,%r15d
   6e64e:	41 83 e7 1f          	and    $0x1f,%r15d
   6e652:	4d 01 f5             	add    %r14,%r13
   6e655:	40 f6 c5 20          	test   $0x20,%bpl
   6e659:	0f 85 f8 02 00 00    	jne    6e957 <verified_tiff::lerc_bits::decode_into+0x467>
   6e65f:	49 89 c4             	mov    %rax,%r12
   6e662:	4d 0f af e7          	imul   %r15,%r12
   6e666:	49 83 c4 07          	add    $0x7,%r12
   6e66a:	49 c1 ec 03          	shr    $0x3,%r12
   6e66e:	48 89 d1             	mov    %rdx,%rcx
   6e671:	4c 29 e9             	sub    %r13,%rcx
   6e674:	49 39 cc             	cmp    %rcx,%r12
   6e677:	0f 87 c0 fe ff ff    	ja     6e53d <verified_tiff::lerc_bits::decode_into+0x4d>
   6e67d:	4d 01 ec             	add    %r13,%r12
   6e680:	45 31 c9             	xor    %r9d,%r9d
   6e683:	4c 89 6c 24 28       	mov    %r13,0x28(%rsp)
   6e688:	4d 89 fb             	mov    %r15,%r11
   6e68b:	48 85 c0             	test   %rax,%rax
   6e68e:	0f 84 b4 02 00 00    	je     6e948 <verified_tiff::lerc_bits::decode_into+0x458>
   6e694:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6e699:	4e 8d 34 d5 00 00 00 	lea    0x0(,%r10,8),%r14
   6e6a0:	00 
   6e6a1:	48 c7 c3 ff ff ff ff 	mov    $0xffffffffffffffff,%rbx
   6e6a8:	44 89 d9             	mov    %r11d,%ecx
   6e6ab:	48 d3 e3             	shl    %cl,%rbx
   6e6ae:	48 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%rbp
   6e6b5:	f7 d3                	not    %ebx
   6e6b7:	48 89 5c 24 50       	mov    %rbx,0x50(%rsp)
   6e6bc:	49 8d 49 ff          	lea    -0x1(%r9),%rcx
   6e6c0:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   6e6c5:	49 c1 e5 03          	shl    $0x3,%r13
   6e6c9:	44 89 f9             	mov    %r15d,%ecx
   6e6cc:	48 d3 e5             	shl    %cl,%rbp
   6e6cf:	49 8d 4a fb          	lea    -0x5(%r10),%rcx
   6e6d3:	48 89 4c 24 48       	mov    %rcx,0x48(%rsp)
   6e6d8:	48 f7 d5             	not    %rbp
   6e6db:	48 89 6c 24 40       	mov    %rbp,0x40(%rsp)
   6e6e0:	31 ed                	xor    %ebp,%ebp
   6e6e2:	4c 89 74 24 18       	mov    %r14,0x18(%rsp)
   6e6e7:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   6e6ec:	4c 89 7c 24 38       	mov    %r15,0x38(%rsp)
   6e6f1:	4c 89 4c 24 30       	mov    %r9,0x30(%rsp)
   6e6f6:	4c 89 5c 24 10       	mov    %r11,0x10(%rsp)
   6e6fb:	48 89 e9             	mov    %rbp,%rcx
   6e6fe:	49 0f af cb          	imul   %r11,%rcx
   6e702:	48 03 4c 24 18       	add    0x18(%rsp),%rcx
   6e707:	4e 8d 14 e5 00 00 00 	lea    0x0(,%r12,8),%r10
   6e70e:	00 
   6e70f:	49 29 ca             	sub    %rcx,%r10
   6e712:	4d 39 d3             	cmp    %r10,%r11
   6e715:	0f 87 7b 03 00 00    	ja     6ea96 <verified_tiff::lerc_bits::decode_into+0x5a6>
   6e71b:	49 83 fc 05          	cmp    $0x5,%r12
   6e71f:	73 64                	jae    6e785 <verified_tiff::lerc_bits::decode_into+0x295>
   6e721:	4d 85 db             	test   %r11,%r11
   6e724:	0f 84 63 01 00 00    	je     6e88d <verified_tiff::lerc_bits::decode_into+0x39d>
   6e72a:	4d 89 da             	mov    %r11,%r10
   6e72d:	41 bb 01 00 00 00    	mov    $0x1,%r11d
   6e733:	4c 89 f3             	mov    %r14,%rbx
   6e736:	45 31 ff             	xor    %r15d,%r15d
   6e739:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   6e740:	48 89 d9             	mov    %rbx,%rcx
   6e743:	48 c1 e9 03          	shr    $0x3,%rcx
   6e747:	48 39 d1             	cmp    %rdx,%rcx
   6e74a:	0f 83 58 03 00 00    	jae    6eaa8 <verified_tiff::lerc_bits::decode_into+0x5b8>
   6e750:	0f b6 0c 0e          	movzbl (%rsi,%rcx,1),%ecx
   6e754:	0f b6 c3             	movzbl %bl,%eax
   6e757:	83 e0 07             	and    $0x7,%eax
   6e75a:	45 31 c9             	xor    %r9d,%r9d
   6e75d:	0f a3 c1             	bt     %eax,%ecx
   6e760:	41 0f 92 c1          	setb   %r9b
   6e764:	4d 0f af cb          	imul   %r11,%r9
   6e768:	4d 01 cf             	add    %r9,%r15
   6e76b:	4d 01 db             	add    %r11,%r11
   6e76e:	48 ff c3             	inc    %rbx
   6e771:	49 ff ca             	dec    %r10
   6e774:	75 ca                	jne    6e740 <verified_tiff::lerc_bits::decode_into+0x250>
   6e776:	45 89 fb             	mov    %r15d,%r11d
   6e779:	4c 8b 7c 24 38       	mov    0x38(%rsp),%r15
   6e77e:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6e783:	eb 74                	jmp    6e7f9 <verified_tiff::lerc_bits::decode_into+0x309>
   6e785:	48 89 cb             	mov    %rcx,%rbx
   6e788:	48 c1 eb 03          	shr    $0x3,%rbx
   6e78c:	4d 89 e2             	mov    %r12,%r10
   6e78f:	49 29 da             	sub    %rbx,%r10
   6e792:	49 83 fa 05          	cmp    $0x5,%r10
   6e796:	49 8d 44 24 fb       	lea    -0x5(%r12),%rax
   6e79b:	48 0f 42 d8          	cmovb  %rax,%rbx
   6e79f:	48 39 d3             	cmp    %rdx,%rbx
   6e7a2:	0f 83 2d 03 00 00    	jae    6ead5 <verified_tiff::lerc_bits::decode_into+0x5e5>
   6e7a8:	4c 8d 5b 01          	lea    0x1(%rbx),%r11
   6e7ac:	49 39 d3             	cmp    %rdx,%r11
   6e7af:	0f 83 36 03 00 00    	jae    6eaeb <verified_tiff::lerc_bits::decode_into+0x5fb>
   6e7b5:	4c 8d 5b 02          	lea    0x2(%rbx),%r11
   6e7b9:	49 39 d3             	cmp    %rdx,%r11
   6e7bc:	0f 83 3f 03 00 00    	jae    6eb01 <verified_tiff::lerc_bits::decode_into+0x611>
   6e7c2:	4c 8d 5b 03          	lea    0x3(%rbx),%r11
   6e7c6:	49 39 d3             	cmp    %rdx,%r11
   6e7c9:	0f 83 48 03 00 00    	jae    6eb17 <verified_tiff::lerc_bits::decode_into+0x627>
   6e7cf:	4c 8d 5b 04          	lea    0x4(%rbx),%r11
   6e7d3:	49 39 d3             	cmp    %rdx,%r11
   6e7d6:	0f 83 51 03 00 00    	jae    6eb2d <verified_tiff::lerc_bits::decode_into+0x63d>
   6e7dc:	8b 04 1e             	mov    (%rsi,%rbx,1),%eax
   6e7df:	44 0f b6 5c 1e 04    	movzbl 0x4(%rsi,%rbx,1),%r11d
   6e7e5:	49 c1 e3 20          	shl    $0x20,%r11
   6e7e9:	49 09 c3             	or     %rax,%r11
   6e7ec:	c1 e3 03             	shl    $0x3,%ebx
   6e7ef:	29 d9                	sub    %ebx,%ecx
   6e7f1:	49 d3 eb             	shr    %cl,%r11
   6e7f4:	44 23 5c 24 50       	and    0x50(%rsp),%r11d
   6e7f9:	4c 39 5c 24 20       	cmp    %r11,0x20(%rsp)
   6e7fe:	0f 82 9b 02 00 00    	jb     6ea9f <verified_tiff::lerc_bits::decode_into+0x5af>
   6e804:	4d 85 c9             	test   %r9,%r9
   6e807:	74 7f                	je     6e888 <verified_tiff::lerc_bits::decode_into+0x398>
   6e809:	4d 85 db             	test   %r11,%r11
   6e80c:	0f 84 82 00 00 00    	je     6e894 <verified_tiff::lerc_bits::decode_into+0x3a4>
   6e812:	49 ff cb             	dec    %r11
   6e815:	4d 0f af df          	imul   %r15,%r11
   6e819:	4d 01 eb             	add    %r13,%r11
   6e81c:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   6e821:	4c 29 d8             	sub    %r11,%rax
   6e824:	49 39 c7             	cmp    %rax,%r15
   6e827:	0f 87 69 02 00 00    	ja     6ea96 <verified_tiff::lerc_bits::decode_into+0x5a6>
   6e82d:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
   6e832:	48 83 f8 05          	cmp    $0x5,%rax
   6e836:	73 68                	jae    6e8a0 <verified_tiff::lerc_bits::decode_into+0x3b0>
   6e838:	4d 85 ff             	test   %r15,%r15
   6e83b:	0f 84 d1 00 00 00    	je     6e912 <verified_tiff::lerc_bits::decode_into+0x422>
   6e841:	4d 89 fa             	mov    %r15,%r10
   6e844:	41 bf 01 00 00 00    	mov    $0x1,%r15d
   6e84a:	31 db                	xor    %ebx,%ebx
   6e84c:	4c 89 d9             	mov    %r11,%rcx
   6e84f:	48 c1 e9 03          	shr    $0x3,%rcx
   6e853:	48 39 d1             	cmp    %rdx,%rcx
   6e856:	0f 83 4c 02 00 00    	jae    6eaa8 <verified_tiff::lerc_bits::decode_into+0x5b8>
   6e85c:	0f b6 04 0e          	movzbl (%rsi,%rcx,1),%eax
   6e860:	41 0f b6 cb          	movzbl %r11b,%ecx
   6e864:	83 e1 07             	and    $0x7,%ecx
   6e867:	45 31 c9             	xor    %r9d,%r9d
   6e86a:	0f a3 c8             	bt     %ecx,%eax
   6e86d:	41 0f 92 c1          	setb   %r9b
   6e871:	4d 0f af cf          	imul   %r15,%r9
   6e875:	4c 01 cb             	add    %r9,%rbx
   6e878:	4d 01 ff             	add    %r15,%r15
   6e87b:	49 ff c3             	inc    %r11
   6e87e:	49 ff ca             	dec    %r10
   6e881:	75 c9                	jne    6e84c <verified_tiff::lerc_bits::decode_into+0x35c>
   6e883:	e9 8c 00 00 00       	jmp    6e914 <verified_tiff::lerc_bits::decode_into+0x424>
   6e888:	44 89 db             	mov    %r11d,%ebx
   6e88b:	eb 09                	jmp    6e896 <verified_tiff::lerc_bits::decode_into+0x3a6>
   6e88d:	31 db                	xor    %ebx,%ebx
   6e88f:	e9 94 00 00 00       	jmp    6e928 <verified_tiff::lerc_bits::decode_into+0x438>
   6e894:	31 db                	xor    %ebx,%ebx
   6e896:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   6e89b:	e9 83 00 00 00       	jmp    6e923 <verified_tiff::lerc_bits::decode_into+0x433>
   6e8a0:	4c 89 d9             	mov    %r11,%rcx
   6e8a3:	48 c1 e9 03          	shr    $0x3,%rcx
   6e8a7:	48 29 c8             	sub    %rcx,%rax
   6e8aa:	48 83 f8 05          	cmp    $0x5,%rax
   6e8ae:	48 0f 42 4c 24 48    	cmovb  0x48(%rsp),%rcx
   6e8b4:	48 39 d1             	cmp    %rdx,%rcx
   6e8b7:	0f 83 9c 02 00 00    	jae    6eb59 <verified_tiff::lerc_bits::decode_into+0x669>
   6e8bd:	48 8d 59 01          	lea    0x1(%rcx),%rbx
   6e8c1:	48 39 d3             	cmp    %rdx,%rbx
   6e8c4:	0f 83 a5 02 00 00    	jae    6eb6f <verified_tiff::lerc_bits::decode_into+0x67f>
   6e8ca:	48 8d 59 02          	lea    0x2(%rcx),%rbx
   6e8ce:	48 39 d3             	cmp    %rdx,%rbx
   6e8d1:	0f 83 ae 02 00 00    	jae    6eb85 <verified_tiff::lerc_bits::decode_into+0x695>
   6e8d7:	48 8d 59 03          	lea    0x3(%rcx),%rbx
   6e8db:	48 39 d3             	cmp    %rdx,%rbx
   6e8de:	0f 83 b7 02 00 00    	jae    6eb9b <verified_tiff::lerc_bits::decode_into+0x6ab>
   6e8e4:	48 8d 59 04          	lea    0x4(%rcx),%rbx
   6e8e8:	48 39 d3             	cmp    %rdx,%rbx
   6e8eb:	0f 83 c0 02 00 00    	jae    6ebb1 <verified_tiff::lerc_bits::decode_into+0x6c1>
   6e8f1:	8b 04 0e             	mov    (%rsi,%rcx,1),%eax
   6e8f4:	0f b6 5c 0e 04       	movzbl 0x4(%rsi,%rcx,1),%ebx
   6e8f9:	48 c1 e3 20          	shl    $0x20,%rbx
   6e8fd:	48 09 c3             	or     %rax,%rbx
   6e900:	c1 e1 03             	shl    $0x3,%ecx
   6e903:	41 29 cb             	sub    %ecx,%r11d
   6e906:	44 89 d9             	mov    %r11d,%ecx
   6e909:	48 d3 eb             	shr    %cl,%rbx
   6e90c:	23 5c 24 40          	and    0x40(%rsp),%ebx
   6e910:	eb 02                	jmp    6e914 <verified_tiff::lerc_bits::decode_into+0x424>
   6e912:	31 db                	xor    %ebx,%ebx
   6e914:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   6e919:	4c 8b 7c 24 38       	mov    0x38(%rsp),%r15
   6e91e:	4c 8b 4c 24 30       	mov    0x30(%rsp),%r9
   6e923:	4c 8b 5c 24 10       	mov    0x10(%rsp),%r11
   6e928:	48 81 fd 00 01 00 00 	cmp    $0x100,%rbp
   6e92f:	0f 84 89 01 00 00    	je     6eabe <verified_tiff::lerc_bits::decode_into+0x5ce>
   6e935:	41 89 1c a8          	mov    %ebx,(%r8,%rbp,4)
   6e939:	48 ff c5             	inc    %rbp
   6e93c:	4d 01 de             	add    %r11,%r14
   6e93f:	48 39 c5             	cmp    %rax,%rbp
   6e942:	0f 85 b3 fd ff ff    	jne    6e6fb <verified_tiff::lerc_bits::decode_into+0x20b>
   6e948:	4c 89 67 08          	mov    %r12,0x8(%rdi)
   6e94c:	48 89 47 10          	mov    %rax,0x10(%rdi)
   6e950:	31 c0                	xor    %eax,%eax
   6e952:	e9 ed fb ff ff       	jmp    6e544 <verified_tiff::lerc_bits::decode_into+0x54>
   6e957:	4d 85 ff             	test   %r15,%r15
   6e95a:	0f 94 c1             	sete   %cl
   6e95d:	4c 39 ea             	cmp    %r13,%rdx
   6e960:	41 0f 96 c1          	setbe  %r9b
   6e964:	41 08 c9             	or     %cl,%r9b
   6e967:	b3 01                	mov    $0x1,%bl
   6e969:	0f 85 d0 fb ff ff    	jne    6e53f <verified_tiff::lerc_bits::decode_into+0x4f>
   6e96f:	46 0f b6 0c 2e       	movzbl (%rsi,%r13,1),%r9d
   6e974:	49 83 f9 02          	cmp    $0x2,%r9
   6e978:	0f 82 c1 fb ff ff    	jb     6e53f <verified_tiff::lerc_bits::decode_into+0x4f>
   6e97e:	49 ff c5             	inc    %r13
   6e981:	4d 8d 51 ff          	lea    -0x1(%r9),%r10
   6e985:	4d 0f af d7          	imul   %r15,%r10
   6e989:	49 83 c2 07          	add    $0x7,%r10
   6e98d:	49 c1 ea 03          	shr    $0x3,%r10
   6e991:	48 89 d1             	mov    %rdx,%rcx
   6e994:	4c 29 e9             	sub    %r13,%rcx
   6e997:	49 39 ca             	cmp    %rcx,%r10
   6e99a:	0f 87 9d fb ff ff    	ja     6e53d <verified_tiff::lerc_bits::decode_into+0x4d>
   6e9a0:	4d 01 ea             	add    %r13,%r10
   6e9a3:	4c 89 54 24 28       	mov    %r10,0x28(%rsp)
   6e9a8:	41 bb 01 00 00 00    	mov    $0x1,%r11d
   6e9ae:	41 83 f9 02          	cmp    $0x2,%r9d
   6e9b2:	74 4a                	je     6e9fe <verified_tiff::lerc_bits::decode_into+0x50e>
   6e9b4:	41 bb 02 00 00 00    	mov    $0x2,%r11d
   6e9ba:	41 80 f9 05          	cmp    $0x5,%r9b
   6e9be:	72 3e                	jb     6e9fe <verified_tiff::lerc_bits::decode_into+0x50e>
   6e9c0:	41 bb 03 00 00 00    	mov    $0x3,%r11d
   6e9c6:	41 80 f9 09          	cmp    $0x9,%r9b
   6e9ca:	72 32                	jb     6e9fe <verified_tiff::lerc_bits::decode_into+0x50e>
   6e9cc:	41 bb 04 00 00 00    	mov    $0x4,%r11d
   6e9d2:	41 80 f9 11          	cmp    $0x11,%r9b
   6e9d6:	72 26                	jb     6e9fe <verified_tiff::lerc_bits::decode_into+0x50e>
   6e9d8:	41 bb 05 00 00 00    	mov    $0x5,%r11d
   6e9de:	41 80 f9 21          	cmp    $0x21,%r9b
   6e9e2:	72 1a                	jb     6e9fe <verified_tiff::lerc_bits::decode_into+0x50e>
   6e9e4:	41 bb 06 00 00 00    	mov    $0x6,%r11d
   6e9ea:	41 80 f9 41          	cmp    $0x41,%r9b
   6e9ee:	72 0e                	jb     6e9fe <verified_tiff::lerc_bits::decode_into+0x50e>
   6e9f0:	41 80 f9 81          	cmp    $0x81,%r9b
   6e9f4:	41 bb 08 00 00 00    	mov    $0x8,%r11d
   6e9fa:	49 83 db 00          	sbb    $0x0,%r11
   6e9fe:	4d 89 dc             	mov    %r11,%r12
   6ea01:	4c 0f af e0          	imul   %rax,%r12
   6ea05:	49 83 c4 07          	add    $0x7,%r12
   6ea09:	49 c1 ec 03          	shr    $0x3,%r12
   6ea0d:	48 89 d1             	mov    %rdx,%rcx
   6ea10:	4c 8b 54 24 28       	mov    0x28(%rsp),%r10
   6ea15:	4c 29 d1             	sub    %r10,%rcx
   6ea18:	49 39 cc             	cmp    %rcx,%r12
   6ea1b:	0f 87 1c fb ff ff    	ja     6e53d <verified_tiff::lerc_bits::decode_into+0x4d>
   6ea21:	4d 01 d4             	add    %r10,%r12
   6ea24:	48 85 c0             	test   %rax,%rax
   6ea27:	0f 85 67 fc ff ff    	jne    6e694 <verified_tiff::lerc_bits::decode_into+0x1a4>
   6ea2d:	e9 16 ff ff ff       	jmp    6e948 <verified_tiff::lerc_bits::decode_into+0x458>
   6ea32:	f6 c3 01             	test   $0x1,%bl
   6ea35:	74 0d                	je     6ea44 <verified_tiff::lerc_bits::decode_into+0x554>
   6ea37:	c1 eb 08             	shr    $0x8,%ebx
   6ea3a:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   6ea3f:	e9 fb fa ff ff       	jmp    6e53f <verified_tiff::lerc_bits::decode_into+0x4f>
   6ea44:	4c 89 fa             	mov    %r15,%rdx
   6ea47:	48 83 c2 05          	add    $0x5,%rdx
   6ea4b:	48 8b 7c 24 18       	mov    0x18(%rsp),%rdi
   6ea50:	48 8b 74 24 20       	mov    0x20(%rsp),%rsi
   6ea55:	b9 01 00 00 00       	mov    $0x1,%ecx
   6ea5a:	ff 15 08 b3 12 00    	call   *0x12b308(%rip)        # 199d68 <_GLOBAL_OFFSET_TABLE_+0x1208>
   6ea60:	a8 01                	test   $0x1,%al
   6ea62:	74 0f                	je     6ea73 <verified_tiff::lerc_bits::decode_into+0x583>
   6ea64:	c1 e8 08             	shr    $0x8,%eax
   6ea67:	89 c3                	mov    %eax,%ebx
   6ea69:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   6ea6e:	e9 cc fa ff ff       	jmp    6e53f <verified_tiff::lerc_bits::decode_into+0x4f>
   6ea73:	48 c1 eb 20          	shr    $0x20,%rbx
   6ea77:	48 b9 00 00 00 00 ff 	movabs $0xffffffff00000000,%rcx
   6ea7e:	ff ff ff 
   6ea81:	48 21 c8             	and    %rcx,%rax
   6ea84:	48 09 d8             	or     %rbx,%rax
   6ea87:	48 8b 7c 24 08       	mov    0x8(%rsp),%rdi
   6ea8c:	4c 8b 44 24 10       	mov    0x10(%rsp),%r8
   6ea91:	e9 9d fb ff ff       	jmp    6e633 <verified_tiff::lerc_bits::decode_into+0x143>
   6ea96:	c6 47 01 00          	movb   $0x0,0x1(%rdi)
   6ea9a:	e9 a3 fa ff ff       	jmp    6e542 <verified_tiff::lerc_bits::decode_into+0x52>
   6ea9f:	c6 47 01 01          	movb   $0x1,0x1(%rdi)
   6eaa3:	e9 9a fa ff ff       	jmp    6e542 <verified_tiff::lerc_bits::decode_into+0x52>
   6eaa8:	48 8d 05 81 1e 12 00 	lea    0x121e81(%rip),%rax        # 190930 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.51.llvm.8789668757449324569>
   6eaaf:	48 89 cf             	mov    %rcx,%rdi
   6eab2:	48 89 d6             	mov    %rdx,%rsi
   6eab5:	48 89 c2             	mov    %rax,%rdx
   6eab8:	ff 15 02 ac 12 00    	call   *0x12ac02(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eabe:	48 8d 15 c3 1d 12 00 	lea    0x121dc3(%rip),%rdx        # 190888 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.23.llvm.8789668757449324569+0x168>
   6eac5:	bf 00 01 00 00       	mov    $0x100,%edi
   6eaca:	be 00 01 00 00       	mov    $0x100,%esi
   6eacf:	ff 15 eb ab 12 00    	call   *0x12abeb(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6ead5:	48 8d 05 dc 1d 12 00 	lea    0x121ddc(%rip),%rax        # 1908b8 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x18>
   6eadc:	48 89 df             	mov    %rbx,%rdi
   6eadf:	48 89 d6             	mov    %rdx,%rsi
   6eae2:	48 89 c2             	mov    %rax,%rdx
   6eae5:	ff 15 d5 ab 12 00    	call   *0x12abd5(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eaeb:	48 8d 05 de 1d 12 00 	lea    0x121dde(%rip),%rax        # 1908d0 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x30>
   6eaf2:	4c 89 df             	mov    %r11,%rdi
   6eaf5:	48 89 d6             	mov    %rdx,%rsi
   6eaf8:	48 89 c2             	mov    %rax,%rdx
   6eafb:	ff 15 bf ab 12 00    	call   *0x12abbf(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eb01:	48 8d 05 e0 1d 12 00 	lea    0x121de0(%rip),%rax        # 1908e8 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x48>
   6eb08:	4c 89 df             	mov    %r11,%rdi
   6eb0b:	48 89 d6             	mov    %rdx,%rsi
   6eb0e:	48 89 c2             	mov    %rax,%rdx
   6eb11:	ff 15 a9 ab 12 00    	call   *0x12aba9(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eb17:	48 8d 05 e2 1d 12 00 	lea    0x121de2(%rip),%rax        # 190900 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x60>
   6eb1e:	4c 89 df             	mov    %r11,%rdi
   6eb21:	48 89 d6             	mov    %rdx,%rsi
   6eb24:	48 89 c2             	mov    %rax,%rdx
   6eb27:	ff 15 93 ab 12 00    	call   *0x12ab93(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eb2d:	48 8d 05 e4 1d 12 00 	lea    0x121de4(%rip),%rax        # 190918 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x78>
   6eb34:	4c 89 df             	mov    %r11,%rdi
   6eb37:	48 89 d6             	mov    %rdx,%rsi
   6eb3a:	48 89 c2             	mov    %rax,%rdx
   6eb3d:	ff 15 7d ab 12 00    	call   *0x12ab7d(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eb43:	48 8d 05 ae 1c 12 00 	lea    0x121cae(%rip),%rax        # 1907f8 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.23.llvm.8789668757449324569+0xd8>
   6eb4a:	4c 89 f7             	mov    %r14,%rdi
   6eb4d:	48 89 d6             	mov    %rdx,%rsi
   6eb50:	48 89 c2             	mov    %rax,%rdx
   6eb53:	ff 15 67 ab 12 00    	call   *0x12ab67(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eb59:	48 8d 05 58 1d 12 00 	lea    0x121d58(%rip),%rax        # 1908b8 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x18>
   6eb60:	48 89 cf             	mov    %rcx,%rdi
   6eb63:	48 89 d6             	mov    %rdx,%rsi
   6eb66:	48 89 c2             	mov    %rax,%rdx
   6eb69:	ff 15 51 ab 12 00    	call   *0x12ab51(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eb6f:	48 8d 05 5a 1d 12 00 	lea    0x121d5a(%rip),%rax        # 1908d0 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x30>
   6eb76:	48 89 df             	mov    %rbx,%rdi
   6eb79:	48 89 d6             	mov    %rdx,%rsi
   6eb7c:	48 89 c2             	mov    %rax,%rdx
   6eb7f:	ff 15 3b ab 12 00    	call   *0x12ab3b(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eb85:	48 8d 05 5c 1d 12 00 	lea    0x121d5c(%rip),%rax        # 1908e8 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x48>
   6eb8c:	48 89 df             	mov    %rbx,%rdi
   6eb8f:	48 89 d6             	mov    %rdx,%rsi
   6eb92:	48 89 c2             	mov    %rax,%rdx
   6eb95:	ff 15 25 ab 12 00    	call   *0x12ab25(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6eb9b:	48 8d 05 5e 1d 12 00 	lea    0x121d5e(%rip),%rax        # 190900 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x60>
   6eba2:	48 89 df             	mov    %rbx,%rdi
   6eba5:	48 89 d6             	mov    %rdx,%rsi
   6eba8:	48 89 c2             	mov    %rax,%rdx
   6ebab:	ff 15 0f ab 12 00    	call   *0x12ab0f(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   6ebb1:	48 8d 05 60 1d 12 00 	lea    0x121d60(%rip),%rax        # 190918 <anon.6fe5b0f4a277a3de2c74ba95ea10c09d.43.llvm.8789668757449324569+0x78>
   6ebb8:	48 89 df             	mov    %rbx,%rdi
   6ebbb:	48 89 d6             	mov    %rdx,%rsi
   6ebbe:	48 89 c2             	mov    %rax,%rdx
   6ebc1:	ff 15 f9 aa 12 00    	call   *0x12aaf9(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
