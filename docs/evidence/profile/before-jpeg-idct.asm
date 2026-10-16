
target/baseline-v18/verified-tiff/target/release/examples/benchmark:     file format elf64-x86-64


Disassembly of section .text:

00000000000313a0 <verified_tiff::jpeg_scan::decode_blocks>:
   313a0:	55                   	push   %rbp
   313a1:	41 57                	push   %r15
   313a3:	41 56                	push   %r14
   313a5:	41 55                	push   %r13
   313a7:	41 54                	push   %r12
   313a9:	53                   	push   %rbx
   313aa:	48 81 ec c8 06 00 00 	sub    $0x6c8,%rsp
   313b1:	48 89 94 24 20 02 00 	mov    %rdx,0x220(%rsp)
   313b8:	00 
   313b9:	48 89 b4 24 18 02 00 	mov    %rsi,0x218(%rsp)
   313c0:	00 
   313c1:	48 89 bc 24 10 02 00 	mov    %rdi,0x210(%rsp)
   313c8:	00 
   313c9:	0f 57 c0             	xorps  %xmm0,%xmm0
   313cc:	0f 29 84 24 a0 02 00 	movaps %xmm0,0x2a0(%rsp)
   313d3:	00 
   313d4:	48 c7 84 24 b0 02 00 	movq   $0x0,0x2b0(%rsp)
   313db:	00 00 00 00 00 
   313e0:	0f 10 01             	movups (%rcx),%xmm0
   313e3:	0f 10 49 10          	movups 0x10(%rcx),%xmm1
   313e7:	0f 29 84 24 80 02 00 	movaps %xmm0,0x280(%rsp)
   313ee:	00 
   313ef:	0f 29 8c 24 90 02 00 	movaps %xmm1,0x290(%rsp)
   313f6:	00 
   313f7:	48 c7 84 24 b8 02 00 	movq   $0x0,0x2b8(%rsp)
   313fe:	00 00 00 00 00 
   31403:	b0 ff                	mov    $0xff,%al
   31405:	4c 89 84 24 08 02 00 	mov    %r8,0x208(%rsp)
   3140c:	00 
   3140d:	4d 85 c0             	test   %r8,%r8
   31410:	0f 84 6f 08 00 00    	je     31c85 <verified_tiff::jpeg_scan::decode_blocks+0x8e5>
   31416:	48 8b b4 24 08 07 00 	mov    0x708(%rsp),%rsi
   3141d:	00 
   3141e:	4c 8b a4 24 00 07 00 	mov    0x700(%rsp),%r12
   31425:	00 
   31426:	31 db                	xor    %ebx,%ebx
   31428:	45 31 f6             	xor    %r14d,%r14d
   3142b:	45 31 ed             	xor    %r13d,%r13d
   3142e:	66 90                	xchg   %ax,%ax
   31430:	48 39 de             	cmp    %rbx,%rsi
   31433:	48 89 dd             	mov    %rbx,%rbp
   31436:	48 0f 47 ee          	cmova  %rsi,%rbp
   3143a:	4c 01 f5             	add    %r14,%rbp
   3143d:	48 83 fd 3f          	cmp    $0x3f,%rbp
   31441:	b8 3f 00 00 00       	mov    $0x3f,%eax
   31446:	48 0f 43 e8          	cmovae %rax,%rbp
   3144a:	4c 89 e8             	mov    %r13,%rax
   3144d:	48 c1 e0 06          	shl    $0x6,%rax
   31451:	48 29 c6             	sub    %rax,%rsi
   31454:	b8 00 00 00 00       	mov    $0x0,%eax
   31459:	48 0f 43 c6          	cmovae %rsi,%rax
   3145d:	48 83 f8 3f          	cmp    $0x3f,%rax
   31461:	41 bf 3f 00 00 00    	mov    $0x3f,%r15d
   31467:	48 89 84 24 38 02 00 	mov    %rax,0x238(%rsp)
   3146e:	00 
   3146f:	4c 0f 42 f8          	cmovb  %rax,%r15
   31473:	48 89 e7             	mov    %rsp,%rdi
   31476:	48 8d b4 24 80 02 00 	lea    0x280(%rsp),%rsi
   3147d:	00 
   3147e:	48 8b 94 24 10 02 00 	mov    0x210(%rsp),%rdx
   31485:	00 
   31486:	48 8b 8c 24 18 02 00 	mov    0x218(%rsp),%rcx
   3148d:	00 
   3148e:	4c 8b 84 24 20 02 00 	mov    0x220(%rsp),%r8
   31495:	00 
   31496:	e8 05 de ff ff       	call   2f2a0 <<verified_tiff::jpeg_scan::Cursor>::next>
   3149b:	80 3c 24 00          	cmpb   $0x0,(%rsp)
   3149f:	0f 85 db 07 00 00    	jne    31c80 <verified_tiff::jpeg_scan::decode_blocks+0x8e0>
   314a5:	4c 89 ac 24 28 02 00 	mov    %r13,0x228(%rsp)
   314ac:	00 
   314ad:	4c 89 b4 24 30 02 00 	mov    %r14,0x230(%rsp)
   314b4:	00 
   314b5:	49 ff c7             	inc    %r15
   314b8:	ba 00 02 00 00       	mov    $0x200,%edx
   314bd:	4c 8d b4 24 c6 02 00 	lea    0x2c6(%rsp),%r14
   314c4:	00 
   314c5:	4c 89 f7             	mov    %r14,%rdi
   314c8:	48 8d 74 24 08       	lea    0x8(%rsp),%rsi
   314cd:	4c 8b 2d 4c 82 05 00 	mov    0x5824c(%rip),%r13        # 89720 <memcpy@GLIBC_2.14>
   314d4:	41 ff d5             	call   *%r13
   314d7:	ba 00 02 00 00       	mov    $0x200,%edx
   314dc:	48 8d bc 24 c8 04 00 	lea    0x4c8(%rsp),%rdi
   314e3:	00 
   314e4:	4c 89 f6             	mov    %r14,%rsi
   314e7:	41 ff d5             	call   *%r13
   314ea:	ba 00 02 00 00       	mov    $0x200,%edx
   314ef:	48 8d bc 24 c0 02 00 	lea    0x2c0(%rsp),%rdi
   314f6:	00 
   314f7:	31 f6                	xor    %esi,%esi
   314f9:	ff 15 51 83 05 00    	call   *0x58351(%rip)        # 89850 <memset@GLIBC_2.2.5>
   314ff:	31 c0                	xor    %eax,%eax
   31501:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   31508:	0f 1f 84 00 00 00 00 
   3150f:	00 
   31510:	89 c1                	mov    %eax,%ecx
   31512:	83 e1 07             	and    $0x7,%ecx
   31515:	c1 e1 06             	shl    $0x6,%ecx
   31518:	48 01 e1             	add    %rsp,%rcx
   3151b:	48 89 c2             	mov    %rax,%rdx
   3151e:	48 83 e2 f8          	and    $0xfffffffffffffff8,%rdx
   31522:	48 8d 14 d4          	lea    (%rsp,%rdx,8),%rdx
   31526:	48 81 c2 c8 04 00 00 	add    $0x4c8,%rdx
   3152d:	31 ff                	xor    %edi,%edi
   3152f:	31 f6                	xor    %esi,%esi
   31531:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   31538:	0f 1f 84 00 00 00 00 
   3153f:	00 
   31540:	4c 8b 04 fa          	mov    (%rdx,%rdi,8),%r8
   31544:	48 c7 04 24 41 2d 00 	movq   $0x2d41,(%rsp)
   3154b:	00 
   3154c:	48 c7 44 24 08 c5 3e 	movq   $0x3ec5,0x8(%rsp)
   31553:	00 00 
   31555:	48 c7 44 24 10 21 3b 	movq   $0x3b21,0x10(%rsp)
   3155c:	00 00 
   3155e:	48 c7 44 24 18 37 35 	movq   $0x3537,0x18(%rsp)
   31565:	00 00 
   31567:	48 c7 44 24 20 41 2d 	movq   $0x2d41,0x20(%rsp)
   3156e:	00 00 
   31570:	48 c7 44 24 28 8e 23 	movq   $0x238e,0x28(%rsp)
   31577:	00 00 
   31579:	48 c7 44 24 30 7e 18 	movq   $0x187e,0x30(%rsp)
   31580:	00 00 
   31582:	48 c7 44 24 38 7c 0c 	movq   $0xc7c,0x38(%rsp)
   31589:	00 00 
   3158b:	48 c7 44 24 40 41 2d 	movq   $0x2d41,0x40(%rsp)
   31592:	00 00 
   31594:	48 c7 44 24 48 37 35 	movq   $0x3537,0x48(%rsp)
   3159b:	00 00 
   3159d:	48 c7 44 24 50 7e 18 	movq   $0x187e,0x50(%rsp)
   315a4:	00 00 
   315a6:	48 c7 44 24 58 84 f3 	movq   $0xfffffffffffff384,0x58(%rsp)
   315ad:	ff ff 
   315af:	48 c7 44 24 60 bf d2 	movq   $0xffffffffffffd2bf,0x60(%rsp)
   315b6:	ff ff 
   315b8:	48 c7 44 24 68 3b c1 	movq   $0xffffffffffffc13b,0x68(%rsp)
   315bf:	ff ff 
   315c1:	48 c7 44 24 70 df c4 	movq   $0xffffffffffffc4df,0x70(%rsp)
   315c8:	ff ff 
   315ca:	48 c7 44 24 78 72 dc 	movq   $0xffffffffffffdc72,0x78(%rsp)
   315d1:	ff ff 
   315d3:	48 c7 84 24 80 00 00 	movq   $0x2d41,0x80(%rsp)
   315da:	00 41 2d 00 00 
   315df:	48 c7 84 24 88 00 00 	movq   $0x238e,0x88(%rsp)
   315e6:	00 8e 23 00 00 
   315eb:	48 c7 84 24 90 00 00 	movq   $0xffffffffffffe782,0x90(%rsp)
   315f2:	00 82 e7 ff ff 
   315f7:	48 c7 84 24 98 00 00 	movq   $0xffffffffffffc13b,0x98(%rsp)
   315fe:	00 3b c1 ff ff 
   31603:	48 c7 84 24 a0 00 00 	movq   $0xffffffffffffd2bf,0xa0(%rsp)
   3160a:	00 bf d2 ff ff 
   3160f:	48 c7 84 24 a8 00 00 	movq   $0xc7c,0xa8(%rsp)
   31616:	00 7c 0c 00 00 
   3161b:	48 c7 84 24 b0 00 00 	movq   $0x3b21,0xb0(%rsp)
   31622:	00 21 3b 00 00 
   31627:	48 c7 84 24 b8 00 00 	movq   $0x3537,0xb8(%rsp)
   3162e:	00 37 35 00 00 
   31633:	48 c7 84 24 c0 00 00 	movq   $0x2d41,0xc0(%rsp)
   3163a:	00 41 2d 00 00 
   3163f:	48 c7 84 24 c8 00 00 	movq   $0xc7c,0xc8(%rsp)
   31646:	00 7c 0c 00 00 
   3164b:	48 c7 84 24 d0 00 00 	movq   $0xffffffffffffc4df,0xd0(%rsp)
   31652:	00 df c4 ff ff 
   31657:	48 c7 84 24 d8 00 00 	movq   $0xffffffffffffdc72,0xd8(%rsp)
   3165e:	00 72 dc ff ff 
   31663:	48 c7 84 24 e0 00 00 	movq   $0x2d41,0xe0(%rsp)
   3166a:	00 41 2d 00 00 
   3166f:	48 c7 84 24 e8 00 00 	movq   $0x3537,0xe8(%rsp)
   31676:	00 37 35 00 00 
   3167b:	48 c7 84 24 f0 00 00 	movq   $0xffffffffffffe782,0xf0(%rsp)
   31682:	00 82 e7 ff ff 
   31687:	48 c7 84 24 f8 00 00 	movq   $0xffffffffffffc13b,0xf8(%rsp)
   3168e:	00 3b c1 ff ff 
   31693:	48 c7 84 24 00 01 00 	movq   $0x2d41,0x100(%rsp)
   3169a:	00 41 2d 00 00 
   3169f:	48 c7 84 24 08 01 00 	movq   $0xfffffffffffff384,0x108(%rsp)
   316a6:	00 84 f3 ff ff 
   316ab:	48 c7 84 24 10 01 00 	movq   $0xffffffffffffc4df,0x110(%rsp)
   316b2:	00 df c4 ff ff 
   316b7:	48 c7 84 24 18 01 00 	movq   $0x238e,0x118(%rsp)
   316be:	00 8e 23 00 00 
   316c3:	48 c7 84 24 20 01 00 	movq   $0x2d41,0x120(%rsp)
   316ca:	00 41 2d 00 00 
   316cf:	48 c7 84 24 28 01 00 	movq   $0xffffffffffffcac9,0x128(%rsp)
   316d6:	00 c9 ca ff ff 
   316db:	48 c7 84 24 30 01 00 	movq   $0xffffffffffffe782,0x130(%rsp)
   316e2:	00 82 e7 ff ff 
   316e7:	48 c7 84 24 38 01 00 	movq   $0x3ec5,0x138(%rsp)
   316ee:	00 c5 3e 00 00 
   316f3:	48 c7 84 24 40 01 00 	movq   $0x2d41,0x140(%rsp)
   316fa:	00 41 2d 00 00 
   316ff:	48 c7 84 24 48 01 00 	movq   $0xffffffffffffdc72,0x148(%rsp)
   31706:	00 72 dc ff ff 
   3170b:	48 c7 84 24 50 01 00 	movq   $0xffffffffffffe782,0x150(%rsp)
   31712:	00 82 e7 ff ff 
   31717:	48 c7 84 24 58 01 00 	movq   $0x3ec5,0x158(%rsp)
   3171e:	00 c5 3e 00 00 
   31723:	48 c7 84 24 60 01 00 	movq   $0xffffffffffffd2bf,0x160(%rsp)
   3172a:	00 bf d2 ff ff 
   3172f:	48 c7 84 24 68 01 00 	movq   $0xfffffffffffff384,0x168(%rsp)
   31736:	00 84 f3 ff ff 
   3173b:	48 c7 84 24 70 01 00 	movq   $0x3b21,0x170(%rsp)
   31742:	00 21 3b 00 00 
   31747:	48 c7 84 24 78 01 00 	movq   $0xffffffffffffcac9,0x178(%rsp)
   3174e:	00 c9 ca ff ff 
   31753:	48 c7 84 24 80 01 00 	movq   $0x2d41,0x180(%rsp)
   3175a:	00 41 2d 00 00 
   3175f:	48 c7 84 24 88 01 00 	movq   $0xffffffffffffcac9,0x188(%rsp)
   31766:	00 c9 ca ff ff 
   3176b:	48 c7 84 24 90 01 00 	movq   $0x187e,0x190(%rsp)
   31772:	00 7e 18 00 00 
   31777:	48 c7 84 24 98 01 00 	movq   $0xc7c,0x198(%rsp)
   3177e:	00 7c 0c 00 00 
   31783:	48 c7 84 24 a0 01 00 	movq   $0xffffffffffffd2bf,0x1a0(%rsp)
   3178a:	00 bf d2 ff ff 
   3178f:	48 c7 84 24 a8 01 00 	movq   $0x3ec5,0x1a8(%rsp)
   31796:	00 c5 3e 00 00 
   3179b:	48 c7 84 24 b0 01 00 	movq   $0xffffffffffffc4df,0x1b0(%rsp)
   317a2:	00 df c4 ff ff 
   317a7:	48 c7 84 24 b8 01 00 	movq   $0x238e,0x1b8(%rsp)
   317ae:	00 8e 23 00 00 
   317b3:	48 c7 84 24 c0 01 00 	movq   $0x2d41,0x1c0(%rsp)
   317ba:	00 41 2d 00 00 
   317bf:	48 c7 84 24 c8 01 00 	movq   $0xffffffffffffc13b,0x1c8(%rsp)
   317c6:	00 3b c1 ff ff 
   317cb:	48 c7 84 24 d0 01 00 	movq   $0x3b21,0x1d0(%rsp)
   317d2:	00 21 3b 00 00 
   317d7:	48 c7 84 24 d8 01 00 	movq   $0xffffffffffffcac9,0x1d8(%rsp)
   317de:	00 c9 ca ff ff 
   317e3:	48 c7 84 24 e0 01 00 	movq   $0x2d41,0x1e0(%rsp)
   317ea:	00 41 2d 00 00 
   317ef:	48 c7 84 24 e8 01 00 	movq   $0xffffffffffffdc72,0x1e8(%rsp)
   317f6:	00 72 dc ff ff 
   317fb:	48 c7 84 24 f0 01 00 	movq   $0x187e,0x1f0(%rsp)
   31802:	00 7e 18 00 00 
   31807:	48 c7 84 24 f8 01 00 	movq   $0xfffffffffffff384,0x1f8(%rsp)
   3180e:	00 84 f3 ff ff 
   31813:	4c 0f af 04 f9       	imul   (%rcx,%rdi,8),%r8
   31818:	4c 01 c6             	add    %r8,%rsi
   3181b:	48 ff c7             	inc    %rdi
   3181e:	48 83 ff 08          	cmp    $0x8,%rdi
   31822:	0f 85 18 fd ff ff    	jne    31540 <verified_tiff::jpeg_scan::decode_blocks+0x1a0>
   31828:	48 89 b4 c4 c0 02 00 	mov    %rsi,0x2c0(%rsp,%rax,8)
   3182f:	00 
   31830:	48 ff c0             	inc    %rax
   31833:	48 83 f8 40          	cmp    $0x40,%rax
   31837:	0f 85 d3 fc ff ff    	jne    31510 <verified_tiff::jpeg_scan::decode_blocks+0x170>
   3183d:	0f 57 c0             	xorps  %xmm0,%xmm0
   31840:	0f 29 84 24 70 02 00 	movaps %xmm0,0x270(%rsp)
   31847:	00 
   31848:	0f 29 84 24 60 02 00 	movaps %xmm0,0x260(%rsp)
   3184f:	00 
   31850:	0f 29 84 24 50 02 00 	movaps %xmm0,0x250(%rsp)
   31857:	00 
   31858:	0f 29 84 24 40 02 00 	movaps %xmm0,0x240(%rsp)
   3185f:	00 
   31860:	31 c0                	xor    %eax,%eax
   31862:	48 8b b4 24 08 07 00 	mov    0x708(%rsp),%rsi
   31869:	00 
   3186a:	49 ba 00 00 00 20 20 	movabs $0x2020000000,%r10
   31871:	00 00 00 
   31874:	4c 8b b4 24 30 02 00 	mov    0x230(%rsp),%r14
   3187b:	00 
   3187c:	4c 8b ac 24 28 02 00 	mov    0x228(%rsp),%r13
   31883:	00 
   31884:	eb 20                	jmp    318a6 <verified_tiff::jpeg_scan::decode_blocks+0x506>
   31886:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   3188d:	00 00 00 
   31890:	31 d2                	xor    %edx,%edx
   31892:	88 94 04 40 02 00 00 	mov    %dl,0x240(%rsp,%rax,1)
   31899:	48 ff c0             	inc    %rax
   3189c:	48 83 f8 40          	cmp    $0x40,%rax
   318a0:	0f 84 3a 03 00 00    	je     31be0 <verified_tiff::jpeg_scan::decode_blocks+0x840>
   318a6:	48 89 c1             	mov    %rax,%rcx
   318a9:	48 83 e1 f8          	and    $0xfffffffffffffff8,%rcx
   318ad:	48 8d 14 cc          	lea    (%rsp,%rcx,8),%rdx
   318b1:	89 c1                	mov    %eax,%ecx
   318b3:	83 e1 07             	and    $0x7,%ecx
   318b6:	4c 8d 0c cc          	lea    (%rsp,%rcx,8),%r9
   318ba:	49 81 c1 c0 02 00 00 	add    $0x2c0,%r9
   318c1:	31 ff                	xor    %edi,%edi
   318c3:	31 c9                	xor    %ecx,%ecx
   318c5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   318cc:	00 00 00 00 
   318d0:	4d 8b 04 f9          	mov    (%r9,%rdi,8),%r8
   318d4:	48 c7 04 24 41 2d 00 	movq   $0x2d41,(%rsp)
   318db:	00 
   318dc:	48 c7 44 24 08 c5 3e 	movq   $0x3ec5,0x8(%rsp)
   318e3:	00 00 
   318e5:	48 c7 44 24 10 21 3b 	movq   $0x3b21,0x10(%rsp)
   318ec:	00 00 
   318ee:	48 c7 44 24 18 37 35 	movq   $0x3537,0x18(%rsp)
   318f5:	00 00 
   318f7:	48 c7 44 24 20 41 2d 	movq   $0x2d41,0x20(%rsp)
   318fe:	00 00 
   31900:	48 c7 44 24 28 8e 23 	movq   $0x238e,0x28(%rsp)
   31907:	00 00 
   31909:	48 c7 44 24 30 7e 18 	movq   $0x187e,0x30(%rsp)
   31910:	00 00 
   31912:	48 c7 44 24 38 7c 0c 	movq   $0xc7c,0x38(%rsp)
   31919:	00 00 
   3191b:	48 c7 44 24 40 41 2d 	movq   $0x2d41,0x40(%rsp)
   31922:	00 00 
   31924:	48 c7 44 24 48 37 35 	movq   $0x3537,0x48(%rsp)
   3192b:	00 00 
   3192d:	48 c7 44 24 50 7e 18 	movq   $0x187e,0x50(%rsp)
   31934:	00 00 
   31936:	48 c7 44 24 58 84 f3 	movq   $0xfffffffffffff384,0x58(%rsp)
   3193d:	ff ff 
   3193f:	48 c7 44 24 60 bf d2 	movq   $0xffffffffffffd2bf,0x60(%rsp)
   31946:	ff ff 
   31948:	48 c7 44 24 68 3b c1 	movq   $0xffffffffffffc13b,0x68(%rsp)
   3194f:	ff ff 
   31951:	48 c7 44 24 70 df c4 	movq   $0xffffffffffffc4df,0x70(%rsp)
   31958:	ff ff 
   3195a:	48 c7 44 24 78 72 dc 	movq   $0xffffffffffffdc72,0x78(%rsp)
   31961:	ff ff 
   31963:	48 c7 84 24 80 00 00 	movq   $0x2d41,0x80(%rsp)
   3196a:	00 41 2d 00 00 
   3196f:	48 c7 84 24 88 00 00 	movq   $0x238e,0x88(%rsp)
   31976:	00 8e 23 00 00 
   3197b:	48 c7 84 24 90 00 00 	movq   $0xffffffffffffe782,0x90(%rsp)
   31982:	00 82 e7 ff ff 
   31987:	48 c7 84 24 98 00 00 	movq   $0xffffffffffffc13b,0x98(%rsp)
   3198e:	00 3b c1 ff ff 
   31993:	48 c7 84 24 a0 00 00 	movq   $0xffffffffffffd2bf,0xa0(%rsp)
   3199a:	00 bf d2 ff ff 
   3199f:	48 c7 84 24 a8 00 00 	movq   $0xc7c,0xa8(%rsp)
   319a6:	00 7c 0c 00 00 
   319ab:	48 c7 84 24 b0 00 00 	movq   $0x3b21,0xb0(%rsp)
   319b2:	00 21 3b 00 00 
   319b7:	48 c7 84 24 b8 00 00 	movq   $0x3537,0xb8(%rsp)
   319be:	00 37 35 00 00 
   319c3:	48 c7 84 24 c0 00 00 	movq   $0x2d41,0xc0(%rsp)
   319ca:	00 41 2d 00 00 
   319cf:	48 c7 84 24 c8 00 00 	movq   $0xc7c,0xc8(%rsp)
   319d6:	00 7c 0c 00 00 
   319db:	48 c7 84 24 d0 00 00 	movq   $0xffffffffffffc4df,0xd0(%rsp)
   319e2:	00 df c4 ff ff 
   319e7:	48 c7 84 24 d8 00 00 	movq   $0xffffffffffffdc72,0xd8(%rsp)
   319ee:	00 72 dc ff ff 
   319f3:	48 c7 84 24 e0 00 00 	movq   $0x2d41,0xe0(%rsp)
   319fa:	00 41 2d 00 00 
   319ff:	48 c7 84 24 e8 00 00 	movq   $0x3537,0xe8(%rsp)
   31a06:	00 37 35 00 00 
   31a0b:	48 c7 84 24 f0 00 00 	movq   $0xffffffffffffe782,0xf0(%rsp)
   31a12:	00 82 e7 ff ff 
   31a17:	48 c7 84 24 f8 00 00 	movq   $0xffffffffffffc13b,0xf8(%rsp)
   31a1e:	00 3b c1 ff ff 
   31a23:	48 c7 84 24 00 01 00 	movq   $0x2d41,0x100(%rsp)
   31a2a:	00 41 2d 00 00 
   31a2f:	48 c7 84 24 08 01 00 	movq   $0xfffffffffffff384,0x108(%rsp)
   31a36:	00 84 f3 ff ff 
   31a3b:	48 c7 84 24 10 01 00 	movq   $0xffffffffffffc4df,0x110(%rsp)
   31a42:	00 df c4 ff ff 
   31a47:	48 c7 84 24 18 01 00 	movq   $0x238e,0x118(%rsp)
   31a4e:	00 8e 23 00 00 
   31a53:	48 c7 84 24 20 01 00 	movq   $0x2d41,0x120(%rsp)
   31a5a:	00 41 2d 00 00 
   31a5f:	48 c7 84 24 28 01 00 	movq   $0xffffffffffffcac9,0x128(%rsp)
   31a66:	00 c9 ca ff ff 
   31a6b:	48 c7 84 24 30 01 00 	movq   $0xffffffffffffe782,0x130(%rsp)
   31a72:	00 82 e7 ff ff 
   31a77:	48 c7 84 24 38 01 00 	movq   $0x3ec5,0x138(%rsp)
   31a7e:	00 c5 3e 00 00 
   31a83:	48 c7 84 24 40 01 00 	movq   $0x2d41,0x140(%rsp)
   31a8a:	00 41 2d 00 00 
   31a8f:	48 c7 84 24 48 01 00 	movq   $0xffffffffffffdc72,0x148(%rsp)
   31a96:	00 72 dc ff ff 
   31a9b:	48 c7 84 24 50 01 00 	movq   $0xffffffffffffe782,0x150(%rsp)
   31aa2:	00 82 e7 ff ff 
   31aa7:	48 c7 84 24 58 01 00 	movq   $0x3ec5,0x158(%rsp)
   31aae:	00 c5 3e 00 00 
   31ab3:	48 c7 84 24 60 01 00 	movq   $0xffffffffffffd2bf,0x160(%rsp)
   31aba:	00 bf d2 ff ff 
   31abf:	48 c7 84 24 68 01 00 	movq   $0xfffffffffffff384,0x168(%rsp)
   31ac6:	00 84 f3 ff ff 
   31acb:	48 c7 84 24 70 01 00 	movq   $0x3b21,0x170(%rsp)
   31ad2:	00 21 3b 00 00 
   31ad7:	48 c7 84 24 78 01 00 	movq   $0xffffffffffffcac9,0x178(%rsp)
   31ade:	00 c9 ca ff ff 
   31ae3:	48 c7 84 24 80 01 00 	movq   $0x2d41,0x180(%rsp)
   31aea:	00 41 2d 00 00 
   31aef:	48 c7 84 24 88 01 00 	movq   $0xffffffffffffcac9,0x188(%rsp)
   31af6:	00 c9 ca ff ff 
   31afb:	48 c7 84 24 90 01 00 	movq   $0x187e,0x190(%rsp)
   31b02:	00 7e 18 00 00 
   31b07:	48 c7 84 24 98 01 00 	movq   $0xc7c,0x198(%rsp)
   31b0e:	00 7c 0c 00 00 
   31b13:	48 c7 84 24 a0 01 00 	movq   $0xffffffffffffd2bf,0x1a0(%rsp)
   31b1a:	00 bf d2 ff ff 
   31b1f:	48 c7 84 24 a8 01 00 	movq   $0x3ec5,0x1a8(%rsp)
   31b26:	00 c5 3e 00 00 
   31b2b:	48 c7 84 24 b0 01 00 	movq   $0xffffffffffffc4df,0x1b0(%rsp)
   31b32:	00 df c4 ff ff 
   31b37:	48 c7 84 24 b8 01 00 	movq   $0x238e,0x1b8(%rsp)
   31b3e:	00 8e 23 00 00 
   31b43:	48 c7 84 24 c0 01 00 	movq   $0x2d41,0x1c0(%rsp)
   31b4a:	00 41 2d 00 00 
   31b4f:	48 c7 84 24 c8 01 00 	movq   $0xffffffffffffc13b,0x1c8(%rsp)
   31b56:	00 3b c1 ff ff 
   31b5b:	48 c7 84 24 d0 01 00 	movq   $0x3b21,0x1d0(%rsp)
   31b62:	00 21 3b 00 00 
   31b67:	48 c7 84 24 d8 01 00 	movq   $0xffffffffffffcac9,0x1d8(%rsp)
   31b6e:	00 c9 ca ff ff 
   31b73:	48 c7 84 24 e0 01 00 	movq   $0x2d41,0x1e0(%rsp)
   31b7a:	00 41 2d 00 00 
   31b7f:	48 c7 84 24 e8 01 00 	movq   $0xffffffffffffdc72,0x1e8(%rsp)
   31b86:	00 72 dc ff ff 
   31b8b:	48 c7 84 24 f0 01 00 	movq   $0x187e,0x1f0(%rsp)
   31b92:	00 7e 18 00 00 
   31b97:	48 c7 84 24 f8 01 00 	movq   $0xfffffffffffff384,0x1f8(%rsp)
   31b9e:	00 84 f3 ff ff 
   31ba3:	4c 0f af 04 3a       	imul   (%rdx,%rdi,1),%r8
   31ba8:	4c 01 c1             	add    %r8,%rcx
   31bab:	48 83 c7 08          	add    $0x8,%rdi
   31baf:	48 83 ff 40          	cmp    $0x40,%rdi
   31bb3:	0f 85 17 fd ff ff    	jne    318d0 <verified_tiff::jpeg_scan::decode_blocks+0x530>
   31bb9:	4c 01 d1             	add    %r10,%rcx
   31bbc:	48 85 c9             	test   %rcx,%rcx
   31bbf:	0f 8e cb fc ff ff    	jle    31890 <verified_tiff::jpeg_scan::decode_blocks+0x4f0>
   31bc5:	48 c1 e9 1e          	shr    $0x1e,%rcx
   31bc9:	b2 ff                	mov    $0xff,%dl
   31bcb:	48 81 f9 fe 00 00 00 	cmp    $0xfe,%rcx
   31bd2:	0f 87 ba fc ff ff    	ja     31892 <verified_tiff::jpeg_scan::decode_blocks+0x4f2>
   31bd8:	89 ca                	mov    %ecx,%edx
   31bda:	e9 b3 fc ff ff       	jmp    31892 <verified_tiff::jpeg_scan::decode_blocks+0x4f2>
   31bdf:	90                   	nop
   31be0:	48 83 bc 24 38 02 00 	cmpq   $0xf,0x238(%rsp)
   31be7:	00 0f 
   31be9:	77 05                	ja     31bf0 <verified_tiff::jpeg_scan::decode_blocks+0x850>
   31beb:	45 31 ff             	xor    %r15d,%r15d
   31bee:	eb 36                	jmp    31c26 <verified_tiff::jpeg_scan::decode_blocks+0x886>
   31bf0:	44 89 f8             	mov    %r15d,%eax
   31bf3:	83 e0 0f             	and    $0xf,%eax
   31bf6:	b9 10 00 00 00       	mov    $0x10,%ecx
   31bfb:	48 0f 44 c1          	cmove  %rcx,%rax
   31bff:	49 29 c7             	sub    %rax,%r15
   31c02:	48 29 c5             	sub    %rax,%rbp
   31c05:	48 ff c5             	inc    %rbp
   31c08:	31 c0                	xor    %eax,%eax
   31c0a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   31c10:	0f 10 84 04 40 02 00 	movups 0x240(%rsp,%rax,1),%xmm0
   31c17:	00 
   31c18:	41 0f 11 04 04       	movups %xmm0,(%r12,%rax,1)
   31c1d:	48 83 c0 10          	add    $0x10,%rax
   31c21:	48 39 c5             	cmp    %rax,%rbp
   31c24:	75 ea                	jne    31c10 <verified_tiff::jpeg_scan::decode_blocks+0x870>
   31c26:	48 8b 84 24 00 07 00 	mov    0x700(%rsp),%rax
   31c2d:	00 
   31c2e:	48 01 d8             	add    %rbx,%rax
   31c31:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   31c38:	0f 1f 84 00 00 00 00 
   31c3f:	00 
   31c40:	4a 8d 3c 3b          	lea    (%rbx,%r15,1),%rdi
   31c44:	48 39 f7             	cmp    %rsi,%rdi
   31c47:	73 4e                	jae    31c97 <verified_tiff::jpeg_scan::decode_blocks+0x8f7>
   31c49:	42 0f b6 8c 3c 40 02 	movzbl 0x240(%rsp,%r15,1),%ecx
   31c50:	00 00 
   31c52:	42 88 0c 38          	mov    %cl,(%rax,%r15,1)
   31c56:	49 ff c7             	inc    %r15
   31c59:	49 83 ff 40          	cmp    $0x40,%r15
   31c5d:	75 e1                	jne    31c40 <verified_tiff::jpeg_scan::decode_blocks+0x8a0>
   31c5f:	49 ff c5             	inc    %r13
   31c62:	49 83 c4 40          	add    $0x40,%r12
   31c66:	48 83 c3 40          	add    $0x40,%rbx
   31c6a:	49 83 c6 c0          	add    $0xffffffffffffffc0,%r14
   31c6e:	4c 3b ac 24 08 02 00 	cmp    0x208(%rsp),%r13
   31c75:	00 
   31c76:	0f 85 b4 f7 ff ff    	jne    31430 <verified_tiff::jpeg_scan::decode_blocks+0x90>
   31c7c:	b0 ff                	mov    $0xff,%al
   31c7e:	eb 05                	jmp    31c85 <verified_tiff::jpeg_scan::decode_blocks+0x8e5>
   31c80:	0f b6 44 24 01       	movzbl 0x1(%rsp),%eax
   31c85:	48 81 c4 c8 06 00 00 	add    $0x6c8,%rsp
   31c8c:	5b                   	pop    %rbx
   31c8d:	41 5c                	pop    %r12
   31c8f:	41 5d                	pop    %r13
   31c91:	41 5e                	pop    %r14
   31c93:	41 5f                	pop    %r15
   31c95:	5d                   	pop    %rbp
   31c96:	c3                   	ret
   31c97:	48 8d 15 d2 49 05 00 	lea    0x549d2(%rip),%rdx        # 86670 <anon.ac36df6000eed6ce22b87f572999359a.32.llvm.17988912158883924058+0x2e8>
   31c9e:	ff 15 b4 7b 05 00    	call   *0x57bb4(%rip)        # 89858 <_DYNAMIC+0x388>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
