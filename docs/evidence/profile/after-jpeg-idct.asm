
target/release/examples/benchmark:     file format elf64-x86-64


Disassembly of section .text:

00000000000361a0 <verified_tiff::jpeg_scan::decode_blocks>:
   361a0:	55                   	push   %rbp
   361a1:	41 57                	push   %r15
   361a3:	41 56                	push   %r14
   361a5:	41 55                	push   %r13
   361a7:	41 54                	push   %r12
   361a9:	53                   	push   %rbx
   361aa:	48 81 ec 98 06 00 00 	sub    $0x698,%rsp
   361b1:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
   361b6:	48 89 74 24 18       	mov    %rsi,0x18(%rsp)
   361bb:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
   361c0:	0f 57 c0             	xorps  %xmm0,%xmm0
   361c3:	0f 29 44 24 60       	movaps %xmm0,0x60(%rsp)
   361c8:	48 c7 44 24 70 00 00 	movq   $0x0,0x70(%rsp)
   361cf:	00 00 
   361d1:	0f 10 01             	movups (%rcx),%xmm0
   361d4:	0f 10 49 10          	movups 0x10(%rcx),%xmm1
   361d8:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   361dd:	0f 29 4c 24 50       	movaps %xmm1,0x50(%rsp)
   361e2:	48 c7 44 24 78 00 00 	movq   $0x0,0x78(%rsp)
   361e9:	00 00 
   361eb:	b0 ff                	mov    $0xff,%al
   361ed:	4c 89 44 24 08       	mov    %r8,0x8(%rsp)
   361f2:	4d 85 c0             	test   %r8,%r8
   361f5:	0f 84 7c 03 00 00    	je     36577 <verified_tiff::jpeg_scan::decode_blocks+0x3d7>
   361fb:	48 8b b4 24 d8 06 00 	mov    0x6d8(%rsp),%rsi
   36202:	00 
   36203:	48 8b 9c 24 d0 06 00 	mov    0x6d0(%rsp),%rbx
   3620a:	00 
   3620b:	45 31 ff             	xor    %r15d,%r15d
   3620e:	4c 8d ac 24 88 00 00 	lea    0x88(%rsp),%r13
   36215:	00 
   36216:	4c 8d 35 7b 46 fd ff 	lea    -0x2b985(%rip),%r14        # a898 <anon.53fc985f19b08ffe8bb527278e35fb9b.6.llvm.3502487714873411978+0x98>
   3621d:	31 c0                	xor    %eax,%eax
   3621f:	31 c9                	xor    %ecx,%ecx
   36221:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   36228:	0f 1f 84 00 00 00 00 
   3622f:	00 
   36230:	4c 39 fe             	cmp    %r15,%rsi
   36233:	4c 89 fd             	mov    %r15,%rbp
   36236:	48 0f 47 ee          	cmova  %rsi,%rbp
   3623a:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
   3623f:	48 01 c5             	add    %rax,%rbp
   36242:	48 83 fd 3f          	cmp    $0x3f,%rbp
   36246:	b8 3f 00 00 00       	mov    $0x3f,%eax
   3624b:	48 0f 43 e8          	cmovae %rax,%rbp
   3624f:	48 89 4c 24 28       	mov    %rcx,0x28(%rsp)
   36254:	48 89 c8             	mov    %rcx,%rax
   36257:	48 c1 e0 06          	shl    $0x6,%rax
   3625b:	48 89 f1             	mov    %rsi,%rcx
   3625e:	48 29 c1             	sub    %rax,%rcx
   36261:	b8 00 00 00 00       	mov    $0x0,%eax
   36266:	48 0f 43 c1          	cmovae %rcx,%rax
   3626a:	48 83 f8 3f          	cmp    $0x3f,%rax
   3626e:	41 bc 3f 00 00 00    	mov    $0x3f,%r12d
   36274:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   36279:	4c 0f 42 e0          	cmovb  %rax,%r12
   3627d:	4c 89 ef             	mov    %r13,%rdi
   36280:	48 8d 74 24 40       	lea    0x40(%rsp),%rsi
   36285:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
   3628a:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
   3628f:	4c 8b 44 24 20       	mov    0x20(%rsp),%r8
   36294:	e8 97 dc ff ff       	call   33f30 <<verified_tiff::jpeg_scan::Cursor>::next>
   36299:	80 bc 24 88 00 00 00 	cmpb   $0x0,0x88(%rsp)
   362a0:	00 
   362a1:	0f 85 c8 02 00 00    	jne    3656f <verified_tiff::jpeg_scan::decode_blocks+0x3cf>
   362a7:	49 ff c4             	inc    %r12
   362aa:	ba 00 02 00 00       	mov    $0x200,%edx
   362af:	48 8d bc 24 96 04 00 	lea    0x496(%rsp),%rdi
   362b6:	00 
   362b7:	48 8d b4 24 90 00 00 	lea    0x90(%rsp),%rsi
   362be:	00 
   362bf:	ff 15 3b 43 05 00    	call   *0x5433b(%rip)        # 8a600 <memcpy@GLIBC_2.14>
   362c5:	ba 00 02 00 00       	mov    $0x200,%edx
   362ca:	48 8d bc 24 90 02 00 	lea    0x290(%rsp),%rdi
   362d1:	00 
   362d2:	48 8d b4 24 96 04 00 	lea    0x496(%rsp),%rsi
   362d9:	00 
   362da:	ff 15 20 43 05 00    	call   *0x54320(%rip)        # 8a600 <memcpy@GLIBC_2.14>
   362e0:	ba 00 02 00 00       	mov    $0x200,%edx
   362e5:	4c 89 ef             	mov    %r13,%rdi
   362e8:	31 f6                	xor    %esi,%esi
   362ea:	ff 15 48 44 05 00    	call   *0x54448(%rip)        # 8a738 <memset@GLIBC_2.2.5>
   362f0:	31 c0                	xor    %eax,%eax
   362f2:	31 c9                	xor    %ecx,%ecx
   362f4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   362fb:	00 00 00 00 00 
   36300:	89 c2                	mov    %eax,%edx
   36302:	83 e2 38             	and    $0x38,%edx
   36305:	89 ce                	mov    %ecx,%esi
   36307:	83 e6 38             	and    $0x38,%esi
   3630a:	49 8b 3c d6          	mov    (%r14,%rdx,8),%rdi
   3630e:	48 0f af bc f4 90 02 	imul   0x290(%rsp,%rsi,8),%rdi
   36315:	00 00 
   36317:	4d 8b 44 d6 08       	mov    0x8(%r14,%rdx,8),%r8
   3631c:	4c 0f af 84 f4 98 02 	imul   0x298(%rsp,%rsi,8),%r8
   36323:	00 00 
   36325:	49 01 f8             	add    %rdi,%r8
   36328:	49 8b 7c d6 10       	mov    0x10(%r14,%rdx,8),%rdi
   3632d:	48 0f af bc f4 a0 02 	imul   0x2a0(%rsp,%rsi,8),%rdi
   36334:	00 00 
   36336:	4d 8b 4c d6 18       	mov    0x18(%r14,%rdx,8),%r9
   3633b:	4c 0f af 8c f4 a8 02 	imul   0x2a8(%rsp,%rsi,8),%r9
   36342:	00 00 
   36344:	49 01 f9             	add    %rdi,%r9
   36347:	4d 01 c1             	add    %r8,%r9
   3634a:	49 8b 7c d6 20       	mov    0x20(%r14,%rdx,8),%rdi
   3634f:	48 0f af bc f4 b0 02 	imul   0x2b0(%rsp,%rsi,8),%rdi
   36356:	00 00 
   36358:	4d 8b 44 d6 28       	mov    0x28(%r14,%rdx,8),%r8
   3635d:	4c 0f af 84 f4 b8 02 	imul   0x2b8(%rsp,%rsi,8),%r8
   36364:	00 00 
   36366:	49 01 f8             	add    %rdi,%r8
   36369:	49 8b 7c d6 30       	mov    0x30(%r14,%rdx,8),%rdi
   3636e:	48 0f af bc f4 c0 02 	imul   0x2c0(%rsp,%rsi,8),%rdi
   36375:	00 00 
   36377:	4c 01 c7             	add    %r8,%rdi
   3637a:	4c 01 cf             	add    %r9,%rdi
   3637d:	49 8b 54 d6 38       	mov    0x38(%r14,%rdx,8),%rdx
   36382:	48 0f af 94 f4 c8 02 	imul   0x2c8(%rsp,%rsi,8),%rdx
   36389:	00 00 
   3638b:	48 01 fa             	add    %rdi,%rdx
   3638e:	48 89 94 cc 88 00 00 	mov    %rdx,0x88(%rsp,%rcx,8)
   36395:	00 
   36396:	48 ff c1             	inc    %rcx
   36399:	48 83 c0 08          	add    $0x8,%rax
   3639d:	48 83 f9 40          	cmp    $0x40,%rcx
   363a1:	0f 85 59 ff ff ff    	jne    36300 <verified_tiff::jpeg_scan::decode_blocks+0x160>
   363a7:	0f 57 c0             	xorps  %xmm0,%xmm0
   363aa:	0f 29 84 24 c0 04 00 	movaps %xmm0,0x4c0(%rsp)
   363b1:	00 
   363b2:	0f 29 84 24 b0 04 00 	movaps %xmm0,0x4b0(%rsp)
   363b9:	00 
   363ba:	0f 29 84 24 a0 04 00 	movaps %xmm0,0x4a0(%rsp)
   363c1:	00 
   363c2:	0f 29 84 24 90 04 00 	movaps %xmm0,0x490(%rsp)
   363c9:	00 
   363ca:	31 c0                	xor    %eax,%eax
   363cc:	49 b9 00 00 00 20 20 	movabs $0x2020000000,%r9
   363d3:	00 00 00 
   363d6:	eb 1e                	jmp    363f6 <verified_tiff::jpeg_scan::decode_blocks+0x256>
   363d8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   363df:	00 
   363e0:	31 c9                	xor    %ecx,%ecx
   363e2:	88 8c 04 90 04 00 00 	mov    %cl,0x490(%rsp,%rax,1)
   363e9:	48 ff c0             	inc    %rax
   363ec:	48 83 f8 40          	cmp    $0x40,%rax
   363f0:	0f 84 ba 00 00 00    	je     364b0 <verified_tiff::jpeg_scan::decode_blocks+0x310>
   363f6:	89 c2                	mov    %eax,%edx
   363f8:	83 e2 38             	and    $0x38,%edx
   363fb:	89 c1                	mov    %eax,%ecx
   363fd:	83 e1 07             	and    $0x7,%ecx
   36400:	49 8b 34 d6          	mov    (%r14,%rdx,8),%rsi
   36404:	49 8b 7c d6 08       	mov    0x8(%r14,%rdx,8),%rdi
   36409:	48 0f af b4 cc 88 00 	imul   0x88(%rsp,%rcx,8),%rsi
   36410:	00 00 
   36412:	48 0f af bc cc c8 00 	imul   0xc8(%rsp,%rcx,8),%rdi
   36419:	00 00 
   3641b:	4d 8b 44 d6 10       	mov    0x10(%r14,%rdx,8),%r8
   36420:	4c 0f af 84 cc 08 01 	imul   0x108(%rsp,%rcx,8),%r8
   36427:	00 00 
   36429:	48 01 f7             	add    %rsi,%rdi
   3642c:	49 8b 74 d6 18       	mov    0x18(%r14,%rdx,8),%rsi
   36431:	48 0f af b4 cc 48 01 	imul   0x148(%rsp,%rcx,8),%rsi
   36438:	00 00 
   3643a:	4c 01 c6             	add    %r8,%rsi
   3643d:	4d 8b 44 d6 20       	mov    0x20(%r14,%rdx,8),%r8
   36442:	4c 0f af 84 cc 88 01 	imul   0x188(%rsp,%rcx,8),%r8
   36449:	00 00 
   3644b:	48 01 fe             	add    %rdi,%rsi
   3644e:	49 8b 7c d6 28       	mov    0x28(%r14,%rdx,8),%rdi
   36453:	48 0f af bc cc c8 01 	imul   0x1c8(%rsp,%rcx,8),%rdi
   3645a:	00 00 
   3645c:	4c 01 c7             	add    %r8,%rdi
   3645f:	4d 8b 44 d6 30       	mov    0x30(%r14,%rdx,8),%r8
   36464:	4c 0f af 84 cc 08 02 	imul   0x208(%rsp,%rcx,8),%r8
   3646b:	00 00 
   3646d:	49 01 f8             	add    %rdi,%r8
   36470:	49 8b 54 d6 38       	mov    0x38(%r14,%rdx,8),%rdx
   36475:	48 0f af 94 cc 48 02 	imul   0x248(%rsp,%rcx,8),%rdx
   3647c:	00 00 
   3647e:	49 01 f0             	add    %rsi,%r8
   36481:	4c 01 ca             	add    %r9,%rdx
   36484:	4c 01 c2             	add    %r8,%rdx
   36487:	48 85 d2             	test   %rdx,%rdx
   3648a:	0f 8e 50 ff ff ff    	jle    363e0 <verified_tiff::jpeg_scan::decode_blocks+0x240>
   36490:	48 c1 ea 1e          	shr    $0x1e,%rdx
   36494:	b1 ff                	mov    $0xff,%cl
   36496:	48 81 fa fe 00 00 00 	cmp    $0xfe,%rdx
   3649d:	0f 87 3f ff ff ff    	ja     363e2 <verified_tiff::jpeg_scan::decode_blocks+0x242>
   364a3:	89 d1                	mov    %edx,%ecx
   364a5:	e9 38 ff ff ff       	jmp    363e2 <verified_tiff::jpeg_scan::decode_blocks+0x242>
   364aa:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   364b0:	48 83 7c 24 38 0f    	cmpq   $0xf,0x38(%rsp)
   364b6:	77 18                	ja     364d0 <verified_tiff::jpeg_scan::decode_blocks+0x330>
   364b8:	45 31 e4             	xor    %r12d,%r12d
   364bb:	48 8b b4 24 d8 06 00 	mov    0x6d8(%rsp),%rsi
   364c2:	00 
   364c3:	eb 50                	jmp    36515 <verified_tiff::jpeg_scan::decode_blocks+0x375>
   364c5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   364cc:	00 00 00 00 
   364d0:	44 89 e0             	mov    %r12d,%eax
   364d3:	83 e0 0f             	and    $0xf,%eax
   364d6:	b9 10 00 00 00       	mov    $0x10,%ecx
   364db:	48 0f 44 c1          	cmove  %rcx,%rax
   364df:	49 29 c4             	sub    %rax,%r12
   364e2:	48 29 c5             	sub    %rax,%rbp
   364e5:	48 ff c5             	inc    %rbp
   364e8:	31 c0                	xor    %eax,%eax
   364ea:	48 8b b4 24 d8 06 00 	mov    0x6d8(%rsp),%rsi
   364f1:	00 
   364f2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   364f9:	1f 84 00 00 00 00 00 
   36500:	0f 10 84 04 90 04 00 	movups 0x490(%rsp,%rax,1),%xmm0
   36507:	00 
   36508:	0f 11 04 03          	movups %xmm0,(%rbx,%rax,1)
   3650c:	48 83 c0 10          	add    $0x10,%rax
   36510:	48 39 c5             	cmp    %rax,%rbp
   36513:	75 eb                	jne    36500 <verified_tiff::jpeg_scan::decode_blocks+0x360>
   36515:	48 8b 84 24 d0 06 00 	mov    0x6d0(%rsp),%rax
   3651c:	00 
   3651d:	4c 01 f8             	add    %r15,%rax
   36520:	4b 8d 3c 27          	lea    (%r15,%r12,1),%rdi
   36524:	48 39 f7             	cmp    %rsi,%rdi
   36527:	73 60                	jae    36589 <verified_tiff::jpeg_scan::decode_blocks+0x3e9>
   36529:	42 0f b6 8c 24 90 04 	movzbl 0x490(%rsp,%r12,1),%ecx
   36530:	00 00 
   36532:	42 88 0c 20          	mov    %cl,(%rax,%r12,1)
   36536:	49 ff c4             	inc    %r12
   36539:	49 83 fc 40          	cmp    $0x40,%r12
   3653d:	75 e1                	jne    36520 <verified_tiff::jpeg_scan::decode_blocks+0x380>
   3653f:	48 8b 4c 24 28       	mov    0x28(%rsp),%rcx
   36544:	48 ff c1             	inc    %rcx
   36547:	48 83 c3 40          	add    $0x40,%rbx
   3654b:	49 83 c7 40          	add    $0x40,%r15
   3654f:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   36554:	48 83 c0 c0          	add    $0xffffffffffffffc0,%rax
   36558:	48 3b 4c 24 08       	cmp    0x8(%rsp),%rcx
   3655d:	4c 8d ac 24 88 00 00 	lea    0x88(%rsp),%r13
   36564:	00 
   36565:	0f 85 c5 fc ff ff    	jne    36230 <verified_tiff::jpeg_scan::decode_blocks+0x90>
   3656b:	b0 ff                	mov    $0xff,%al
   3656d:	eb 08                	jmp    36577 <verified_tiff::jpeg_scan::decode_blocks+0x3d7>
   3656f:	0f b6 84 24 89 00 00 	movzbl 0x89(%rsp),%eax
   36576:	00 
   36577:	48 81 c4 98 06 00 00 	add    $0x698,%rsp
   3657e:	5b                   	pop    %rbx
   3657f:	41 5c                	pop    %r12
   36581:	41 5d                	pop    %r13
   36583:	41 5e                	pop    %r14
   36585:	41 5f                	pop    %r15
   36587:	5d                   	pop    %rbp
   36588:	c3                   	ret
   36589:	48 8d 15 08 16 05 00 	lea    0x51608(%rip),%rdx        # 87b98 <anon.53fc985f19b08ffe8bb527278e35fb9b.11.llvm.3502487714873411978+0x4b0>
   36590:	ff 15 6a 41 05 00    	call   *0x5416a(%rip)        # 8a700 <_DYNAMIC+0x350>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
