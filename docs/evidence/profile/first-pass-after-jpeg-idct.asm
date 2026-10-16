
target/release/examples/benchmark:     file format elf64-x86-64


Disassembly of section .text:

0000000000035380 <verified_tiff::jpeg_scan::decode_blocks>:
   35380:	55                   	push   %rbp
   35381:	41 57                	push   %r15
   35383:	41 56                	push   %r14
   35385:	41 55                	push   %r13
   35387:	41 54                	push   %r12
   35389:	53                   	push   %rbx
   3538a:	48 81 ec 98 06 00 00 	sub    $0x698,%rsp
   35391:	48 89 54 24 20       	mov    %rdx,0x20(%rsp)
   35396:	48 89 74 24 18       	mov    %rsi,0x18(%rsp)
   3539b:	48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
   353a0:	0f 57 c0             	xorps  %xmm0,%xmm0
   353a3:	0f 29 44 24 60       	movaps %xmm0,0x60(%rsp)
   353a8:	48 c7 44 24 70 00 00 	movq   $0x0,0x70(%rsp)
   353af:	00 00 
   353b1:	0f 10 01             	movups (%rcx),%xmm0
   353b4:	0f 10 49 10          	movups 0x10(%rcx),%xmm1
   353b8:	0f 29 44 24 40       	movaps %xmm0,0x40(%rsp)
   353bd:	0f 29 4c 24 50       	movaps %xmm1,0x50(%rsp)
   353c2:	48 c7 44 24 78 00 00 	movq   $0x0,0x78(%rsp)
   353c9:	00 00 
   353cb:	b0 ff                	mov    $0xff,%al
   353cd:	4c 89 44 24 08       	mov    %r8,0x8(%rsp)
   353d2:	4d 85 c0             	test   %r8,%r8
   353d5:	0f 84 7c 03 00 00    	je     35757 <verified_tiff::jpeg_scan::decode_blocks+0x3d7>
   353db:	48 8b b4 24 d8 06 00 	mov    0x6d8(%rsp),%rsi
   353e2:	00 
   353e3:	48 8b 9c 24 d0 06 00 	mov    0x6d0(%rsp),%rbx
   353ea:	00 
   353eb:	45 31 ff             	xor    %r15d,%r15d
   353ee:	4c 8d ac 24 88 00 00 	lea    0x88(%rsp),%r13
   353f5:	00 
   353f6:	4c 8d 35 9b 54 fd ff 	lea    -0x2ab65(%rip),%r14        # a898 <anon.53fc985f19b08ffe8bb527278e35fb9b.6.llvm.16823691178101282037+0x98>
   353fd:	31 c0                	xor    %eax,%eax
   353ff:	31 c9                	xor    %ecx,%ecx
   35401:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   35408:	0f 1f 84 00 00 00 00 
   3540f:	00 
   35410:	4c 39 fe             	cmp    %r15,%rsi
   35413:	4c 89 fd             	mov    %r15,%rbp
   35416:	48 0f 47 ee          	cmova  %rsi,%rbp
   3541a:	48 89 44 24 30       	mov    %rax,0x30(%rsp)
   3541f:	48 01 c5             	add    %rax,%rbp
   35422:	48 83 fd 3f          	cmp    $0x3f,%rbp
   35426:	b8 3f 00 00 00       	mov    $0x3f,%eax
   3542b:	48 0f 43 e8          	cmovae %rax,%rbp
   3542f:	48 89 4c 24 28       	mov    %rcx,0x28(%rsp)
   35434:	48 89 c8             	mov    %rcx,%rax
   35437:	48 c1 e0 06          	shl    $0x6,%rax
   3543b:	48 89 f1             	mov    %rsi,%rcx
   3543e:	48 29 c1             	sub    %rax,%rcx
   35441:	b8 00 00 00 00       	mov    $0x0,%eax
   35446:	48 0f 43 c1          	cmovae %rcx,%rax
   3544a:	48 83 f8 3f          	cmp    $0x3f,%rax
   3544e:	41 bc 3f 00 00 00    	mov    $0x3f,%r12d
   35454:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   35459:	4c 0f 42 e0          	cmovb  %rax,%r12
   3545d:	4c 89 ef             	mov    %r13,%rdi
   35460:	48 8d 74 24 40       	lea    0x40(%rsp),%rsi
   35465:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
   3546a:	48 8b 4c 24 18       	mov    0x18(%rsp),%rcx
   3546f:	4c 8b 44 24 20       	mov    0x20(%rsp),%r8
   35474:	e8 97 dc ff ff       	call   33110 <<verified_tiff::jpeg_scan::Cursor>::next>
   35479:	80 bc 24 88 00 00 00 	cmpb   $0x0,0x88(%rsp)
   35480:	00 
   35481:	0f 85 c8 02 00 00    	jne    3574f <verified_tiff::jpeg_scan::decode_blocks+0x3cf>
   35487:	49 ff c4             	inc    %r12
   3548a:	ba 00 02 00 00       	mov    $0x200,%edx
   3548f:	48 8d bc 24 96 04 00 	lea    0x496(%rsp),%rdi
   35496:	00 
   35497:	48 8d b4 24 90 00 00 	lea    0x90(%rsp),%rsi
   3549e:	00 
   3549f:	ff 15 cb 3f 05 00    	call   *0x53fcb(%rip)        # 89470 <memcpy@GLIBC_2.14>
   354a5:	ba 00 02 00 00       	mov    $0x200,%edx
   354aa:	48 8d bc 24 90 02 00 	lea    0x290(%rsp),%rdi
   354b1:	00 
   354b2:	48 8d b4 24 96 04 00 	lea    0x496(%rsp),%rsi
   354b9:	00 
   354ba:	ff 15 b0 3f 05 00    	call   *0x53fb0(%rip)        # 89470 <memcpy@GLIBC_2.14>
   354c0:	ba 00 02 00 00       	mov    $0x200,%edx
   354c5:	4c 89 ef             	mov    %r13,%rdi
   354c8:	31 f6                	xor    %esi,%esi
   354ca:	ff 15 d8 40 05 00    	call   *0x540d8(%rip)        # 895a8 <memset@GLIBC_2.2.5>
   354d0:	31 c0                	xor    %eax,%eax
   354d2:	31 c9                	xor    %ecx,%ecx
   354d4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   354db:	00 00 00 00 00 
   354e0:	89 c2                	mov    %eax,%edx
   354e2:	83 e2 38             	and    $0x38,%edx
   354e5:	89 ce                	mov    %ecx,%esi
   354e7:	83 e6 38             	and    $0x38,%esi
   354ea:	49 8b 3c d6          	mov    (%r14,%rdx,8),%rdi
   354ee:	48 0f af bc f4 90 02 	imul   0x290(%rsp,%rsi,8),%rdi
   354f5:	00 00 
   354f7:	4d 8b 44 d6 08       	mov    0x8(%r14,%rdx,8),%r8
   354fc:	4c 0f af 84 f4 98 02 	imul   0x298(%rsp,%rsi,8),%r8
   35503:	00 00 
   35505:	49 01 f8             	add    %rdi,%r8
   35508:	49 8b 7c d6 10       	mov    0x10(%r14,%rdx,8),%rdi
   3550d:	48 0f af bc f4 a0 02 	imul   0x2a0(%rsp,%rsi,8),%rdi
   35514:	00 00 
   35516:	4d 8b 4c d6 18       	mov    0x18(%r14,%rdx,8),%r9
   3551b:	4c 0f af 8c f4 a8 02 	imul   0x2a8(%rsp,%rsi,8),%r9
   35522:	00 00 
   35524:	49 01 f9             	add    %rdi,%r9
   35527:	4d 01 c1             	add    %r8,%r9
   3552a:	49 8b 7c d6 20       	mov    0x20(%r14,%rdx,8),%rdi
   3552f:	48 0f af bc f4 b0 02 	imul   0x2b0(%rsp,%rsi,8),%rdi
   35536:	00 00 
   35538:	4d 8b 44 d6 28       	mov    0x28(%r14,%rdx,8),%r8
   3553d:	4c 0f af 84 f4 b8 02 	imul   0x2b8(%rsp,%rsi,8),%r8
   35544:	00 00 
   35546:	49 01 f8             	add    %rdi,%r8
   35549:	49 8b 7c d6 30       	mov    0x30(%r14,%rdx,8),%rdi
   3554e:	48 0f af bc f4 c0 02 	imul   0x2c0(%rsp,%rsi,8),%rdi
   35555:	00 00 
   35557:	4c 01 c7             	add    %r8,%rdi
   3555a:	4c 01 cf             	add    %r9,%rdi
   3555d:	49 8b 54 d6 38       	mov    0x38(%r14,%rdx,8),%rdx
   35562:	48 0f af 94 f4 c8 02 	imul   0x2c8(%rsp,%rsi,8),%rdx
   35569:	00 00 
   3556b:	48 01 fa             	add    %rdi,%rdx
   3556e:	48 89 94 cc 88 00 00 	mov    %rdx,0x88(%rsp,%rcx,8)
   35575:	00 
   35576:	48 ff c1             	inc    %rcx
   35579:	48 83 c0 08          	add    $0x8,%rax
   3557d:	48 83 f9 40          	cmp    $0x40,%rcx
   35581:	0f 85 59 ff ff ff    	jne    354e0 <verified_tiff::jpeg_scan::decode_blocks+0x160>
   35587:	0f 57 c0             	xorps  %xmm0,%xmm0
   3558a:	0f 29 84 24 c0 04 00 	movaps %xmm0,0x4c0(%rsp)
   35591:	00 
   35592:	0f 29 84 24 b0 04 00 	movaps %xmm0,0x4b0(%rsp)
   35599:	00 
   3559a:	0f 29 84 24 a0 04 00 	movaps %xmm0,0x4a0(%rsp)
   355a1:	00 
   355a2:	0f 29 84 24 90 04 00 	movaps %xmm0,0x490(%rsp)
   355a9:	00 
   355aa:	31 c0                	xor    %eax,%eax
   355ac:	49 b9 00 00 00 20 20 	movabs $0x2020000000,%r9
   355b3:	00 00 00 
   355b6:	eb 1e                	jmp    355d6 <verified_tiff::jpeg_scan::decode_blocks+0x256>
   355b8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   355bf:	00 
   355c0:	31 c9                	xor    %ecx,%ecx
   355c2:	88 8c 04 90 04 00 00 	mov    %cl,0x490(%rsp,%rax,1)
   355c9:	48 ff c0             	inc    %rax
   355cc:	48 83 f8 40          	cmp    $0x40,%rax
   355d0:	0f 84 ba 00 00 00    	je     35690 <verified_tiff::jpeg_scan::decode_blocks+0x310>
   355d6:	89 c2                	mov    %eax,%edx
   355d8:	83 e2 38             	and    $0x38,%edx
   355db:	89 c1                	mov    %eax,%ecx
   355dd:	83 e1 07             	and    $0x7,%ecx
   355e0:	49 8b 34 d6          	mov    (%r14,%rdx,8),%rsi
   355e4:	49 8b 7c d6 08       	mov    0x8(%r14,%rdx,8),%rdi
   355e9:	48 0f af b4 cc 88 00 	imul   0x88(%rsp,%rcx,8),%rsi
   355f0:	00 00 
   355f2:	48 0f af bc cc c8 00 	imul   0xc8(%rsp,%rcx,8),%rdi
   355f9:	00 00 
   355fb:	4d 8b 44 d6 10       	mov    0x10(%r14,%rdx,8),%r8
   35600:	4c 0f af 84 cc 08 01 	imul   0x108(%rsp,%rcx,8),%r8
   35607:	00 00 
   35609:	48 01 f7             	add    %rsi,%rdi
   3560c:	49 8b 74 d6 18       	mov    0x18(%r14,%rdx,8),%rsi
   35611:	48 0f af b4 cc 48 01 	imul   0x148(%rsp,%rcx,8),%rsi
   35618:	00 00 
   3561a:	4c 01 c6             	add    %r8,%rsi
   3561d:	4d 8b 44 d6 20       	mov    0x20(%r14,%rdx,8),%r8
   35622:	4c 0f af 84 cc 88 01 	imul   0x188(%rsp,%rcx,8),%r8
   35629:	00 00 
   3562b:	48 01 fe             	add    %rdi,%rsi
   3562e:	49 8b 7c d6 28       	mov    0x28(%r14,%rdx,8),%rdi
   35633:	48 0f af bc cc c8 01 	imul   0x1c8(%rsp,%rcx,8),%rdi
   3563a:	00 00 
   3563c:	4c 01 c7             	add    %r8,%rdi
   3563f:	4d 8b 44 d6 30       	mov    0x30(%r14,%rdx,8),%r8
   35644:	4c 0f af 84 cc 08 02 	imul   0x208(%rsp,%rcx,8),%r8
   3564b:	00 00 
   3564d:	49 01 f8             	add    %rdi,%r8
   35650:	49 8b 54 d6 38       	mov    0x38(%r14,%rdx,8),%rdx
   35655:	48 0f af 94 cc 48 02 	imul   0x248(%rsp,%rcx,8),%rdx
   3565c:	00 00 
   3565e:	49 01 f0             	add    %rsi,%r8
   35661:	4c 01 ca             	add    %r9,%rdx
   35664:	4c 01 c2             	add    %r8,%rdx
   35667:	48 85 d2             	test   %rdx,%rdx
   3566a:	0f 8e 50 ff ff ff    	jle    355c0 <verified_tiff::jpeg_scan::decode_blocks+0x240>
   35670:	48 c1 ea 1e          	shr    $0x1e,%rdx
   35674:	b1 ff                	mov    $0xff,%cl
   35676:	48 81 fa fe 00 00 00 	cmp    $0xfe,%rdx
   3567d:	0f 87 3f ff ff ff    	ja     355c2 <verified_tiff::jpeg_scan::decode_blocks+0x242>
   35683:	89 d1                	mov    %edx,%ecx
   35685:	e9 38 ff ff ff       	jmp    355c2 <verified_tiff::jpeg_scan::decode_blocks+0x242>
   3568a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   35690:	48 83 7c 24 38 0f    	cmpq   $0xf,0x38(%rsp)
   35696:	77 18                	ja     356b0 <verified_tiff::jpeg_scan::decode_blocks+0x330>
   35698:	45 31 e4             	xor    %r12d,%r12d
   3569b:	48 8b b4 24 d8 06 00 	mov    0x6d8(%rsp),%rsi
   356a2:	00 
   356a3:	eb 50                	jmp    356f5 <verified_tiff::jpeg_scan::decode_blocks+0x375>
   356a5:	66 66 2e 0f 1f 84 00 	data16 cs nopw 0x0(%rax,%rax,1)
   356ac:	00 00 00 00 
   356b0:	44 89 e0             	mov    %r12d,%eax
   356b3:	83 e0 0f             	and    $0xf,%eax
   356b6:	b9 10 00 00 00       	mov    $0x10,%ecx
   356bb:	48 0f 44 c1          	cmove  %rcx,%rax
   356bf:	49 29 c4             	sub    %rax,%r12
   356c2:	48 29 c5             	sub    %rax,%rbp
   356c5:	48 ff c5             	inc    %rbp
   356c8:	31 c0                	xor    %eax,%eax
   356ca:	48 8b b4 24 d8 06 00 	mov    0x6d8(%rsp),%rsi
   356d1:	00 
   356d2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   356d9:	1f 84 00 00 00 00 00 
   356e0:	0f 10 84 04 90 04 00 	movups 0x490(%rsp,%rax,1),%xmm0
   356e7:	00 
   356e8:	0f 11 04 03          	movups %xmm0,(%rbx,%rax,1)
   356ec:	48 83 c0 10          	add    $0x10,%rax
   356f0:	48 39 c5             	cmp    %rax,%rbp
   356f3:	75 eb                	jne    356e0 <verified_tiff::jpeg_scan::decode_blocks+0x360>
   356f5:	48 8b 84 24 d0 06 00 	mov    0x6d0(%rsp),%rax
   356fc:	00 
   356fd:	4c 01 f8             	add    %r15,%rax
   35700:	4b 8d 3c 27          	lea    (%r15,%r12,1),%rdi
   35704:	48 39 f7             	cmp    %rsi,%rdi
   35707:	73 60                	jae    35769 <verified_tiff::jpeg_scan::decode_blocks+0x3e9>
   35709:	42 0f b6 8c 24 90 04 	movzbl 0x490(%rsp,%r12,1),%ecx
   35710:	00 00 
   35712:	42 88 0c 20          	mov    %cl,(%rax,%r12,1)
   35716:	49 ff c4             	inc    %r12
   35719:	49 83 fc 40          	cmp    $0x40,%r12
   3571d:	75 e1                	jne    35700 <verified_tiff::jpeg_scan::decode_blocks+0x380>
   3571f:	48 8b 4c 24 28       	mov    0x28(%rsp),%rcx
   35724:	48 ff c1             	inc    %rcx
   35727:	48 83 c3 40          	add    $0x40,%rbx
   3572b:	49 83 c7 40          	add    $0x40,%r15
   3572f:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   35734:	48 83 c0 c0          	add    $0xffffffffffffffc0,%rax
   35738:	48 3b 4c 24 08       	cmp    0x8(%rsp),%rcx
   3573d:	4c 8d ac 24 88 00 00 	lea    0x88(%rsp),%r13
   35744:	00 
   35745:	0f 85 c5 fc ff ff    	jne    35410 <verified_tiff::jpeg_scan::decode_blocks+0x90>
   3574b:	b0 ff                	mov    $0xff,%al
   3574d:	eb 08                	jmp    35757 <verified_tiff::jpeg_scan::decode_blocks+0x3d7>
   3574f:	0f b6 84 24 89 00 00 	movzbl 0x89(%rsp),%eax
   35756:	00 
   35757:	48 81 c4 98 06 00 00 	add    $0x698,%rsp
   3575e:	5b                   	pop    %rbx
   3575f:	41 5c                	pop    %r12
   35761:	41 5d                	pop    %r13
   35763:	41 5e                	pop    %r14
   35765:	41 5f                	pop    %r15
   35767:	5d                   	pop    %rbp
   35768:	c3                   	ret
   35769:	48 8d 15 98 12 05 00 	lea    0x51298(%rip),%rdx        # 86a08 <anon.53fc985f19b08ffe8bb527278e35fb9b.11.llvm.16823691178101282037+0x4b0>
   35770:	ff 15 fa 3d 05 00    	call   *0x53dfa(%rip)        # 89570 <_DYNAMIC+0x350>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
