
target/baseline-v18/verified-tiff/target/release/examples/benchmark:     file format elf64-x86-64


Disassembly of section .text:

000000000002e320 <<verified_tiff::deflate::Cursor>::step>:
   2e320:	55                   	push   %rbp
   2e321:	41 57                	push   %r15
   2e323:	41 56                	push   %r14
   2e325:	41 55                	push   %r13
   2e327:	41 54                	push   %r12
   2e329:	53                   	push   %rbx
   2e32a:	48 81 ec f8 09 00 00 	sub    $0x9f8,%rsp
   2e331:	0f b6 87 88 83 00 00 	movzbl 0x8388(%rdi),%eax
   2e338:	48 83 f8 03          	cmp    $0x3,%rax
   2e33c:	0f 87 7c 03 00 00    	ja     2e6be <<verified_tiff::deflate::Cursor>::step+0x39e>
   2e342:	4c 8d 05 0b c0 fd ff 	lea    -0x23ff5(%rip),%r8        # a354 <anon.ac36df6000eed6ce22b87f572999359a.13.llvm.17988912158883924058+0xc>
   2e349:	49 63 04 80          	movslq (%r8,%rax,4),%rax
   2e34d:	4c 01 c0             	add    %r8,%rax
   2e350:	ff e0                	jmp    *%rax
   2e352:	48 8b 8f 40 83 00 00 	mov    0x8340(%rdi),%rcx
   2e359:	4c 8b 8f 50 83 00 00 	mov    0x8350(%rdi),%r9
   2e360:	66 bd 02 00          	mov    $0x2,%bp
   2e364:	4c 39 c9             	cmp    %r9,%rcx
   2e367:	0f 84 6f 0d 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e36d:	48 39 d1             	cmp    %rdx,%rcx
   2e370:	0f 83 dd 0e 00 00    	jae    2f253 <<verified_tiff::deflate::Cursor>::step+0xf33>
   2e376:	48 8d 9f 40 83 00 00 	lea    0x8340(%rdi),%rbx
   2e37d:	4c 8b 87 48 83 00 00 	mov    0x8348(%rdi),%r8
   2e384:	49 83 f8 07          	cmp    $0x7,%r8
   2e388:	41 bc 07 00 00 00    	mov    $0x7,%r12d
   2e38e:	4d 0f 42 e0          	cmovb  %r8,%r12
   2e392:	0f 85 f1 03 00 00    	jne    2e789 <<verified_tiff::deflate::Cursor>::step+0x469>
   2e398:	48 8d 41 01          	lea    0x1(%rcx),%rax
   2e39c:	48 89 03             	mov    %rax,(%rbx)
   2e39f:	45 31 c0             	xor    %r8d,%r8d
   2e3a2:	44 0f b6 34 0e       	movzbl (%rsi,%rcx,1),%r14d
   2e3a7:	4c 89 87 48 83 00 00 	mov    %r8,0x8348(%rdi)
   2e3ae:	4c 39 c8             	cmp    %r9,%rax
   2e3b1:	0f 84 25 0d 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e3b7:	e9 e8 03 00 00       	jmp    2e7a4 <<verified_tiff::deflate::Cursor>::step+0x484>
   2e3bc:	48 89 4c 24 10       	mov    %rcx,0x10(%rsp)
   2e3c1:	48 8d 87 00 80 00 00 	lea    0x8000(%rdi),%rax
   2e3c8:	48 8d 4c 24 28       	lea    0x28(%rsp),%rcx
   2e3cd:	48 89 54 24 08       	mov    %rdx,0x8(%rsp)
   2e3d2:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2e3d7:	49 89 fc             	mov    %rdi,%r12
   2e3da:	48 89 cf             	mov    %rcx,%rdi
   2e3dd:	49 89 f6             	mov    %rsi,%r14
   2e3e0:	48 89 c6             	mov    %rax,%rsi
   2e3e3:	ff 15 37 b3 05 00    	call   *0x5b337(%rip)        # 89720 <memcpy@GLIBC_2.14>
   2e3e9:	4c 89 f1             	mov    %r14,%rcx
   2e3ec:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
   2e3f1:	4c 89 e0             	mov    %r12,%rax
   2e3f4:	4d 8d bc 24 40 83 00 	lea    0x8340(%r12),%r15
   2e3fb:	00 
   2e3fc:	49 8b 94 24 50 83 00 	mov    0x8350(%r12),%rdx
   2e403:	00 
   2e404:	49 8b bc 24 40 83 00 	mov    0x8340(%r12),%rdi
   2e40b:	00 
   2e40c:	4d 8b 8c 24 48 83 00 	mov    0x8348(%r12),%r9
   2e413:	00 
   2e414:	45 31 c0             	xor    %r8d,%r8d
   2e417:	45 31 ed             	xor    %r13d,%r13d
   2e41a:	45 31 d2             	xor    %r10d,%r10d
   2e41d:	eb 18                	jmp    2e437 <<verified_tiff::deflate::Cursor>::step+0x117>
   2e41f:	90                   	nop
   2e420:	4d 01 d3             	add    %r10,%r11
   2e423:	4d 01 db             	add    %r11,%r11
   2e426:	49 83 c5 08          	add    $0x8,%r13
   2e42a:	4d 89 da             	mov    %r11,%r10
   2e42d:	49 83 fd 78          	cmp    $0x78,%r13
   2e431:	0f 84 2e 03 00 00    	je     2e765 <<verified_tiff::deflate::Cursor>::step+0x445>
   2e437:	48 39 d7             	cmp    %rdx,%rdi
   2e43a:	0f 84 3d 03 00 00    	je     2e77d <<verified_tiff::deflate::Cursor>::step+0x45d>
   2e440:	48 39 f7             	cmp    %rsi,%rdi
   2e443:	0f 83 ea 0d 00 00    	jae    2f233 <<verified_tiff::deflate::Cursor>::step+0xf13>
   2e449:	0f b6 2c 39          	movzbl (%rcx,%rdi,1),%ebp
   2e44d:	49 83 f9 07          	cmp    $0x7,%r9
   2e451:	bb 07 00 00 00       	mov    $0x7,%ebx
   2e456:	49 0f 42 d9          	cmovb  %r9,%rbx
   2e45a:	45 31 db             	xor    %r11d,%r11d
   2e45d:	48 0f a3 dd          	bt     %rbx,%rbp
   2e461:	40 0f 92 c5          	setb   %bpl
   2e465:	49 83 f9 07          	cmp    $0x7,%r9
   2e469:	74 05                	je     2e470 <<verified_tiff::deflate::Cursor>::step+0x150>
   2e46b:	49 ff c1             	inc    %r9
   2e46e:	eb 09                	jmp    2e479 <<verified_tiff::deflate::Cursor>::step+0x159>
   2e470:	48 ff c7             	inc    %rdi
   2e473:	49 89 3f             	mov    %rdi,(%r15)
   2e476:	45 31 c9             	xor    %r9d,%r9d
   2e479:	41 88 eb             	mov    %bpl,%r11b
   2e47c:	4c 89 88 48 83 00 00 	mov    %r9,0x8348(%rax)
   2e483:	4f 8d 04 43          	lea    (%r11,%r8,2),%r8
   2e487:	4e 8b 5c 2c 30       	mov    0x30(%rsp,%r13,1),%r11
   2e48c:	4c 89 c5             	mov    %r8,%rbp
   2e48f:	4c 29 d5             	sub    %r10,%rbp
   2e492:	72 8c                	jb     2e420 <<verified_tiff::deflate::Cursor>::step+0x100>
   2e494:	4c 39 dd             	cmp    %r11,%rbp
   2e497:	73 87                	jae    2e420 <<verified_tiff::deflate::Cursor>::step+0x100>
   2e499:	48 8d b4 24 a8 00 00 	lea    0xa8(%rsp),%rsi
   2e4a0:	00 
   2e4a1:	48 8d bc 24 70 03 00 	lea    0x370(%rsp),%rdi
   2e4a8:	00 
   2e4a9:	ba 20 01 00 00       	mov    $0x120,%edx
   2e4ae:	ff 15 6c b2 05 00    	call   *0x5b26c(%rip)        # 89720 <memcpy@GLIBC_2.14>
   2e4b4:	4c 89 e7             	mov    %r12,%rdi
   2e4b7:	be 02 00 00 00       	mov    $0x2,%esi
   2e4bc:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   2e4c1:	eb 1e                	jmp    2e4e1 <<verified_tiff::deflate::Cursor>::step+0x1c1>
   2e4c3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2e4ca:	84 00 00 00 00 00 
   2e4d0:	48 83 c6 03          	add    $0x3,%rsi
   2e4d4:	48 81 fe 22 01 00 00 	cmp    $0x122,%rsi
   2e4db:	0f 84 84 02 00 00    	je     2e765 <<verified_tiff::deflate::Cursor>::step+0x445>
   2e4e1:	0f b6 8c 34 6e 03 00 	movzbl 0x36e(%rsp,%rsi,1),%ecx
   2e4e8:	00 
   2e4e9:	48 8d 0c cd f8 ff ff 	lea    -0x8(,%rcx,8),%rcx
   2e4f0:	ff 
   2e4f1:	4c 39 e9             	cmp    %r13,%rcx
   2e4f4:	75 0c                	jne    2e502 <<verified_tiff::deflate::Cursor>::step+0x1e2>
   2e4f6:	48 85 ed             	test   %rbp,%rbp
   2e4f9:	0f 84 e9 03 00 00    	je     2e8e8 <<verified_tiff::deflate::Cursor>::step+0x5c8>
   2e4ff:	48 ff cd             	dec    %rbp
   2e502:	0f b6 8c 34 6f 03 00 	movzbl 0x36f(%rsp,%rsi,1),%ecx
   2e509:	00 
   2e50a:	48 8d 0c cd f8 ff ff 	lea    -0x8(,%rcx,8),%rcx
   2e511:	ff 
   2e512:	4c 39 e9             	cmp    %r13,%rcx
   2e515:	75 0c                	jne    2e523 <<verified_tiff::deflate::Cursor>::step+0x203>
   2e517:	48 85 ed             	test   %rbp,%rbp
   2e51a:	0f 84 ce 03 00 00    	je     2e8ee <<verified_tiff::deflate::Cursor>::step+0x5ce>
   2e520:	48 ff cd             	dec    %rbp
   2e523:	0f b6 8c 34 70 03 00 	movzbl 0x370(%rsp,%rsi,1),%ecx
   2e52a:	00 
   2e52b:	48 8d 0c cd f8 ff ff 	lea    -0x8(,%rcx,8),%rcx
   2e532:	ff 
   2e533:	4c 39 e9             	cmp    %r13,%rcx
   2e536:	75 98                	jne    2e4d0 <<verified_tiff::deflate::Cursor>::step+0x1b0>
   2e538:	48 85 ed             	test   %rbp,%rbp
   2e53b:	0f 84 b0 03 00 00    	je     2e8f1 <<verified_tiff::deflate::Cursor>::step+0x5d1>
   2e541:	48 ff cd             	dec    %rbp
   2e544:	eb 8a                	jmp    2e4d0 <<verified_tiff::deflate::Cursor>::step+0x1b0>
   2e546:	48 8b 87 58 83 00 00 	mov    0x8358(%rdi),%rax
   2e54d:	48 85 c0             	test   %rax,%rax
   2e550:	0f 84 1b 02 00 00    	je     2e771 <<verified_tiff::deflate::Cursor>::step+0x451>
   2e556:	48 8b 97 60 83 00 00 	mov    0x8360(%rdi),%rdx
   2e55d:	66 bd 02 00          	mov    $0x2,%bp
   2e561:	41 b5 01             	mov    $0x1,%r13b
   2e564:	48 85 d2             	test   %rdx,%rdx
   2e567:	0f 84 72 0b 00 00    	je     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e56d:	48 8b b7 68 83 00 00 	mov    0x8368(%rdi),%rsi
   2e574:	48 39 d6             	cmp    %rdx,%rsi
   2e577:	0f 82 62 0b 00 00    	jb     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e57d:	48 3b 97 70 83 00 00 	cmp    0x8370(%rdi),%rdx
   2e584:	0f 87 55 0b 00 00    	ja     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e58a:	41 89 f0             	mov    %esi,%r8d
   2e58d:	41 29 d0             	sub    %edx,%r8d
   2e590:	41 81 e0 ff 7f 00 00 	and    $0x7fff,%r8d
   2e597:	46 0f b6 04 07       	movzbl (%rdi,%r8,1),%r8d
   2e59c:	48 ff c8             	dec    %rax
   2e59f:	48 89 87 58 83 00 00 	mov    %rax,0x8358(%rdi)
   2e5a6:	48 39 ce             	cmp    %rcx,%rsi
   2e5a9:	0f 84 30 0b 00 00    	je     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e5af:	41 0f b6 c0          	movzbl %r8b,%eax
   2e5b3:	89 f1                	mov    %esi,%ecx
   2e5b5:	81 e1 ff 7f 00 00    	and    $0x7fff,%ecx
   2e5bb:	88 04 0f             	mov    %al,(%rdi,%rcx,1)
   2e5be:	48 8b 8f 78 83 00 00 	mov    0x8378(%rdi),%rcx
   2e5c5:	48 01 c1             	add    %rax,%rcx
   2e5c8:	49 b9 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r9
   2e5cf:	00 0f 00 
   2e5d2:	48 89 c8             	mov    %rcx,%rax
   2e5d5:	49 f7 e1             	mul    %r9
   2e5d8:	48 89 c8             	mov    %rcx,%rax
   2e5db:	48 29 d0             	sub    %rdx,%rax
   2e5de:	48 d1 e8             	shr    $1,%rax
   2e5e1:	48 01 d0             	add    %rdx,%rax
   2e5e4:	48 c1 e8 0f          	shr    $0xf,%rax
   2e5e8:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2e5ef:	48 29 c1             	sub    %rax,%rcx
   2e5f2:	48 89 8f 78 83 00 00 	mov    %rcx,0x8378(%rdi)
   2e5f9:	48 03 8f 80 83 00 00 	add    0x8380(%rdi),%rcx
   2e600:	48 89 c8             	mov    %rcx,%rax
   2e603:	49 f7 e1             	mul    %r9
   2e606:	48 89 c8             	mov    %rcx,%rax
   2e609:	48 29 d0             	sub    %rdx,%rax
   2e60c:	48 d1 e8             	shr    $1,%rax
   2e60f:	48 01 d0             	add    %rdx,%rax
   2e612:	48 c1 e8 0f          	shr    $0xf,%rax
   2e616:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2e61d:	48 29 c1             	sub    %rax,%rcx
   2e620:	48 89 8f 80 83 00 00 	mov    %rcx,0x8380(%rdi)
   2e627:	48 ff c6             	inc    %rsi
   2e62a:	48 89 b7 68 83 00 00 	mov    %rsi,0x8368(%rdi)
   2e631:	66 bd 01 00          	mov    $0x1,%bp
   2e635:	45 89 c5             	mov    %r8d,%r13d
   2e638:	e9 a2 0a 00 00       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e63d:	4c 8b 87 58 83 00 00 	mov    0x8358(%rdi),%r8
   2e644:	4d 85 c0             	test   %r8,%r8
   2e647:	0f 84 cf 02 00 00    	je     2e91c <<verified_tiff::deflate::Cursor>::step+0x5fc>
   2e64d:	48 8b 87 40 83 00 00 	mov    0x8340(%rdi),%rax
   2e654:	4c 8b 9f 50 83 00 00 	mov    0x8350(%rdi),%r11
   2e65b:	66 bd 02 00          	mov    $0x2,%bp
   2e65f:	4c 39 d8             	cmp    %r11,%rax
   2e662:	0f 84 74 0a 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e668:	48 39 d0             	cmp    %rdx,%rax
   2e66b:	0f 83 cf 0b 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2e671:	48 8b 9f 48 83 00 00 	mov    0x8348(%rdi),%rbx
   2e678:	48 83 fb 07          	cmp    $0x7,%rbx
   2e67c:	41 b9 07 00 00 00    	mov    $0x7,%r9d
   2e682:	4c 0f 42 cb          	cmovb  %rbx,%r9
   2e686:	44 0f b6 14 06       	movzbl (%rsi,%rax,1),%r10d
   2e68b:	45 0f a3 ca          	bt     %r9d,%r10d
   2e68f:	41 0f 92 c1          	setb   %r9b
   2e693:	48 83 fb 07          	cmp    $0x7,%rbx
   2e697:	0f 85 4b 01 00 00    	jne    2e7e8 <<verified_tiff::deflate::Cursor>::step+0x4c8>
   2e69d:	48 ff c0             	inc    %rax
   2e6a0:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2e6a7:	31 db                	xor    %ebx,%ebx
   2e6a9:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2e6b0:	4c 39 d8             	cmp    %r11,%rax
   2e6b3:	0f 84 23 0a 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e6b9:	e9 3d 01 00 00       	jmp    2e7fb <<verified_tiff::deflate::Cursor>::step+0x4db>
   2e6be:	48 8b 8f 40 83 00 00 	mov    0x8340(%rdi),%rcx
   2e6c5:	48 8b 87 50 83 00 00 	mov    0x8350(%rdi),%rax
   2e6cc:	49 89 c0             	mov    %rax,%r8
   2e6cf:	66 bd 02 00          	mov    $0x2,%bp
   2e6d3:	41 b5 01             	mov    $0x1,%r13b
   2e6d6:	49 29 c8             	sub    %rcx,%r8
   2e6d9:	74 0a                	je     2e6e5 <<verified_tiff::deflate::Cursor>::step+0x3c5>
   2e6db:	49 83 f8 01          	cmp    $0x1,%r8
   2e6df:	0f 85 fa 09 00 00    	jne    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e6e5:	48 39 c8             	cmp    %rcx,%rax
   2e6e8:	74 0e                	je     2e6f8 <<verified_tiff::deflate::Cursor>::step+0x3d8>
   2e6ea:	48 83 bf 48 83 00 00 	cmpq   $0x0,0x8348(%rdi)
   2e6f1:	00 
   2e6f2:	0f 84 e7 09 00 00    	je     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e6f8:	48 89 d1             	mov    %rdx,%rcx
   2e6fb:	48 29 c1             	sub    %rax,%rcx
   2e6fe:	41 0f 92 c0          	setb   %r8b
   2e702:	48 83 f9 04          	cmp    $0x4,%rcx
   2e706:	0f 92 c1             	setb   %cl
   2e709:	44 08 c1             	or     %r8b,%cl
   2e70c:	0f 85 ca 09 00 00    	jne    2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e712:	49 89 d0             	mov    %rdx,%r8
   2e715:	49 29 c0             	sub    %rax,%r8
   2e718:	0f 86 4b 0b 00 00    	jbe    2f269 <<verified_tiff::deflate::Cursor>::step+0xf49>
   2e71e:	48 8d 48 01          	lea    0x1(%rax),%rcx
   2e722:	48 39 d1             	cmp    %rdx,%rcx
   2e725:	0f 83 54 0b 00 00    	jae    2f27f <<verified_tiff::deflate::Cursor>::step+0xf5f>
   2e72b:	49 83 c0 fe          	add    $0xfffffffffffffffe,%r8
   2e72f:	49 83 f8 01          	cmp    $0x1,%r8
   2e733:	0f 86 a3 09 00 00    	jbe    2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e739:	8b 04 06             	mov    (%rsi,%rax,1),%eax
   2e73c:	0f c8                	bswap  %eax
   2e73e:	48 8b 8f 80 83 00 00 	mov    0x8380(%rdi),%rcx
   2e745:	48 c1 e1 10          	shl    $0x10,%rcx
   2e749:	48 03 8f 78 83 00 00 	add    0x8378(%rdi),%rcx
   2e750:	48 39 c8             	cmp    %rcx,%rax
   2e753:	0f 85 86 09 00 00    	jne    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e759:	c6 87 88 83 00 00 05 	movb   $0x5,0x8388(%rdi)
   2e760:	e9 c7 01 00 00       	jmp    2e92c <<verified_tiff::deflate::Cursor>::step+0x60c>
   2e765:	41 b5 01             	mov    $0x1,%r13b
   2e768:	66 bd 02 00          	mov    $0x2,%bp
   2e76c:	e9 6e 09 00 00       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e771:	c6 87 88 83 00 00 02 	movb   $0x2,0x8388(%rdi)
   2e778:	e9 af 01 00 00       	jmp    2e92c <<verified_tiff::deflate::Cursor>::step+0x60c>
   2e77d:	45 31 ed             	xor    %r13d,%r13d
   2e780:	66 bd 02 00          	mov    $0x2,%bp
   2e784:	e9 56 09 00 00       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e789:	49 ff c0             	inc    %r8
   2e78c:	48 89 c8             	mov    %rcx,%rax
   2e78f:	44 0f b6 34 0e       	movzbl (%rsi,%rcx,1),%r14d
   2e794:	4c 89 87 48 83 00 00 	mov    %r8,0x8348(%rdi)
   2e79b:	4c 39 c8             	cmp    %r9,%rax
   2e79e:	0f 84 38 09 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e7a4:	48 39 d0             	cmp    %rdx,%rax
   2e7a7:	0f 83 93 0a 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2e7ad:	49 83 f8 07          	cmp    $0x7,%r8
   2e7b1:	b9 07 00 00 00       	mov    $0x7,%ecx
   2e7b6:	49 0f 42 c8          	cmovb  %r8,%rcx
   2e7ba:	44 0f b6 14 06       	movzbl (%rsi,%rax,1),%r10d
   2e7bf:	41 0f a3 ca          	bt     %ecx,%r10d
   2e7c3:	41 0f 92 c2          	setb   %r10b
   2e7c7:	49 83 f8 07          	cmp    $0x7,%r8
   2e7cb:	75 63                	jne    2e830 <<verified_tiff::deflate::Cursor>::step+0x510>
   2e7cd:	48 ff c0             	inc    %rax
   2e7d0:	48 89 03             	mov    %rax,(%rbx)
   2e7d3:	45 31 c0             	xor    %r8d,%r8d
   2e7d6:	4c 89 87 48 83 00 00 	mov    %r8,0x8348(%rdi)
   2e7dd:	4c 39 c8             	cmp    %r9,%rax
   2e7e0:	0f 84 f6 08 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e7e6:	eb 5b                	jmp    2e843 <<verified_tiff::deflate::Cursor>::step+0x523>
   2e7e8:	48 ff c3             	inc    %rbx
   2e7eb:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2e7f2:	4c 39 d8             	cmp    %r11,%rax
   2e7f5:	0f 84 e1 08 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e7fb:	48 39 d0             	cmp    %rdx,%rax
   2e7fe:	0f 83 3c 0a 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2e804:	49 89 ce             	mov    %rcx,%r14
   2e807:	44 0f b6 14 06       	movzbl (%rsi,%rax,1),%r10d
   2e80c:	48 83 fb 07          	cmp    $0x7,%rbx
   2e810:	b9 07 00 00 00       	mov    $0x7,%ecx
   2e815:	48 0f 42 cb          	cmovb  %rbx,%rcx
   2e819:	41 d2 ea             	shr    %cl,%r10b
   2e81c:	48 83 fb 07          	cmp    $0x7,%rbx
   2e820:	75 7a                	jne    2e89c <<verified_tiff::deflate::Cursor>::step+0x57c>
   2e822:	48 ff c0             	inc    %rax
   2e825:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2e82c:	31 db                	xor    %ebx,%ebx
   2e82e:	eb 6f                	jmp    2e89f <<verified_tiff::deflate::Cursor>::step+0x57f>
   2e830:	49 ff c0             	inc    %r8
   2e833:	4c 89 87 48 83 00 00 	mov    %r8,0x8348(%rdi)
   2e83a:	4c 39 c8             	cmp    %r9,%rax
   2e83d:	0f 84 99 08 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e843:	48 39 d0             	cmp    %rdx,%rax
   2e846:	0f 83 f4 09 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2e84c:	44 0f b6 0c 06       	movzbl (%rsi,%rax,1),%r9d
   2e851:	49 83 f8 07          	cmp    $0x7,%r8
   2e855:	b9 07 00 00 00       	mov    $0x7,%ecx
   2e85a:	49 0f 42 c8          	cmovb  %r8,%rcx
   2e85e:	41 d2 e9             	shr    %cl,%r9b
   2e861:	45 00 c9             	add    %r9b,%r9b
   2e864:	41 80 e1 02          	and    $0x2,%r9b
   2e868:	45 08 d1             	or     %r10b,%r9b
   2e86b:	49 83 f8 07          	cmp    $0x7,%r8
   2e86f:	0f 85 be 00 00 00    	jne    2e933 <<verified_tiff::deflate::Cursor>::step+0x613>
   2e875:	48 ff c0             	inc    %rax
   2e878:	48 89 03             	mov    %rax,(%rbx)
   2e87b:	45 31 c0             	xor    %r8d,%r8d
   2e87e:	4c 89 87 48 83 00 00 	mov    %r8,0x8348(%rdi)
   2e885:	41 b5 01             	mov    $0x1,%r13b
   2e888:	41 0f b6 c9          	movzbl %r9b,%ecx
   2e88c:	4c 8d 0d d1 ba fd ff 	lea    -0x2452f(%rip),%r9        # a364 <anon.ac36df6000eed6ce22b87f572999359a.13.llvm.17988912158883924058+0x1c>
   2e893:	49 63 0c 89          	movslq (%r9,%rcx,4),%rcx
   2e897:	4c 01 c9             	add    %r9,%rcx
   2e89a:	ff e1                	jmp    *%rcx
   2e89c:	48 ff c3             	inc    %rbx
   2e89f:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2e8a6:	4c 39 d8             	cmp    %r11,%rax
   2e8a9:	0f 84 2d 08 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2e8af:	48 39 d0             	cmp    %rdx,%rax
   2e8b2:	0f 83 88 09 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2e8b8:	44 0f b6 3c 06       	movzbl (%rsi,%rax,1),%r15d
   2e8bd:	48 83 fb 07          	cmp    $0x7,%rbx
   2e8c1:	b9 07 00 00 00       	mov    $0x7,%ecx
   2e8c6:	48 0f 42 cb          	cmovb  %rbx,%rcx
   2e8ca:	41 d2 ef             	shr    %cl,%r15b
   2e8cd:	48 83 fb 07          	cmp    $0x7,%rbx
   2e8d1:	0f 85 a4 03 00 00    	jne    2ec7b <<verified_tiff::deflate::Cursor>::step+0x95b>
   2e8d7:	48 ff c0             	inc    %rax
   2e8da:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2e8e1:	31 db                	xor    %ebx,%ebx
   2e8e3:	e9 96 03 00 00       	jmp    2ec7e <<verified_tiff::deflate::Cursor>::step+0x95e>
   2e8e8:	48 83 c6 fe          	add    $0xfffffffffffffffe,%rsi
   2e8ec:	eb 03                	jmp    2e8f1 <<verified_tiff::deflate::Cursor>::step+0x5d1>
   2e8ee:	48 ff ce             	dec    %rsi
   2e8f1:	48 81 fe 00 01 00 00 	cmp    $0x100,%rsi
   2e8f8:	73 1c                	jae    2e916 <<verified_tiff::deflate::Cursor>::step+0x5f6>
   2e8fa:	4c 8b 87 68 83 00 00 	mov    0x8368(%rdi),%r8
   2e901:	49 39 c0             	cmp    %rax,%r8
   2e904:	0f 85 f0 00 00 00    	jne    2e9fa <<verified_tiff::deflate::Cursor>::step+0x6da>
   2e90a:	66 bd 02 00          	mov    $0x2,%bp
   2e90e:	41 b5 01             	mov    $0x1,%r13b
   2e911:	e9 c9 07 00 00       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e916:	0f 85 69 01 00 00    	jne    2ea85 <<verified_tiff::deflate::Cursor>::step+0x765>
   2e91c:	0f b6 87 89 83 00 00 	movzbl 0x8389(%rdi),%eax
   2e923:	c0 e0 02             	shl    $0x2,%al
   2e926:	88 87 88 83 00 00    	mov    %al,0x8388(%rdi)
   2e92c:	31 ed                	xor    %ebp,%ebp
   2e92e:	e9 ac 07 00 00       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e933:	49 ff c0             	inc    %r8
   2e936:	4c 89 87 48 83 00 00 	mov    %r8,0x8348(%rdi)
   2e93d:	41 b5 01             	mov    $0x1,%r13b
   2e940:	41 0f b6 c9          	movzbl %r9b,%ecx
   2e944:	4c 8d 0d 19 ba fd ff 	lea    -0x245e7(%rip),%r9        # a364 <anon.ac36df6000eed6ce22b87f572999359a.13.llvm.17988912158883924058+0x1c>
   2e94b:	49 63 0c 89          	movslq (%r9,%rcx,4),%rcx
   2e94f:	4c 01 c9             	add    %r9,%rcx
   2e952:	ff e1                	jmp    *%rcx
   2e954:	49 89 f7             	mov    %rsi,%r15
   2e957:	49 89 d5             	mov    %rdx,%r13
   2e95a:	4d 85 c0             	test   %r8,%r8
   2e95d:	74 15                	je     2e974 <<verified_tiff::deflate::Cursor>::step+0x654>
   2e95f:	48 ff c0             	inc    %rax
   2e962:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2e969:	48 c7 87 48 83 00 00 	movq   $0x0,0x8348(%rdi)
   2e970:	00 00 00 00 
   2e974:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
   2e979:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   2e97e:	41 b8 10 00 00 00    	mov    $0x10,%r8d
   2e984:	48 89 de             	mov    %rbx,%rsi
   2e987:	4c 89 fa             	mov    %r15,%rdx
   2e98a:	4c 89 e9             	mov    %r13,%rcx
   2e98d:	e8 de f8 ff ff       	call   2e270 <<verified_tiff::deflate_bits::Bits>::read>
   2e992:	80 7c 24 28 01       	cmpb   $0x1,0x28(%rsp)
   2e997:	0f 84 b8 03 00 00    	je     2ed55 <<verified_tiff::deflate::Cursor>::step+0xa35>
   2e99d:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   2e9a2:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
   2e9a7:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   2e9ac:	41 b8 10 00 00 00    	mov    $0x10,%r8d
   2e9b2:	48 89 de             	mov    %rbx,%rsi
   2e9b5:	4c 89 fa             	mov    %r15,%rdx
   2e9b8:	4c 89 e9             	mov    %r13,%rcx
   2e9bb:	e8 b0 f8 ff ff       	call   2e270 <<verified_tiff::deflate_bits::Bits>::read>
   2e9c0:	80 7c 24 28 01       	cmpb   $0x1,0x28(%rsp)
   2e9c5:	0f 84 8a 03 00 00    	je     2ed55 <<verified_tiff::deflate::Cursor>::step+0xa35>
   2e9cb:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   2e9d0:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
   2e9d5:	48 01 c8             	add    %rcx,%rax
   2e9d8:	41 b5 01             	mov    $0x1,%r13b
   2e9db:	48 3d ff ff 00 00    	cmp    $0xffff,%rax
   2e9e1:	4c 8b 7c 24 08       	mov    0x8(%rsp),%r15
   2e9e6:	0f 85 f3 06 00 00    	jne    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2e9ec:	49 89 8f 58 83 00 00 	mov    %rcx,0x8358(%r15)
   2e9f3:	b1 01                	mov    $0x1,%cl
   2e9f5:	e9 28 04 00 00       	jmp    2ee22 <<verified_tiff::deflate::Cursor>::step+0xb02>
   2e9fa:	44 89 c0             	mov    %r8d,%eax
   2e9fd:	25 ff 7f 00 00       	and    $0x7fff,%eax
   2ea02:	40 88 34 07          	mov    %sil,(%rdi,%rax,1)
   2ea06:	48 89 f1             	mov    %rsi,%rcx
   2ea09:	48 03 8f 78 83 00 00 	add    0x8378(%rdi),%rcx
   2ea10:	49 b9 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r9
   2ea17:	00 0f 00 
   2ea1a:	48 89 c8             	mov    %rcx,%rax
   2ea1d:	49 f7 e1             	mul    %r9
   2ea20:	48 89 c8             	mov    %rcx,%rax
   2ea23:	48 29 d0             	sub    %rdx,%rax
   2ea26:	48 d1 e8             	shr    $1,%rax
   2ea29:	48 01 d0             	add    %rdx,%rax
   2ea2c:	48 c1 e8 0f          	shr    $0xf,%rax
   2ea30:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2ea37:	48 29 c1             	sub    %rax,%rcx
   2ea3a:	48 89 8f 78 83 00 00 	mov    %rcx,0x8378(%rdi)
   2ea41:	48 03 8f 80 83 00 00 	add    0x8380(%rdi),%rcx
   2ea48:	48 89 c8             	mov    %rcx,%rax
   2ea4b:	49 f7 e1             	mul    %r9
   2ea4e:	48 89 c8             	mov    %rcx,%rax
   2ea51:	48 29 d0             	sub    %rdx,%rax
   2ea54:	48 d1 e8             	shr    $1,%rax
   2ea57:	48 01 d0             	add    %rdx,%rax
   2ea5a:	48 c1 e8 0f          	shr    $0xf,%rax
   2ea5e:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2ea65:	48 29 c1             	sub    %rax,%rcx
   2ea68:	48 89 8f 80 83 00 00 	mov    %rcx,0x8380(%rdi)
   2ea6f:	49 ff c0             	inc    %r8
   2ea72:	4c 89 87 68 83 00 00 	mov    %r8,0x8368(%rdi)
   2ea79:	66 bd 01 00          	mov    $0x1,%bp
   2ea7d:	41 89 f5             	mov    %esi,%r13d
   2ea80:	e9 5a 06 00 00       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2ea85:	66 bd 02 00          	mov    $0x2,%bp
   2ea89:	41 b5 01             	mov    $0x1,%r13b
   2ea8c:	48 81 fe 1d 01 00 00 	cmp    $0x11d,%rsi
   2ea93:	0f 87 46 06 00 00    	ja     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2ea99:	48 c7 44 24 28 03 00 	movq   $0x3,0x28(%rsp)
   2eaa0:	00 00 
   2eaa2:	48 c7 44 24 30 04 00 	movq   $0x4,0x30(%rsp)
   2eaa9:	00 00 
   2eaab:	48 c7 44 24 38 05 00 	movq   $0x5,0x38(%rsp)
   2eab2:	00 00 
   2eab4:	48 c7 44 24 40 06 00 	movq   $0x6,0x40(%rsp)
   2eabb:	00 00 
   2eabd:	48 c7 44 24 48 07 00 	movq   $0x7,0x48(%rsp)
   2eac4:	00 00 
   2eac6:	48 c7 44 24 50 08 00 	movq   $0x8,0x50(%rsp)
   2eacd:	00 00 
   2eacf:	48 c7 44 24 58 09 00 	movq   $0x9,0x58(%rsp)
   2ead6:	00 00 
   2ead8:	48 c7 44 24 60 0a 00 	movq   $0xa,0x60(%rsp)
   2eadf:	00 00 
   2eae1:	48 c7 44 24 68 0b 00 	movq   $0xb,0x68(%rsp)
   2eae8:	00 00 
   2eaea:	48 c7 44 24 70 0d 00 	movq   $0xd,0x70(%rsp)
   2eaf1:	00 00 
   2eaf3:	48 c7 44 24 78 0f 00 	movq   $0xf,0x78(%rsp)
   2eafa:	00 00 
   2eafc:	48 c7 84 24 80 00 00 	movq   $0x11,0x80(%rsp)
   2eb03:	00 11 00 00 00 
   2eb08:	48 c7 84 24 88 00 00 	movq   $0x13,0x88(%rsp)
   2eb0f:	00 13 00 00 00 
   2eb14:	48 c7 84 24 90 00 00 	movq   $0x17,0x90(%rsp)
   2eb1b:	00 17 00 00 00 
   2eb20:	48 c7 84 24 98 00 00 	movq   $0x1b,0x98(%rsp)
   2eb27:	00 1b 00 00 00 
   2eb2c:	48 c7 84 24 a0 00 00 	movq   $0x1f,0xa0(%rsp)
   2eb33:	00 1f 00 00 00 
   2eb38:	48 c7 84 24 a8 00 00 	movq   $0x23,0xa8(%rsp)
   2eb3f:	00 23 00 00 00 
   2eb44:	48 c7 84 24 b0 00 00 	movq   $0x2b,0xb0(%rsp)
   2eb4b:	00 2b 00 00 00 
   2eb50:	48 c7 84 24 b8 00 00 	movq   $0x33,0xb8(%rsp)
   2eb57:	00 33 00 00 00 
   2eb5c:	48 c7 84 24 c0 00 00 	movq   $0x3b,0xc0(%rsp)
   2eb63:	00 3b 00 00 00 
   2eb68:	48 c7 84 24 c8 00 00 	movq   $0x43,0xc8(%rsp)
   2eb6f:	00 43 00 00 00 
   2eb74:	48 c7 84 24 d0 00 00 	movq   $0x53,0xd0(%rsp)
   2eb7b:	00 53 00 00 00 
   2eb80:	48 c7 84 24 d8 00 00 	movq   $0x63,0xd8(%rsp)
   2eb87:	00 63 00 00 00 
   2eb8c:	48 c7 84 24 e0 00 00 	movq   $0x73,0xe0(%rsp)
   2eb93:	00 73 00 00 00 
   2eb98:	48 c7 84 24 e8 00 00 	movq   $0x83,0xe8(%rsp)
   2eb9f:	00 83 00 00 00 
   2eba4:	48 c7 84 24 f0 00 00 	movq   $0xa3,0xf0(%rsp)
   2ebab:	00 a3 00 00 00 
   2ebb0:	48 c7 84 24 f8 00 00 	movq   $0xc3,0xf8(%rsp)
   2ebb7:	00 c3 00 00 00 
   2ebbc:	48 c7 84 24 00 01 00 	movq   $0xe3,0x100(%rsp)
   2ebc3:	00 e3 00 00 00 
   2ebc8:	48 c7 84 24 08 01 00 	movq   $0x102,0x108(%rsp)
   2ebcf:	00 02 01 00 00 
   2ebd4:	4c 8d 86 fb fe ff ff 	lea    -0x105(%rsi),%r8
   2ebdb:	49 c1 e8 02          	shr    $0x2,%r8
   2ebdf:	31 c0                	xor    %eax,%eax
   2ebe1:	48 81 fe 1d 01 00 00 	cmp    $0x11d,%rsi
   2ebe8:	4c 0f 44 c0          	cmove  %rax,%r8
   2ebec:	48 8b 9c f4 20 f8 ff 	mov    -0x7e0(%rsp,%rsi,8),%rbx
   2ebf3:	ff 
   2ebf4:	48 81 fe 09 01 00 00 	cmp    $0x109,%rsi
   2ebfb:	4c 0f 42 c0          	cmovb  %rax,%r8
   2ebff:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   2ec04:	4c 89 fe             	mov    %r15,%rsi
   2ec07:	4c 89 f2             	mov    %r14,%rdx
   2ec0a:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2ec0f:	e8 5c f6 ff ff       	call   2e270 <<verified_tiff::deflate_bits::Bits>::read>
   2ec14:	80 7c 24 28 01       	cmpb   $0x1,0x28(%rsp)
   2ec19:	0f 84 36 01 00 00    	je     2ed55 <<verified_tiff::deflate::Cursor>::step+0xa35>
   2ec1f:	48 03 5c 24 30       	add    0x30(%rsp),%rbx
   2ec24:	48 89 5c 24 18       	mov    %rbx,0x18(%rsp)
   2ec29:	49 8d b4 24 a0 81 00 	lea    0x81a0(%r12),%rsi
   2ec30:	00 
   2ec31:	48 8d 5c 24 28       	lea    0x28(%rsp),%rbx
   2ec36:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2ec3b:	48 89 df             	mov    %rbx,%rdi
   2ec3e:	ff 15 dc aa 05 00    	call   *0x5aadc(%rip)        # 89720 <memcpy@GLIBC_2.14>
   2ec44:	48 8d bc 24 70 03 00 	lea    0x370(%rsp),%rdi
   2ec4b:	00 
   2ec4c:	48 89 de             	mov    %rbx,%rsi
   2ec4f:	4c 89 f2             	mov    %r14,%rdx
   2ec52:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2ec57:	4d 89 f8             	mov    %r15,%r8
   2ec5a:	e8 41 67 00 00       	call   353a0 <<verified_tiff::deflate_bits::Huff>::read>
   2ec5f:	80 bc 24 70 03 00 00 	cmpb   $0x1,0x370(%rsp)
   2ec66:	01 
   2ec67:	0f 85 2a 02 00 00    	jne    2ee97 <<verified_tiff::deflate::Cursor>::step+0xb77>
   2ec6d:	44 0f b6 ac 24 71 03 	movzbl 0x371(%rsp),%r13d
   2ec74:	00 00 
   2ec76:	e9 64 04 00 00       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2ec7b:	48 ff c3             	inc    %rbx
   2ec7e:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2ec85:	4c 39 d8             	cmp    %r11,%rax
   2ec88:	0f 84 4e 04 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2ec8e:	48 39 d0             	cmp    %rdx,%rax
   2ec91:	0f 83 a9 05 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2ec97:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   2ec9c:	48 83 fb 07          	cmp    $0x7,%rbx
   2eca0:	b9 07 00 00 00       	mov    $0x7,%ecx
   2eca5:	48 0f 42 cb          	cmovb  %rbx,%rcx
   2eca9:	41 d2 ec             	shr    %cl,%r12b
   2ecac:	48 83 fb 07          	cmp    $0x7,%rbx
   2ecb0:	0f 85 aa 00 00 00    	jne    2ed60 <<verified_tiff::deflate::Cursor>::step+0xa40>
   2ecb6:	48 ff c0             	inc    %rax
   2ecb9:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2ecc0:	31 db                	xor    %ebx,%ebx
   2ecc2:	e9 9c 00 00 00       	jmp    2ed63 <<verified_tiff::deflate::Cursor>::step+0xa43>
   2ecc7:	49 89 ff             	mov    %rdi,%r15
   2ecca:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   2eccf:	31 f6                	xor    %esi,%esi
   2ecd1:	e8 8a 40 00 00       	call   32d60 <verified_tiff::deflate_header::fixed>
   2ecd6:	80 7c 24 28 01       	cmpb   $0x1,0x28(%rsp)
   2ecdb:	74 78                	je     2ed55 <<verified_tiff::deflate::Cursor>::step+0xa35>
   2ecdd:	48 8d 74 24 30       	lea    0x30(%rsp),%rsi
   2ece2:	48 8d 9c 24 76 03 00 	lea    0x376(%rsp),%rbx
   2ece9:	00 
   2ecea:	4c 8b 2d 2f aa 05 00 	mov    0x5aa2f(%rip),%r13        # 89720 <memcpy@GLIBC_2.14>
   2ecf1:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2ecf6:	48 89 df             	mov    %rbx,%rdi
   2ecf9:	41 ff d5             	call   *%r13
   2ecfc:	49 8d bf 00 80 00 00 	lea    0x8000(%r15),%rdi
   2ed03:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2ed08:	48 89 de             	mov    %rbx,%rsi
   2ed0b:	41 ff d5             	call   *%r13
   2ed0e:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   2ed13:	be 01 00 00 00       	mov    $0x1,%esi
   2ed18:	e8 43 40 00 00       	call   32d60 <verified_tiff::deflate_header::fixed>
   2ed1d:	80 7c 24 28 01       	cmpb   $0x1,0x28(%rsp)
   2ed22:	74 31                	je     2ed55 <<verified_tiff::deflate::Cursor>::step+0xa35>
   2ed24:	48 8d 74 24 30       	lea    0x30(%rsp),%rsi
   2ed29:	48 8d 9c 24 76 03 00 	lea    0x376(%rsp),%rbx
   2ed30:	00 
   2ed31:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2ed36:	48 89 df             	mov    %rbx,%rdi
   2ed39:	e9 cd 00 00 00       	jmp    2ee0b <<verified_tiff::deflate::Cursor>::step+0xaeb>
   2ed3e:	49 89 ff             	mov    %rdi,%r15
   2ed41:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   2ed46:	48 89 d9             	mov    %rbx,%rcx
   2ed49:	e8 d2 42 00 00       	call   33020 <verified_tiff::deflate_header::dynamic>
   2ed4e:	80 7c 24 28 01       	cmpb   $0x1,0x28(%rsp)
   2ed53:	75 57                	jne    2edac <<verified_tiff::deflate::Cursor>::step+0xa8c>
   2ed55:	44 0f b6 6c 24 29    	movzbl 0x29(%rsp),%r13d
   2ed5b:	e9 7f 03 00 00       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2ed60:	48 ff c3             	inc    %rbx
   2ed63:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2ed6a:	4c 39 d8             	cmp    %r11,%rax
   2ed6d:	0f 84 69 03 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2ed73:	48 39 d0             	cmp    %rdx,%rax
   2ed76:	0f 83 c4 04 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2ed7c:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2ed81:	48 83 fb 07          	cmp    $0x7,%rbx
   2ed85:	b9 07 00 00 00       	mov    $0x7,%ecx
   2ed8a:	48 0f 42 cb          	cmovb  %rbx,%rcx
   2ed8e:	41 d2 ed             	shr    %cl,%r13b
   2ed91:	48 83 fb 07          	cmp    $0x7,%rbx
   2ed95:	0f 85 a6 00 00 00    	jne    2ee41 <<verified_tiff::deflate::Cursor>::step+0xb21>
   2ed9b:	48 ff c0             	inc    %rax
   2ed9e:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2eda5:	31 db                	xor    %ebx,%ebx
   2eda7:	e9 98 00 00 00       	jmp    2ee44 <<verified_tiff::deflate::Cursor>::step+0xb24>
   2edac:	48 8d 74 24 30       	lea    0x30(%rsp),%rsi
   2edb1:	48 8d 9c 24 76 03 00 	lea    0x376(%rsp),%rbx
   2edb8:	00 
   2edb9:	4c 8b 2d 60 a9 05 00 	mov    0x5a960(%rip),%r13        # 89720 <memcpy@GLIBC_2.14>
   2edc0:	ba 40 03 00 00       	mov    $0x340,%edx
   2edc5:	48 89 df             	mov    %rbx,%rdi
   2edc8:	41 ff d5             	call   *%r13
   2edcb:	48 8d ac 24 b8 06 00 	lea    0x6b8(%rsp),%rbp
   2edd2:	00 
   2edd3:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2edd8:	48 89 ef             	mov    %rbp,%rdi
   2eddb:	48 89 de             	mov    %rbx,%rsi
   2edde:	41 ff d5             	call   *%r13
   2ede1:	48 8d b4 24 16 05 00 	lea    0x516(%rsp),%rsi
   2ede8:	00 
   2ede9:	48 8d 9c 24 58 08 00 	lea    0x858(%rsp),%rbx
   2edf0:	00 
   2edf1:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2edf6:	48 89 df             	mov    %rbx,%rdi
   2edf9:	41 ff d5             	call   *%r13
   2edfc:	49 8d bf 00 80 00 00 	lea    0x8000(%r15),%rdi
   2ee03:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2ee08:	48 89 ee             	mov    %rbp,%rsi
   2ee0b:	41 ff d5             	call   *%r13
   2ee0e:	49 8d bf a0 81 00 00 	lea    0x81a0(%r15),%rdi
   2ee15:	ba a0 01 00 00       	mov    $0x1a0,%edx
   2ee1a:	48 89 de             	mov    %rbx,%rsi
   2ee1d:	41 ff d5             	call   *%r13
   2ee20:	b1 02                	mov    $0x2,%cl
   2ee22:	41 88 8f 88 83 00 00 	mov    %cl,0x8388(%r15)
   2ee29:	41 0f b6 cc          	movzbl %r12b,%ecx
   2ee2d:	41 0f b6 d6          	movzbl %r14b,%edx
   2ee31:	0f a3 ca             	bt     %ecx,%edx
   2ee34:	41 0f 92 87 89 83 00 	setb   0x8389(%r15)
   2ee3b:	00 
   2ee3c:	e9 eb fa ff ff       	jmp    2e92c <<verified_tiff::deflate::Cursor>::step+0x60c>
   2ee41:	48 ff c3             	inc    %rbx
   2ee44:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2ee4b:	4c 39 d8             	cmp    %r11,%rax
   2ee4e:	0f 84 88 02 00 00    	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2ee54:	44 88 6c 24 08       	mov    %r13b,0x8(%rsp)
   2ee59:	48 39 d0             	cmp    %rdx,%rax
   2ee5c:	0f 83 de 03 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2ee62:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2ee67:	48 83 fb 07          	cmp    $0x7,%rbx
   2ee6b:	b9 07 00 00 00       	mov    $0x7,%ecx
   2ee70:	48 0f 42 cb          	cmovb  %rbx,%rcx
   2ee74:	41 d2 ed             	shr    %cl,%r13b
   2ee77:	44 88 6c 24 10       	mov    %r13b,0x10(%rsp)
   2ee7c:	48 83 fb 07          	cmp    $0x7,%rbx
   2ee80:	0f 85 01 02 00 00    	jne    2f087 <<verified_tiff::deflate::Cursor>::step+0xd67>
   2ee86:	48 ff c0             	inc    %rax
   2ee89:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2ee90:	31 db                	xor    %ebx,%ebx
   2ee92:	e9 f3 01 00 00       	jmp    2f08a <<verified_tiff::deflate::Cursor>::step+0xd6a>
   2ee97:	48 8b 84 24 78 03 00 	mov    0x378(%rsp),%rax
   2ee9e:	00 
   2ee9f:	48 83 f8 1d          	cmp    $0x1d,%rax
   2eea3:	0f 87 36 02 00 00    	ja     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2eea9:	48 c7 44 24 28 01 00 	movq   $0x1,0x28(%rsp)
   2eeb0:	00 00 
   2eeb2:	48 c7 44 24 30 02 00 	movq   $0x2,0x30(%rsp)
   2eeb9:	00 00 
   2eebb:	48 c7 44 24 38 03 00 	movq   $0x3,0x38(%rsp)
   2eec2:	00 00 
   2eec4:	48 c7 44 24 40 04 00 	movq   $0x4,0x40(%rsp)
   2eecb:	00 00 
   2eecd:	48 c7 44 24 48 05 00 	movq   $0x5,0x48(%rsp)
   2eed4:	00 00 
   2eed6:	48 c7 44 24 50 07 00 	movq   $0x7,0x50(%rsp)
   2eedd:	00 00 
   2eedf:	48 c7 44 24 58 09 00 	movq   $0x9,0x58(%rsp)
   2eee6:	00 00 
   2eee8:	48 c7 44 24 60 0d 00 	movq   $0xd,0x60(%rsp)
   2eeef:	00 00 
   2eef1:	48 c7 44 24 68 11 00 	movq   $0x11,0x68(%rsp)
   2eef8:	00 00 
   2eefa:	48 c7 44 24 70 19 00 	movq   $0x19,0x70(%rsp)
   2ef01:	00 00 
   2ef03:	48 c7 44 24 78 21 00 	movq   $0x21,0x78(%rsp)
   2ef0a:	00 00 
   2ef0c:	48 c7 84 24 80 00 00 	movq   $0x31,0x80(%rsp)
   2ef13:	00 31 00 00 00 
   2ef18:	48 c7 84 24 88 00 00 	movq   $0x41,0x88(%rsp)
   2ef1f:	00 41 00 00 00 
   2ef24:	48 c7 84 24 90 00 00 	movq   $0x61,0x90(%rsp)
   2ef2b:	00 61 00 00 00 
   2ef30:	48 c7 84 24 98 00 00 	movq   $0x81,0x98(%rsp)
   2ef37:	00 81 00 00 00 
   2ef3c:	48 c7 84 24 a0 00 00 	movq   $0xc1,0xa0(%rsp)
   2ef43:	00 c1 00 00 00 
   2ef48:	48 c7 84 24 a8 00 00 	movq   $0x101,0xa8(%rsp)
   2ef4f:	00 01 01 00 00 
   2ef54:	48 c7 84 24 b0 00 00 	movq   $0x181,0xb0(%rsp)
   2ef5b:	00 81 01 00 00 
   2ef60:	48 c7 84 24 b8 00 00 	movq   $0x201,0xb8(%rsp)
   2ef67:	00 01 02 00 00 
   2ef6c:	48 c7 84 24 c0 00 00 	movq   $0x301,0xc0(%rsp)
   2ef73:	00 01 03 00 00 
   2ef78:	48 c7 84 24 c8 00 00 	movq   $0x401,0xc8(%rsp)
   2ef7f:	00 01 04 00 00 
   2ef84:	48 c7 84 24 d0 00 00 	movq   $0x601,0xd0(%rsp)
   2ef8b:	00 01 06 00 00 
   2ef90:	48 c7 84 24 d8 00 00 	movq   $0x801,0xd8(%rsp)
   2ef97:	00 01 08 00 00 
   2ef9c:	48 c7 84 24 e0 00 00 	movq   $0xc01,0xe0(%rsp)
   2efa3:	00 01 0c 00 00 
   2efa8:	48 c7 84 24 e8 00 00 	movq   $0x1001,0xe8(%rsp)
   2efaf:	00 01 10 00 00 
   2efb4:	48 c7 84 24 f0 00 00 	movq   $0x1801,0xf0(%rsp)
   2efbb:	00 01 18 00 00 
   2efc0:	48 c7 84 24 f8 00 00 	movq   $0x2001,0xf8(%rsp)
   2efc7:	00 01 20 00 00 
   2efcc:	48 c7 84 24 00 01 00 	movq   $0x3001,0x100(%rsp)
   2efd3:	00 01 30 00 00 
   2efd8:	48 c7 84 24 08 01 00 	movq   $0x4001,0x108(%rsp)
   2efdf:	00 01 40 00 00 
   2efe4:	48 c7 84 24 10 01 00 	movq   $0x6001,0x110(%rsp)
   2efeb:	00 01 60 00 00 
   2eff0:	48 8b 4c c4 28       	mov    0x28(%rsp,%rax,8),%rcx
   2eff5:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   2effa:	48 89 c1             	mov    %rax,%rcx
   2effd:	48 d1 e9             	shr    $1,%rcx
   2f000:	48 ff c9             	dec    %rcx
   2f003:	45 31 c0             	xor    %r8d,%r8d
   2f006:	48 83 f8 04          	cmp    $0x4,%rax
   2f00a:	4c 0f 43 c1          	cmovae %rcx,%r8
   2f00e:	48 8d 7c 24 28       	lea    0x28(%rsp),%rdi
   2f013:	4c 89 fe             	mov    %r15,%rsi
   2f016:	4c 89 f2             	mov    %r14,%rdx
   2f019:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   2f01e:	e8 4d f2 ff ff       	call   2e270 <<verified_tiff::deflate_bits::Bits>::read>
   2f023:	80 7c 24 28 01       	cmpb   $0x1,0x28(%rsp)
   2f028:	0f 84 27 fd ff ff    	je     2ed55 <<verified_tiff::deflate::Cursor>::step+0xa35>
   2f02e:	49 8b 84 24 68 83 00 	mov    0x8368(%r12),%rax
   2f035:	00 
   2f036:	48 8b 4c 24 10       	mov    0x10(%rsp),%rcx
   2f03b:	48 29 c1             	sub    %rax,%rcx
   2f03e:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   2f043:	48 39 ca             	cmp    %rcx,%rdx
   2f046:	0f 87 93 00 00 00    	ja     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2f04c:	48 8b 4c 24 20       	mov    0x20(%rsp),%rcx
   2f051:	48 03 4c 24 30       	add    0x30(%rsp),%rcx
   2f056:	48 39 c1             	cmp    %rax,%rcx
   2f059:	0f 87 80 00 00 00    	ja     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2f05f:	49 3b 8c 24 70 83 00 	cmp    0x8370(%r12),%rcx
   2f066:	00 
   2f067:	77 76                	ja     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2f069:	49 89 8c 24 60 83 00 	mov    %rcx,0x8360(%r12)
   2f070:	00 
   2f071:	49 89 94 24 58 83 00 	mov    %rdx,0x8358(%r12)
   2f078:	00 
   2f079:	41 c6 84 24 88 83 00 	movb   $0x3,0x8388(%r12)
   2f080:	00 03 
   2f082:	e9 a5 f8 ff ff       	jmp    2e92c <<verified_tiff::deflate::Cursor>::step+0x60c>
   2f087:	48 ff c3             	inc    %rbx
   2f08a:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2f091:	4c 39 d8             	cmp    %r11,%rax
   2f094:	74 46                	je     2f0dc <<verified_tiff::deflate::Cursor>::step+0xdbc>
   2f096:	48 39 d0             	cmp    %rdx,%rax
   2f099:	0f 83 a1 01 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2f09f:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2f0a4:	48 83 fb 07          	cmp    $0x7,%rbx
   2f0a8:	b9 07 00 00 00       	mov    $0x7,%ecx
   2f0ad:	48 0f 42 cb          	cmovb  %rbx,%rcx
   2f0b1:	41 d2 ed             	shr    %cl,%r13b
   2f0b4:	44 88 6c 24 18       	mov    %r13b,0x18(%rsp)
   2f0b9:	48 83 fb 07          	cmp    $0x7,%rbx
   2f0bd:	75 0e                	jne    2f0cd <<verified_tiff::deflate::Cursor>::step+0xdad>
   2f0bf:	48 ff c0             	inc    %rax
   2f0c2:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2f0c9:	31 db                	xor    %ebx,%ebx
   2f0cb:	eb 03                	jmp    2f0d0 <<verified_tiff::deflate::Cursor>::step+0xdb0>
   2f0cd:	48 ff c3             	inc    %rbx
   2f0d0:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2f0d7:	4c 39 d8             	cmp    %r11,%rax
   2f0da:	75 21                	jne    2f0fd <<verified_tiff::deflate::Cursor>::step+0xddd>
   2f0dc:	45 31 ed             	xor    %r13d,%r13d
   2f0df:	41 0f b6 cd          	movzbl %r13b,%ecx
   2f0e3:	c1 e1 08             	shl    $0x8,%ecx
   2f0e6:	0f b7 c5             	movzwl %bp,%eax
   2f0e9:	09 c8                	or     %ecx,%eax
   2f0eb:	48 81 c4 f8 09 00 00 	add    $0x9f8,%rsp
   2f0f2:	5b                   	pop    %rbx
   2f0f3:	41 5c                	pop    %r12
   2f0f5:	41 5d                	pop    %r13
   2f0f7:	41 5e                	pop    %r14
   2f0f9:	41 5f                	pop    %r15
   2f0fb:	5d                   	pop    %rbp
   2f0fc:	c3                   	ret
   2f0fd:	48 39 d0             	cmp    %rdx,%rax
   2f100:	0f 83 3a 01 00 00    	jae    2f240 <<verified_tiff::deflate::Cursor>::step+0xf20>
   2f106:	0f b6 34 06          	movzbl (%rsi,%rax,1),%esi
   2f10a:	48 83 fb 07          	cmp    $0x7,%rbx
   2f10e:	b9 07 00 00 00       	mov    $0x7,%ecx
   2f113:	48 0f 42 cb          	cmovb  %rbx,%rcx
   2f117:	40 d2 ee             	shr    %cl,%sil
   2f11a:	48 83 fb 07          	cmp    $0x7,%rbx
   2f11e:	75 13                	jne    2f133 <<verified_tiff::deflate::Cursor>::step+0xe13>
   2f120:	0f b6 4c 24 08       	movzbl 0x8(%rsp),%ecx
   2f125:	48 ff c0             	inc    %rax
   2f128:	48 89 87 40 83 00 00 	mov    %rax,0x8340(%rdi)
   2f12f:	31 db                	xor    %ebx,%ebx
   2f131:	eb 08                	jmp    2f13b <<verified_tiff::deflate::Cursor>::step+0xe1b>
   2f133:	0f b6 4c 24 08       	movzbl 0x8(%rsp),%ecx
   2f138:	48 ff c3             	inc    %rbx
   2f13b:	48 89 9f 48 83 00 00 	mov    %rbx,0x8348(%rdi)
   2f142:	49 ff c8             	dec    %r8
   2f145:	4c 89 87 58 83 00 00 	mov    %r8,0x8358(%rdi)
   2f14c:	4c 8b 87 68 83 00 00 	mov    0x8368(%rdi),%r8
   2f153:	41 b5 01             	mov    $0x1,%r13b
   2f156:	4d 39 f0             	cmp    %r14,%r8
   2f159:	74 84                	je     2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2f15b:	45 00 d2             	add    %r10b,%r10b
   2f15e:	41 80 e2 02          	and    $0x2,%r10b
   2f162:	45 08 d1             	or     %r10b,%r9b
   2f165:	41 c0 e7 02          	shl    $0x2,%r15b
   2f169:	41 80 e7 04          	and    $0x4,%r15b
   2f16d:	45 08 cf             	or     %r9b,%r15b
   2f170:	41 c0 e4 03          	shl    $0x3,%r12b
   2f174:	41 80 e4 08          	and    $0x8,%r12b
   2f178:	45 08 fc             	or     %r15b,%r12b
   2f17b:	c0 e1 04             	shl    $0x4,%cl
   2f17e:	80 e1 10             	and    $0x10,%cl
   2f181:	44 08 e1             	or     %r12b,%cl
   2f184:	0f b6 44 24 10       	movzbl 0x10(%rsp),%eax
   2f189:	c0 e0 05             	shl    $0x5,%al
   2f18c:	24 20                	and    $0x20,%al
   2f18e:	08 c8                	or     %cl,%al
   2f190:	0f b6 4c 24 18       	movzbl 0x18(%rsp),%ecx
   2f195:	c0 e1 06             	shl    $0x6,%cl
   2f198:	80 e1 40             	and    $0x40,%cl
   2f19b:	08 c1                	or     %al,%cl
   2f19d:	40 c0 e6 07          	shl    $0x7,%sil
   2f1a1:	40 08 ce             	or     %cl,%sil
   2f1a4:	40 0f b6 c6          	movzbl %sil,%eax
   2f1a8:	44 89 c1             	mov    %r8d,%ecx
   2f1ab:	81 e1 ff 7f 00 00    	and    $0x7fff,%ecx
   2f1b1:	88 04 0f             	mov    %al,(%rdi,%rcx,1)
   2f1b4:	48 8b 8f 78 83 00 00 	mov    0x8378(%rdi),%rcx
   2f1bb:	48 01 c1             	add    %rax,%rcx
   2f1be:	49 b9 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r9
   2f1c5:	00 0f 00 
   2f1c8:	48 89 c8             	mov    %rcx,%rax
   2f1cb:	49 f7 e1             	mul    %r9
   2f1ce:	48 89 c8             	mov    %rcx,%rax
   2f1d1:	48 29 d0             	sub    %rdx,%rax
   2f1d4:	48 d1 e8             	shr    $1,%rax
   2f1d7:	48 01 d0             	add    %rdx,%rax
   2f1da:	48 c1 e8 0f          	shr    $0xf,%rax
   2f1de:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2f1e5:	48 29 c1             	sub    %rax,%rcx
   2f1e8:	48 89 8f 78 83 00 00 	mov    %rcx,0x8378(%rdi)
   2f1ef:	48 03 8f 80 83 00 00 	add    0x8380(%rdi),%rcx
   2f1f6:	48 89 c8             	mov    %rcx,%rax
   2f1f9:	49 f7 e1             	mul    %r9
   2f1fc:	48 89 c8             	mov    %rcx,%rax
   2f1ff:	48 29 d0             	sub    %rdx,%rax
   2f202:	48 d1 e8             	shr    $1,%rax
   2f205:	48 01 d0             	add    %rdx,%rax
   2f208:	48 c1 e8 0f          	shr    $0xf,%rax
   2f20c:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2f213:	48 29 c1             	sub    %rax,%rcx
   2f216:	48 89 8f 80 83 00 00 	mov    %rcx,0x8380(%rdi)
   2f21d:	49 ff c0             	inc    %r8
   2f220:	4c 89 87 68 83 00 00 	mov    %r8,0x8368(%rdi)
   2f227:	66 bd 01 00          	mov    $0x1,%bp
   2f22b:	41 89 f5             	mov    %esi,%r13d
   2f22e:	e9 ac fe ff ff       	jmp    2f0df <<verified_tiff::deflate::Cursor>::step+0xdbf>
   2f233:	48 8d 15 ce 75 05 00 	lea    0x575ce(%rip),%rdx        # 86808 <anon.51bf5d2968f6ff1b5e1c2e9cd5e69e4a.4.llvm.4425139754156285127>
   2f23a:	ff 15 18 a6 05 00    	call   *0x5a618(%rip)        # 89858 <_DYNAMIC+0x388>
   2f240:	48 89 d6             	mov    %rdx,%rsi
   2f243:	48 8d 15 2e 72 05 00 	lea    0x5722e(%rip),%rdx        # 86478 <anon.ac36df6000eed6ce22b87f572999359a.32.llvm.17988912158883924058+0xf0>
   2f24a:	48 89 c7             	mov    %rax,%rdi
   2f24d:	ff 15 05 a6 05 00    	call   *0x5a605(%rip)        # 89858 <_DYNAMIC+0x388>
   2f253:	48 8d 05 1e 72 05 00 	lea    0x5721e(%rip),%rax        # 86478 <anon.ac36df6000eed6ce22b87f572999359a.32.llvm.17988912158883924058+0xf0>
   2f25a:	48 89 cf             	mov    %rcx,%rdi
   2f25d:	48 89 d6             	mov    %rdx,%rsi
   2f260:	48 89 c2             	mov    %rax,%rdx
   2f263:	ff 15 ef a5 05 00    	call   *0x5a5ef(%rip)        # 89858 <_DYNAMIC+0x388>
   2f269:	48 8d 0d 58 6d 05 00 	lea    0x56d58(%rip),%rcx        # 85fc8 <anon.35f1884359c0d9048fdad1d4cf8af759.18.llvm.17727380151100635305>
   2f270:	48 89 c7             	mov    %rax,%rdi
   2f273:	48 89 d6             	mov    %rdx,%rsi
   2f276:	48 89 ca             	mov    %rcx,%rdx
   2f279:	ff 15 d9 a5 05 00    	call   *0x5a5d9(%rip)        # 89858 <_DYNAMIC+0x388>
   2f27f:	48 8d 05 5a 6d 05 00 	lea    0x56d5a(%rip),%rax        # 85fe0 <anon.35f1884359c0d9048fdad1d4cf8af759.19.llvm.17727380151100635305>
   2f286:	48 89 cf             	mov    %rcx,%rdi
   2f289:	48 89 d6             	mov    %rdx,%rsi
   2f28c:	48 89 c2             	mov    %rax,%rdx
   2f28f:	ff 15 c3 a5 05 00    	call   *0x5a5c3(%rip)        # 89858 <_DYNAMIC+0x388>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
