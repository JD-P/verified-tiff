
target/release/examples/benchmark:     file format elf64-x86-64


Disassembly of section .text:

0000000000029600 <<verified_tiff::deflate::Cursor>::step>:
   29600:	55                   	push   %rbp
   29601:	41 57                	push   %r15
   29603:	41 56                	push   %r14
   29605:	41 55                	push   %r13
   29607:	41 54                	push   %r12
   29609:	53                   	push   %rbx
   2960a:	48 81 ec 00 10 00 00 	sub    $0x1000,%rsp
   29611:	48 c7 04 24 00 00 00 	movq   $0x0,(%rsp)
   29618:	00 
   29619:	48 81 ec 68 0a 00 00 	sub    $0xa68,%rsp
   29620:	0f b6 87 08 89 00 00 	movzbl 0x8908(%rdi),%eax
   29627:	48 83 f8 03          	cmp    $0x3,%rax
   2962b:	0f 87 7b 03 00 00    	ja     299ac <<verified_tiff::deflate::Cursor>::step+0x3ac>
   29631:	4c 8d 05 e8 0a fe ff 	lea    -0x1f518(%rip),%r8        # a120 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x76d>
   29638:	49 63 04 80          	movslq (%r8,%rax,4),%rax
   2963c:	4c 01 c0             	add    %r8,%rax
   2963f:	ff e0                	jmp    *%rax
   29641:	48 8b 8f c0 88 00 00 	mov    0x88c0(%rdi),%rcx
   29648:	4c 8b 8f d0 88 00 00 	mov    0x88d0(%rdi),%r9
   2964f:	66 bd 02 00          	mov    $0x2,%bp
   29653:	4c 39 c9             	cmp    %r9,%rcx
   29656:	0f 84 ec 0a 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2965c:	48 39 d1             	cmp    %rdx,%rcx
   2965f:	0f 83 aa 0b 00 00    	jae    2a20f <<verified_tiff::deflate::Cursor>::step+0xc0f>
   29665:	48 8d 9f c0 88 00 00 	lea    0x88c0(%rdi),%rbx
   2966c:	4c 8b 87 c8 88 00 00 	mov    0x88c8(%rdi),%r8
   29673:	49 83 f8 07          	cmp    $0x7,%r8
   29677:	41 bd 07 00 00 00    	mov    $0x7,%r13d
   2967d:	4d 0f 42 e8          	cmovb  %r8,%r13
   29681:	0f 85 e0 03 00 00    	jne    29a67 <<verified_tiff::deflate::Cursor>::step+0x467>
   29687:	48 8d 41 01          	lea    0x1(%rcx),%rax
   2968b:	48 89 03             	mov    %rax,(%rbx)
   2968e:	45 31 c0             	xor    %r8d,%r8d
   29691:	44 0f b6 34 0e       	movzbl (%rsi,%rcx,1),%r14d
   29696:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   2969d:	4c 39 c8             	cmp    %r9,%rax
   296a0:	0f 84 a2 0a 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   296a6:	e9 d7 03 00 00       	jmp    29a82 <<verified_tiff::deflate::Cursor>::step+0x482>
   296ab:	48 8d 9f c0 88 00 00 	lea    0x88c0(%rdi),%rbx
   296b2:	4c 8b 8f d0 88 00 00 	mov    0x88d0(%rdi),%r9
   296b9:	48 8b 87 c0 88 00 00 	mov    0x88c0(%rdi),%rax
   296c0:	4c 8b b7 c8 88 00 00 	mov    0x88c8(%rdi),%r14
   296c7:	49 c7 c2 f1 ff ff ff 	mov    $0xfffffffffffffff1,%r10
   296ce:	45 31 db             	xor    %r11d,%r11d
   296d1:	45 31 ff             	xor    %r15d,%r15d
   296d4:	eb 1c                	jmp    296f2 <<verified_tiff::deflate::Cursor>::step+0xf2>
   296d6:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   296dd:	00 00 00 
   296e0:	4d 01 fc             	add    %r15,%r12
   296e3:	4d 01 e4             	add    %r12,%r12
   296e6:	4d 89 e7             	mov    %r12,%r15
   296e9:	49 ff c2             	inc    %r10
   296ec:	0f 84 6d 03 00 00    	je     29a5f <<verified_tiff::deflate::Cursor>::step+0x45f>
   296f2:	66 bd 02 00          	mov    $0x2,%bp
   296f6:	4c 39 c8             	cmp    %r9,%rax
   296f9:	0f 84 49 0a 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   296ff:	48 39 d0             	cmp    %rdx,%rax
   29702:	0f 83 de 0a 00 00    	jae    2a1e6 <<verified_tiff::deflate::Cursor>::step+0xbe6>
   29708:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   2970d:	49 83 fe 07          	cmp    $0x7,%r14
   29711:	41 bd 07 00 00 00    	mov    $0x7,%r13d
   29717:	4d 0f 42 ee          	cmovb  %r14,%r13
   2971b:	45 31 c0             	xor    %r8d,%r8d
   2971e:	4d 0f a3 ec          	bt     %r13,%r12
   29722:	41 0f 92 c4          	setb   %r12b
   29726:	49 83 fe 07          	cmp    $0x7,%r14
   2972a:	74 14                	je     29740 <<verified_tiff::deflate::Cursor>::step+0x140>
   2972c:	49 ff c6             	inc    %r14
   2972f:	eb 18                	jmp    29749 <<verified_tiff::deflate::Cursor>::step+0x149>
   29731:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   29738:	0f 1f 84 00 00 00 00 
   2973f:	00 
   29740:	48 ff c0             	inc    %rax
   29743:	48 89 03             	mov    %rax,(%rbx)
   29746:	45 31 f6             	xor    %r14d,%r14d
   29749:	45 88 e0             	mov    %r12b,%r8b
   2974c:	4c 89 b7 c8 88 00 00 	mov    %r14,0x88c8(%rdi)
   29753:	4f 8d 1c 58          	lea    (%r8,%r11,2),%r11
   29757:	4e 8b a4 d7 80 80 00 	mov    0x8080(%rdi,%r10,8),%r12
   2975e:	00 
   2975f:	4d 89 d8             	mov    %r11,%r8
   29762:	4d 29 f8             	sub    %r15,%r8
   29765:	0f 82 75 ff ff ff    	jb     296e0 <<verified_tiff::deflate::Cursor>::step+0xe0>
   2976b:	4d 39 e0             	cmp    %r12,%r8
   2976e:	0f 83 6c ff ff ff    	jae    296e0 <<verified_tiff::deflate::Cursor>::step+0xe0>
   29774:	4e 03 84 d7 00 81 00 	add    0x8100(%rdi,%r10,8),%r8
   2977b:	00 
   2977c:	49 81 f8 20 01 00 00 	cmp    $0x120,%r8
   29783:	0f 83 9c 0a 00 00    	jae    2a225 <<verified_tiff::deflate::Cursor>::step+0xc25>
   29789:	46 0f b7 8c 47 00 81 	movzwl 0x8100(%rdi,%r8,2),%r9d
   29790:	00 00 
   29792:	49 81 f9 00 01 00 00 	cmp    $0x100,%r9
   29799:	0f 83 3c 03 00 00    	jae    29adb <<verified_tiff::deflate::Cursor>::step+0x4db>
   2979f:	48 8b b7 e8 88 00 00 	mov    0x88e8(%rdi),%rsi
   297a6:	41 b4 01             	mov    $0x1,%r12b
   297a9:	48 39 ce             	cmp    %rcx,%rsi
   297ac:	0f 84 99 09 00 00    	je     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   297b2:	89 f0                	mov    %esi,%eax
   297b4:	25 ff 7f 00 00       	and    $0x7fff,%eax
   297b9:	44 88 0c 07          	mov    %r9b,(%rdi,%rax,1)
   297bd:	4c 89 c9             	mov    %r9,%rcx
   297c0:	48 03 8f f8 88 00 00 	add    0x88f8(%rdi),%rcx
   297c7:	49 b8 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r8
   297ce:	00 0f 00 
   297d1:	48 89 c8             	mov    %rcx,%rax
   297d4:	49 f7 e0             	mul    %r8
   297d7:	48 89 c8             	mov    %rcx,%rax
   297da:	48 29 d0             	sub    %rdx,%rax
   297dd:	48 d1 e8             	shr    $1,%rax
   297e0:	48 01 d0             	add    %rdx,%rax
   297e3:	48 c1 e8 0f          	shr    $0xf,%rax
   297e7:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   297ee:	48 29 c1             	sub    %rax,%rcx
   297f1:	48 89 8f f8 88 00 00 	mov    %rcx,0x88f8(%rdi)
   297f8:	48 03 8f 00 89 00 00 	add    0x8900(%rdi),%rcx
   297ff:	48 89 c8             	mov    %rcx,%rax
   29802:	49 f7 e0             	mul    %r8
   29805:	48 89 c8             	mov    %rcx,%rax
   29808:	48 29 d0             	sub    %rdx,%rax
   2980b:	48 d1 e8             	shr    $1,%rax
   2980e:	48 01 d0             	add    %rdx,%rax
   29811:	48 c1 e8 0f          	shr    $0xf,%rax
   29815:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   2981c:	48 29 c1             	sub    %rax,%rcx
   2981f:	48 89 8f 00 89 00 00 	mov    %rcx,0x8900(%rdi)
   29826:	48 ff c6             	inc    %rsi
   29829:	48 89 b7 e8 88 00 00 	mov    %rsi,0x88e8(%rdi)
   29830:	66 bd 01 00          	mov    $0x1,%bp
   29834:	45 89 cc             	mov    %r9d,%r12d
   29837:	e9 0f 09 00 00       	jmp    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   2983c:	48 8b 87 d8 88 00 00 	mov    0x88d8(%rdi),%rax
   29843:	48 85 c0             	test   %rax,%rax
   29846:	0f 84 07 02 00 00    	je     29a53 <<verified_tiff::deflate::Cursor>::step+0x453>
   2984c:	48 8b 97 e0 88 00 00 	mov    0x88e0(%rdi),%rdx
   29853:	66 bd 02 00          	mov    $0x2,%bp
   29857:	41 b4 01             	mov    $0x1,%r12b
   2985a:	48 85 d2             	test   %rdx,%rdx
   2985d:	0f 84 e8 08 00 00    	je     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29863:	48 8b b7 e8 88 00 00 	mov    0x88e8(%rdi),%rsi
   2986a:	48 39 d6             	cmp    %rdx,%rsi
   2986d:	0f 82 d8 08 00 00    	jb     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29873:	48 3b 97 f0 88 00 00 	cmp    0x88f0(%rdi),%rdx
   2987a:	0f 87 cb 08 00 00    	ja     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29880:	41 89 f0             	mov    %esi,%r8d
   29883:	41 29 d0             	sub    %edx,%r8d
   29886:	41 81 e0 ff 7f 00 00 	and    $0x7fff,%r8d
   2988d:	46 0f b6 04 07       	movzbl (%rdi,%r8,1),%r8d
   29892:	48 ff c8             	dec    %rax
   29895:	48 89 87 d8 88 00 00 	mov    %rax,0x88d8(%rdi)
   2989c:	48 39 ce             	cmp    %rcx,%rsi
   2989f:	0f 84 a6 08 00 00    	je     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   298a5:	41 0f b6 c0          	movzbl %r8b,%eax
   298a9:	89 f1                	mov    %esi,%ecx
   298ab:	81 e1 ff 7f 00 00    	and    $0x7fff,%ecx
   298b1:	88 04 0f             	mov    %al,(%rdi,%rcx,1)
   298b4:	48 8b 8f f8 88 00 00 	mov    0x88f8(%rdi),%rcx
   298bb:	48 01 c1             	add    %rax,%rcx
   298be:	49 b9 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r9
   298c5:	00 0f 00 
   298c8:	48 89 c8             	mov    %rcx,%rax
   298cb:	49 f7 e1             	mul    %r9
   298ce:	48 89 c8             	mov    %rcx,%rax
   298d1:	48 29 d0             	sub    %rdx,%rax
   298d4:	48 d1 e8             	shr    $1,%rax
   298d7:	48 01 d0             	add    %rdx,%rax
   298da:	48 c1 e8 0f          	shr    $0xf,%rax
   298de:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   298e5:	48 29 c1             	sub    %rax,%rcx
   298e8:	48 89 8f f8 88 00 00 	mov    %rcx,0x88f8(%rdi)
   298ef:	48 03 8f 00 89 00 00 	add    0x8900(%rdi),%rcx
   298f6:	48 89 c8             	mov    %rcx,%rax
   298f9:	49 f7 e1             	mul    %r9
   298fc:	48 89 c8             	mov    %rcx,%rax
   298ff:	48 29 d0             	sub    %rdx,%rax
   29902:	48 d1 e8             	shr    $1,%rax
   29905:	48 01 d0             	add    %rdx,%rax
   29908:	48 c1 e8 0f          	shr    $0xf,%rax
   2990c:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   29913:	48 29 c1             	sub    %rax,%rcx
   29916:	48 89 8f 00 89 00 00 	mov    %rcx,0x8900(%rdi)
   2991d:	48 ff c6             	inc    %rsi
   29920:	48 89 b7 e8 88 00 00 	mov    %rsi,0x88e8(%rdi)
   29927:	66 bd 01 00          	mov    $0x1,%bp
   2992b:	45 89 c4             	mov    %r8d,%r12d
   2992e:	e9 18 08 00 00       	jmp    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29933:	4c 8b 87 d8 88 00 00 	mov    0x88d8(%rdi),%r8
   2993a:	4d 85 c0             	test   %r8,%r8
   2993d:	0f 84 a5 01 00 00    	je     29ae8 <<verified_tiff::deflate::Cursor>::step+0x4e8>
   29943:	48 8b 87 c0 88 00 00 	mov    0x88c0(%rdi),%rax
   2994a:	4c 8b 8f c8 88 00 00 	mov    0x88c8(%rdi),%r9
   29951:	4c 8b 9f d0 88 00 00 	mov    0x88d0(%rdi),%r11
   29958:	4d 85 c9             	test   %r9,%r9
   2995b:	0f 84 65 01 00 00    	je     29ac6 <<verified_tiff::deflate::Cursor>::step+0x4c6>
   29961:	66 bd 02 00          	mov    $0x2,%bp
   29965:	4c 39 d8             	cmp    %r11,%rax
   29968:	0f 84 da 07 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2996e:	48 39 d0             	cmp    %rdx,%rax
   29971:	0f 83 85 08 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   29977:	49 83 f9 07          	cmp    $0x7,%r9
   2997b:	41 ba 07 00 00 00    	mov    $0x7,%r10d
   29981:	4d 0f 42 d1          	cmovb  %r9,%r10
   29985:	0f b6 1c 06          	movzbl (%rsi,%rax,1),%ebx
   29989:	44 0f a3 d3          	bt     %r10d,%ebx
   2998d:	0f 92 c3             	setb   %bl
   29990:	49 83 f9 07          	cmp    $0x7,%r9
   29994:	0f 85 0f 02 00 00    	jne    29ba9 <<verified_tiff::deflate::Cursor>::step+0x5a9>
   2999a:	48 ff c0             	inc    %rax
   2999d:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   299a4:	45 31 c9             	xor    %r9d,%r9d
   299a7:	e9 00 02 00 00       	jmp    29bac <<verified_tiff::deflate::Cursor>::step+0x5ac>
   299ac:	48 8b 8f c0 88 00 00 	mov    0x88c0(%rdi),%rcx
   299b3:	48 8b 87 d0 88 00 00 	mov    0x88d0(%rdi),%rax
   299ba:	49 89 c0             	mov    %rax,%r8
   299bd:	66 bd 02 00          	mov    $0x2,%bp
   299c1:	41 b4 01             	mov    $0x1,%r12b
   299c4:	49 29 c8             	sub    %rcx,%r8
   299c7:	74 0a                	je     299d3 <<verified_tiff::deflate::Cursor>::step+0x3d3>
   299c9:	49 83 f8 01          	cmp    $0x1,%r8
   299cd:	0f 85 78 07 00 00    	jne    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   299d3:	48 39 c8             	cmp    %rcx,%rax
   299d6:	74 0e                	je     299e6 <<verified_tiff::deflate::Cursor>::step+0x3e6>
   299d8:	48 83 bf c8 88 00 00 	cmpq   $0x0,0x88c8(%rdi)
   299df:	00 
   299e0:	0f 84 65 07 00 00    	je     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   299e6:	48 89 d1             	mov    %rdx,%rcx
   299e9:	48 29 c1             	sub    %rax,%rcx
   299ec:	41 0f 92 c0          	setb   %r8b
   299f0:	48 83 f9 04          	cmp    $0x4,%rcx
   299f4:	0f 92 c1             	setb   %cl
   299f7:	44 08 c1             	or     %r8b,%cl
   299fa:	0f 85 48 07 00 00    	jne    2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   29a00:	49 89 d0             	mov    %rdx,%r8
   29a03:	49 29 c0             	sub    %rax,%r8
   29a06:	0f 86 2e 08 00 00    	jbe    2a23a <<verified_tiff::deflate::Cursor>::step+0xc3a>
   29a0c:	48 8d 48 01          	lea    0x1(%rax),%rcx
   29a10:	48 39 d1             	cmp    %rdx,%rcx
   29a13:	0f 83 37 08 00 00    	jae    2a250 <<verified_tiff::deflate::Cursor>::step+0xc50>
   29a19:	49 83 c0 fe          	add    $0xfffffffffffffffe,%r8
   29a1d:	49 83 f8 01          	cmp    $0x1,%r8
   29a21:	0f 86 21 07 00 00    	jbe    2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   29a27:	8b 04 06             	mov    (%rsi,%rax,1),%eax
   29a2a:	0f c8                	bswap  %eax
   29a2c:	48 8b 8f 00 89 00 00 	mov    0x8900(%rdi),%rcx
   29a33:	48 c1 e1 10          	shl    $0x10,%rcx
   29a37:	48 03 8f f8 88 00 00 	add    0x88f8(%rdi),%rcx
   29a3e:	48 39 c8             	cmp    %rcx,%rax
   29a41:	0f 85 04 07 00 00    	jne    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29a47:	c6 87 08 89 00 00 05 	movb   $0x5,0x8908(%rdi)
   29a4e:	e9 a5 00 00 00       	jmp    29af8 <<verified_tiff::deflate::Cursor>::step+0x4f8>
   29a53:	c6 87 08 89 00 00 02 	movb   $0x2,0x8908(%rdi)
   29a5a:	e9 99 00 00 00       	jmp    29af8 <<verified_tiff::deflate::Cursor>::step+0x4f8>
   29a5f:	41 b4 01             	mov    $0x1,%r12b
   29a62:	e9 e4 06 00 00       	jmp    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29a67:	49 ff c0             	inc    %r8
   29a6a:	48 89 c8             	mov    %rcx,%rax
   29a6d:	44 0f b6 34 0e       	movzbl (%rsi,%rcx,1),%r14d
   29a72:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   29a79:	4c 39 c8             	cmp    %r9,%rax
   29a7c:	0f 84 c6 06 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   29a82:	48 39 d0             	cmp    %rdx,%rax
   29a85:	0f 83 71 07 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   29a8b:	49 83 f8 07          	cmp    $0x7,%r8
   29a8f:	b9 07 00 00 00       	mov    $0x7,%ecx
   29a94:	49 0f 42 c8          	cmovb  %r8,%rcx
   29a98:	44 0f b6 14 06       	movzbl (%rsi,%rax,1),%r10d
   29a9d:	41 0f a3 ca          	bt     %ecx,%r10d
   29aa1:	41 0f 92 c2          	setb   %r10b
   29aa5:	49 83 f8 07          	cmp    $0x7,%r8
   29aa9:	75 54                	jne    29aff <<verified_tiff::deflate::Cursor>::step+0x4ff>
   29aab:	48 ff c0             	inc    %rax
   29aae:	48 89 03             	mov    %rax,(%rbx)
   29ab1:	45 31 c0             	xor    %r8d,%r8d
   29ab4:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   29abb:	4c 39 c8             	cmp    %r9,%rax
   29abe:	0f 84 84 06 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   29ac4:	eb 4c                	jmp    29b12 <<verified_tiff::deflate::Cursor>::step+0x512>
   29ac6:	4c 39 d8             	cmp    %r11,%rax
   29ac9:	0f 85 9c 00 00 00    	jne    29b6b <<verified_tiff::deflate::Cursor>::step+0x56b>
   29acf:	66 bd 02 00          	mov    $0x2,%bp
   29ad3:	45 31 e4             	xor    %r12d,%r12d
   29ad6:	e9 70 06 00 00       	jmp    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29adb:	41 81 f9 00 01 00 00 	cmp    $0x100,%r9d
   29ae2:	0f 85 11 01 00 00    	jne    29bf9 <<verified_tiff::deflate::Cursor>::step+0x5f9>
   29ae8:	0f b6 87 09 89 00 00 	movzbl 0x8909(%rdi),%eax
   29aef:	c0 e0 02             	shl    $0x2,%al
   29af2:	88 87 08 89 00 00    	mov    %al,0x8908(%rdi)
   29af8:	31 ed                	xor    %ebp,%ebp
   29afa:	e9 4c 06 00 00       	jmp    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29aff:	49 ff c0             	inc    %r8
   29b02:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   29b09:	4c 39 c8             	cmp    %r9,%rax
   29b0c:	0f 84 36 06 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   29b12:	48 39 d0             	cmp    %rdx,%rax
   29b15:	0f 83 e1 06 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   29b1b:	44 0f b6 0c 06       	movzbl (%rsi,%rax,1),%r9d
   29b20:	49 83 f8 07          	cmp    $0x7,%r8
   29b24:	b9 07 00 00 00       	mov    $0x7,%ecx
   29b29:	49 0f 42 c8          	cmovb  %r8,%rcx
   29b2d:	41 d2 e9             	shr    %cl,%r9b
   29b30:	45 00 c9             	add    %r9b,%r9b
   29b33:	41 80 e1 02          	and    $0x2,%r9b
   29b37:	45 08 d1             	or     %r10b,%r9b
   29b3a:	49 83 f8 07          	cmp    $0x7,%r8
   29b3e:	0f 85 fc 01 00 00    	jne    29d40 <<verified_tiff::deflate::Cursor>::step+0x740>
   29b44:	48 ff c0             	inc    %rax
   29b47:	48 89 03             	mov    %rax,(%rbx)
   29b4a:	45 31 c0             	xor    %r8d,%r8d
   29b4d:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   29b54:	41 b4 01             	mov    $0x1,%r12b
   29b57:	41 0f b6 c9          	movzbl %r9b,%ecx
   29b5b:	4c 8d 0d ce 05 fe ff 	lea    -0x1fa32(%rip),%r9        # a130 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x77d>
   29b62:	49 63 0c 89          	movslq (%r9,%rcx,4),%rcx
   29b66:	4c 01 c9             	add    %r9,%rcx
   29b69:	ff e1                	jmp    *%rcx
   29b6b:	48 39 d0             	cmp    %rdx,%rax
   29b6e:	0f 83 f2 06 00 00    	jae    2a266 <<verified_tiff::deflate::Cursor>::step+0xc66>
   29b74:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   29b79:	48 ff c0             	inc    %rax
   29b7c:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   29b83:	49 ff c8             	dec    %r8
   29b86:	4c 89 87 d8 88 00 00 	mov    %r8,0x88d8(%rdi)
   29b8d:	48 8b b7 e8 88 00 00 	mov    0x88e8(%rdi),%rsi
   29b94:	48 39 ce             	cmp    %rcx,%rsi
   29b97:	0f 85 68 02 00 00    	jne    29e05 <<verified_tiff::deflate::Cursor>::step+0x805>
   29b9d:	66 bd 02 00          	mov    $0x2,%bp
   29ba1:	41 b4 01             	mov    $0x1,%r12b
   29ba4:	e9 a2 05 00 00       	jmp    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29ba9:	49 ff c1             	inc    %r9
   29bac:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   29bb3:	4c 39 d8             	cmp    %r11,%rax
   29bb6:	0f 84 8c 05 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   29bbc:	48 39 d0             	cmp    %rdx,%rax
   29bbf:	0f 83 37 06 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   29bc5:	49 89 ca             	mov    %rcx,%r10
   29bc8:	44 0f b6 34 06       	movzbl (%rsi,%rax,1),%r14d
   29bcd:	49 83 f9 07          	cmp    $0x7,%r9
   29bd1:	b9 07 00 00 00       	mov    $0x7,%ecx
   29bd6:	49 0f 42 c9          	cmovb  %r9,%rcx
   29bda:	41 d2 ee             	shr    %cl,%r14b
   29bdd:	49 83 f9 07          	cmp    $0x7,%r9
   29be1:	0f 85 a9 02 00 00    	jne    29e90 <<verified_tiff::deflate::Cursor>::step+0x890>
   29be7:	48 ff c0             	inc    %rax
   29bea:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   29bf1:	45 31 c9             	xor    %r9d,%r9d
   29bf4:	e9 9a 02 00 00       	jmp    29e93 <<verified_tiff::deflate::Cursor>::step+0x893>
   29bf9:	41 b4 01             	mov    $0x1,%r12b
   29bfc:	41 81 f9 1d 01 00 00 	cmp    $0x11d,%r9d
   29c03:	0f 87 42 05 00 00    	ja     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29c09:	48 89 4c 24 08       	mov    %rcx,0x8(%rsp)
   29c0e:	48 89 3c 24          	mov    %rdi,(%rsp)
   29c12:	48 8d 05 27 05 fe ff 	lea    -0x1fad9(%rip),%rax        # a140 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x78d>
   29c19:	4d 8d 81 fb fe ff ff 	lea    -0x105(%r9),%r8
   29c20:	49 c1 e8 02          	shr    $0x2,%r8
   29c24:	31 c9                	xor    %ecx,%ecx
   29c26:	49 81 f9 1d 01 00 00 	cmp    $0x11d,%r9
   29c2d:	4c 0f 44 c1          	cmove  %rcx,%r8
   29c31:	4e 8b ac c8 f8 f7 ff 	mov    -0x808(%rax,%r9,8),%r13
   29c38:	ff 
   29c39:	41 81 f9 09 01 00 00 	cmp    $0x109,%r9d
   29c40:	4c 0f 42 c1          	cmovb  %rcx,%r8
   29c44:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   29c49:	48 89 f0             	mov    %rsi,%rax
   29c4c:	48 89 de             	mov    %rbx,%rsi
   29c4f:	48 89 d1             	mov    %rdx,%rcx
   29c52:	49 89 c7             	mov    %rax,%r15
   29c55:	48 89 c2             	mov    %rax,%rdx
   29c58:	49 89 ce             	mov    %rcx,%r14
   29c5b:	e8 30 f5 ff ff       	call   29190 <<verified_tiff::deflate_bits::Bits>::read>
   29c60:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   29c65:	0f 84 00 03 00 00    	je     29f6b <<verified_tiff::deflate::Cursor>::step+0x96b>
   29c6b:	4c 03 6c 24 20       	add    0x20(%rsp),%r13
   29c70:	48 8b 04 24          	mov    (%rsp),%rax
   29c74:	48 8d b0 60 84 00 00 	lea    0x8460(%rax),%rsi
   29c7b:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   29c80:	4c 89 fa             	mov    %r15,%rdx
   29c83:	4c 89 f1             	mov    %r14,%rcx
   29c86:	49 89 d8             	mov    %rbx,%r8
   29c89:	e8 92 0f 00 00       	call   2ac20 <<verified_tiff::deflate_bits::Huff>::read>
   29c8e:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   29c93:	0f 84 d2 02 00 00    	je     29f6b <<verified_tiff::deflate::Cursor>::step+0x96b>
   29c99:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   29c9e:	48 83 f8 1d          	cmp    $0x1d,%rax
   29ca2:	0f 87 a3 04 00 00    	ja     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29ca8:	48 8d 0d 79 05 fe ff 	lea    -0x1fa87(%rip),%rcx        # a228 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x875>
   29caf:	48 8b 0c c1          	mov    (%rcx,%rax,8),%rcx
   29cb3:	48 89 4c 24 10       	mov    %rcx,0x10(%rsp)
   29cb8:	48 89 c1             	mov    %rax,%rcx
   29cbb:	48 d1 e9             	shr    $1,%rcx
   29cbe:	48 ff c9             	dec    %rcx
   29cc1:	45 31 c0             	xor    %r8d,%r8d
   29cc4:	48 83 f8 04          	cmp    $0x4,%rax
   29cc8:	4c 0f 43 c1          	cmovae %rcx,%r8
   29ccc:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   29cd1:	48 89 de             	mov    %rbx,%rsi
   29cd4:	4c 89 fa             	mov    %r15,%rdx
   29cd7:	4c 89 f1             	mov    %r14,%rcx
   29cda:	e8 b1 f4 ff ff       	call   29190 <<verified_tiff::deflate_bits::Bits>::read>
   29cdf:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   29ce4:	0f 84 81 02 00 00    	je     29f6b <<verified_tiff::deflate::Cursor>::step+0x96b>
   29cea:	48 8b 04 24          	mov    (%rsp),%rax
   29cee:	48 8b 88 e8 88 00 00 	mov    0x88e8(%rax),%rcx
   29cf5:	48 8b 54 24 08       	mov    0x8(%rsp),%rdx
   29cfa:	48 29 ca             	sub    %rcx,%rdx
   29cfd:	49 39 d5             	cmp    %rdx,%r13
   29d00:	0f 87 45 04 00 00    	ja     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29d06:	48 8b 54 24 10       	mov    0x10(%rsp),%rdx
   29d0b:	48 03 54 24 20       	add    0x20(%rsp),%rdx
   29d10:	48 39 ca             	cmp    %rcx,%rdx
   29d13:	0f 87 32 04 00 00    	ja     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29d19:	48 3b 90 f0 88 00 00 	cmp    0x88f0(%rax),%rdx
   29d20:	0f 87 25 04 00 00    	ja     2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29d26:	48 89 90 e0 88 00 00 	mov    %rdx,0x88e0(%rax)
   29d2d:	4c 89 a8 d8 88 00 00 	mov    %r13,0x88d8(%rax)
   29d34:	c6 80 08 89 00 00 03 	movb   $0x3,0x8908(%rax)
   29d3b:	e9 b8 fd ff ff       	jmp    29af8 <<verified_tiff::deflate::Cursor>::step+0x4f8>
   29d40:	49 ff c0             	inc    %r8
   29d43:	4c 89 87 c8 88 00 00 	mov    %r8,0x88c8(%rdi)
   29d4a:	41 b4 01             	mov    $0x1,%r12b
   29d4d:	41 0f b6 c9          	movzbl %r9b,%ecx
   29d51:	4c 8d 0d d8 03 fe ff 	lea    -0x1fc28(%rip),%r9        # a130 <anon.c4e80b896a3e53712bf2c93b8497aafb.7.llvm.13193850590807550747+0x77d>
   29d58:	49 63 0c 89          	movslq (%r9,%rcx,4),%rcx
   29d5c:	4c 01 c9             	add    %r9,%rcx
   29d5f:	ff e1                	jmp    *%rcx
   29d61:	49 89 f7             	mov    %rsi,%r15
   29d64:	49 89 d4             	mov    %rdx,%r12
   29d67:	4d 85 c0             	test   %r8,%r8
   29d6a:	74 15                	je     29d81 <<verified_tiff::deflate::Cursor>::step+0x781>
   29d6c:	48 ff c0             	inc    %rax
   29d6f:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   29d76:	48 c7 87 c8 88 00 00 	movq   $0x0,0x88c8(%rdi)
   29d7d:	00 00 00 00 
   29d81:	48 89 3c 24          	mov    %rdi,(%rsp)
   29d85:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   29d8a:	41 b8 10 00 00 00    	mov    $0x10,%r8d
   29d90:	48 89 de             	mov    %rbx,%rsi
   29d93:	4c 89 fa             	mov    %r15,%rdx
   29d96:	4c 89 e1             	mov    %r12,%rcx
   29d99:	e8 f2 f3 ff ff       	call   29190 <<verified_tiff::deflate_bits::Bits>::read>
   29d9e:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   29da3:	0f 84 c2 01 00 00    	je     29f6b <<verified_tiff::deflate::Cursor>::step+0x96b>
   29da9:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   29dae:	48 89 44 24 08       	mov    %rax,0x8(%rsp)
   29db3:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   29db8:	41 b8 10 00 00 00    	mov    $0x10,%r8d
   29dbe:	48 89 de             	mov    %rbx,%rsi
   29dc1:	4c 89 fa             	mov    %r15,%rdx
   29dc4:	4c 89 e1             	mov    %r12,%rcx
   29dc7:	e8 c4 f3 ff ff       	call   29190 <<verified_tiff::deflate_bits::Bits>::read>
   29dcc:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   29dd1:	0f 84 94 01 00 00    	je     29f6b <<verified_tiff::deflate::Cursor>::step+0x96b>
   29dd7:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   29ddc:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   29de1:	48 01 c8             	add    %rcx,%rax
   29de4:	41 b4 01             	mov    $0x1,%r12b
   29de7:	48 3d ff ff 00 00    	cmp    $0xffff,%rax
   29ded:	4c 8b 3c 24          	mov    (%rsp),%r15
   29df1:	0f 85 54 03 00 00    	jne    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29df7:	49 89 8f d8 88 00 00 	mov    %rcx,0x88d8(%r15)
   29dfe:	b1 01                	mov    $0x1,%cl
   29e00:	e9 34 02 00 00       	jmp    2a039 <<verified_tiff::deflate::Cursor>::step+0xa39>
   29e05:	41 0f b6 c4          	movzbl %r12b,%eax
   29e09:	89 f1                	mov    %esi,%ecx
   29e0b:	81 e1 ff 7f 00 00    	and    $0x7fff,%ecx
   29e11:	88 04 0f             	mov    %al,(%rdi,%rcx,1)
   29e14:	48 8b 8f f8 88 00 00 	mov    0x88f8(%rdi),%rcx
   29e1b:	48 01 c1             	add    %rax,%rcx
   29e1e:	49 b8 cd c5 2f 0d e1 	movabs $0xf00e10d2fc5cd,%r8
   29e25:	00 0f 00 
   29e28:	48 89 c8             	mov    %rcx,%rax
   29e2b:	49 f7 e0             	mul    %r8
   29e2e:	48 89 c8             	mov    %rcx,%rax
   29e31:	48 29 d0             	sub    %rdx,%rax
   29e34:	48 d1 e8             	shr    $1,%rax
   29e37:	48 01 d0             	add    %rdx,%rax
   29e3a:	48 c1 e8 0f          	shr    $0xf,%rax
   29e3e:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   29e45:	48 29 c1             	sub    %rax,%rcx
   29e48:	48 89 8f f8 88 00 00 	mov    %rcx,0x88f8(%rdi)
   29e4f:	48 03 8f 00 89 00 00 	add    0x8900(%rdi),%rcx
   29e56:	48 89 c8             	mov    %rcx,%rax
   29e59:	49 f7 e0             	mul    %r8
   29e5c:	48 89 c8             	mov    %rcx,%rax
   29e5f:	48 29 d0             	sub    %rdx,%rax
   29e62:	48 d1 e8             	shr    $1,%rax
   29e65:	48 01 d0             	add    %rdx,%rax
   29e68:	48 c1 e8 0f          	shr    $0xf,%rax
   29e6c:	48 69 c0 f1 ff 00 00 	imul   $0xfff1,%rax,%rax
   29e73:	48 29 c1             	sub    %rax,%rcx
   29e76:	48 89 8f 00 89 00 00 	mov    %rcx,0x8900(%rdi)
   29e7d:	48 ff c6             	inc    %rsi
   29e80:	48 89 b7 e8 88 00 00 	mov    %rsi,0x88e8(%rdi)
   29e87:	66 bd 01 00          	mov    $0x1,%bp
   29e8b:	e9 bb 02 00 00       	jmp    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29e90:	49 ff c1             	inc    %r9
   29e93:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   29e9a:	4c 39 d8             	cmp    %r11,%rax
   29e9d:	0f 84 a5 02 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   29ea3:	48 39 d0             	cmp    %rdx,%rax
   29ea6:	0f 83 50 03 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   29eac:	44 0f b6 3c 06       	movzbl (%rsi,%rax,1),%r15d
   29eb1:	49 83 f9 07          	cmp    $0x7,%r9
   29eb5:	b9 07 00 00 00       	mov    $0x7,%ecx
   29eba:	49 0f 42 c9          	cmovb  %r9,%rcx
   29ebe:	41 d2 ef             	shr    %cl,%r15b
   29ec1:	49 83 f9 07          	cmp    $0x7,%r9
   29ec5:	0f 85 ab 00 00 00    	jne    29f76 <<verified_tiff::deflate::Cursor>::step+0x976>
   29ecb:	48 ff c0             	inc    %rax
   29ece:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   29ed5:	45 31 c9             	xor    %r9d,%r9d
   29ed8:	e9 9c 00 00 00       	jmp    29f79 <<verified_tiff::deflate::Cursor>::step+0x979>
   29edd:	49 89 ff             	mov    %rdi,%r15
   29ee0:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   29ee5:	31 f6                	xor    %esi,%esi
   29ee7:	e8 04 cf 00 00       	call   36df0 <verified_tiff::deflate_header::fixed>
   29eec:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   29ef1:	74 78                	je     29f6b <<verified_tiff::deflate::Cursor>::step+0x96b>
   29ef3:	48 8d 74 24 20       	lea    0x20(%rsp),%rsi
   29ef8:	48 8d 9c 24 e8 08 00 	lea    0x8e8(%rsp),%rbx
   29eff:	00 
   29f00:	4c 8b 25 69 f5 05 00 	mov    0x5f569(%rip),%r12        # 89470 <memcpy@GLIBC_2.14>
   29f07:	ba 60 04 00 00       	mov    $0x460,%edx
   29f0c:	48 89 df             	mov    %rbx,%rdi
   29f0f:	41 ff d4             	call   *%r12
   29f12:	49 8d bf 00 80 00 00 	lea    0x8000(%r15),%rdi
   29f19:	ba 60 04 00 00       	mov    $0x460,%edx
   29f1e:	48 89 de             	mov    %rbx,%rsi
   29f21:	41 ff d4             	call   *%r12
   29f24:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   29f29:	be 01 00 00 00       	mov    $0x1,%esi
   29f2e:	e8 bd ce 00 00       	call   36df0 <verified_tiff::deflate_header::fixed>
   29f33:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   29f38:	74 31                	je     29f6b <<verified_tiff::deflate::Cursor>::step+0x96b>
   29f3a:	48 8d 74 24 20       	lea    0x20(%rsp),%rsi
   29f3f:	48 8d 9c 24 e8 08 00 	lea    0x8e8(%rsp),%rbx
   29f46:	00 
   29f47:	ba 60 04 00 00       	mov    $0x460,%edx
   29f4c:	48 89 df             	mov    %rbx,%rdi
   29f4f:	e9 ce 00 00 00       	jmp    2a022 <<verified_tiff::deflate::Cursor>::step+0xa22>
   29f54:	49 89 ff             	mov    %rdi,%r15
   29f57:	48 8d 7c 24 18       	lea    0x18(%rsp),%rdi
   29f5c:	48 89 d9             	mov    %rbx,%rcx
   29f5f:	e8 4c d1 00 00       	call   370b0 <verified_tiff::deflate_header::dynamic>
   29f64:	80 7c 24 18 01       	cmpb   $0x1,0x18(%rsp)
   29f69:	75 58                	jne    29fc3 <<verified_tiff::deflate::Cursor>::step+0x9c3>
   29f6b:	44 0f b6 64 24 19    	movzbl 0x19(%rsp),%r12d
   29f71:	e9 d5 01 00 00       	jmp    2a14b <<verified_tiff::deflate::Cursor>::step+0xb4b>
   29f76:	49 ff c1             	inc    %r9
   29f79:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   29f80:	4c 39 d8             	cmp    %r11,%rax
   29f83:	0f 84 bf 01 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   29f89:	48 39 d0             	cmp    %rdx,%rax
   29f8c:	0f 83 6a 02 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   29f92:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   29f97:	49 83 f9 07          	cmp    $0x7,%r9
   29f9b:	b9 07 00 00 00       	mov    $0x7,%ecx
   29fa0:	49 0f 42 c9          	cmovb  %r9,%rcx
   29fa4:	41 d2 ec             	shr    %cl,%r12b
   29fa7:	49 83 f9 07          	cmp    $0x7,%r9
   29fab:	0f 85 a7 00 00 00    	jne    2a058 <<verified_tiff::deflate::Cursor>::step+0xa58>
   29fb1:	48 ff c0             	inc    %rax
   29fb4:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   29fbb:	45 31 c9             	xor    %r9d,%r9d
   29fbe:	e9 98 00 00 00       	jmp    2a05b <<verified_tiff::deflate::Cursor>::step+0xa5b>
   29fc3:	48 8d 74 24 20       	lea    0x20(%rsp),%rsi
   29fc8:	48 8d 9c 24 e8 08 00 	lea    0x8e8(%rsp),%rbx
   29fcf:	00 
   29fd0:	4c 8b 25 99 f4 05 00 	mov    0x5f499(%rip),%r12        # 89470 <memcpy@GLIBC_2.14>
   29fd7:	ba c0 08 00 00       	mov    $0x8c0,%edx
   29fdc:	48 89 df             	mov    %rbx,%rdi
   29fdf:	41 ff d4             	call   *%r12
   29fe2:	48 8d ac 24 a8 11 00 	lea    0x11a8(%rsp),%rbp
   29fe9:	00 
   29fea:	ba 60 04 00 00       	mov    $0x460,%edx
   29fef:	48 89 ef             	mov    %rbp,%rdi
   29ff2:	48 89 de             	mov    %rbx,%rsi
   29ff5:	41 ff d4             	call   *%r12
   29ff8:	48 8d b4 24 48 0d 00 	lea    0xd48(%rsp),%rsi
   29fff:	00 
   2a000:	48 8d 9c 24 08 16 00 	lea    0x1608(%rsp),%rbx
   2a007:	00 
   2a008:	ba 60 04 00 00       	mov    $0x460,%edx
   2a00d:	48 89 df             	mov    %rbx,%rdi
   2a010:	41 ff d4             	call   *%r12
   2a013:	49 8d bf 00 80 00 00 	lea    0x8000(%r15),%rdi
   2a01a:	ba 60 04 00 00       	mov    $0x460,%edx
   2a01f:	48 89 ee             	mov    %rbp,%rsi
   2a022:	41 ff d4             	call   *%r12
   2a025:	49 8d bf 60 84 00 00 	lea    0x8460(%r15),%rdi
   2a02c:	ba 60 04 00 00       	mov    $0x460,%edx
   2a031:	48 89 de             	mov    %rbx,%rsi
   2a034:	41 ff d4             	call   *%r12
   2a037:	b1 02                	mov    $0x2,%cl
   2a039:	41 88 8f 08 89 00 00 	mov    %cl,0x8908(%r15)
   2a040:	41 0f b6 cd          	movzbl %r13b,%ecx
   2a044:	41 0f b6 d6          	movzbl %r14b,%edx
   2a048:	0f a3 ca             	bt     %ecx,%edx
   2a04b:	41 0f 92 87 09 89 00 	setb   0x8909(%r15)
   2a052:	00 
   2a053:	e9 a0 fa ff ff       	jmp    29af8 <<verified_tiff::deflate::Cursor>::step+0x4f8>
   2a058:	49 ff c1             	inc    %r9
   2a05b:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2a062:	4c 39 d8             	cmp    %r11,%rax
   2a065:	0f 84 dd 00 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a06b:	48 39 d0             	cmp    %rdx,%rax
   2a06e:	0f 83 88 01 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2a074:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2a079:	49 83 f9 07          	cmp    $0x7,%r9
   2a07d:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a082:	49 0f 42 c9          	cmovb  %r9,%rcx
   2a086:	41 d2 ed             	shr    %cl,%r13b
   2a089:	49 83 f9 07          	cmp    $0x7,%r9
   2a08d:	75 0f                	jne    2a09e <<verified_tiff::deflate::Cursor>::step+0xa9e>
   2a08f:	48 ff c0             	inc    %rax
   2a092:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2a099:	45 31 c9             	xor    %r9d,%r9d
   2a09c:	eb 03                	jmp    2a0a1 <<verified_tiff::deflate::Cursor>::step+0xaa1>
   2a09e:	49 ff c1             	inc    %r9
   2a0a1:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2a0a8:	4c 39 d8             	cmp    %r11,%rax
   2a0ab:	0f 84 97 00 00 00    	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a0b1:	44 88 2c 24          	mov    %r13b,(%rsp)
   2a0b5:	48 39 d0             	cmp    %rdx,%rax
   2a0b8:	0f 83 3e 01 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2a0be:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2a0c3:	49 83 f9 07          	cmp    $0x7,%r9
   2a0c7:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a0cc:	49 0f 42 c9          	cmovb  %r9,%rcx
   2a0d0:	41 d2 ed             	shr    %cl,%r13b
   2a0d3:	44 88 6c 24 08       	mov    %r13b,0x8(%rsp)
   2a0d8:	49 83 f9 07          	cmp    $0x7,%r9
   2a0dc:	75 0f                	jne    2a0ed <<verified_tiff::deflate::Cursor>::step+0xaed>
   2a0de:	48 ff c0             	inc    %rax
   2a0e1:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2a0e8:	45 31 c9             	xor    %r9d,%r9d
   2a0eb:	eb 03                	jmp    2a0f0 <<verified_tiff::deflate::Cursor>::step+0xaf0>
   2a0ed:	49 ff c1             	inc    %r9
   2a0f0:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2a0f7:	4c 39 d8             	cmp    %r11,%rax
   2a0fa:	74 4c                	je     2a148 <<verified_tiff::deflate::Cursor>::step+0xb48>
   2a0fc:	48 39 d0             	cmp    %rdx,%rax
   2a0ff:	0f 83 f7 00 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2a105:	44 0f b6 2c 06       	movzbl (%rsi,%rax,1),%r13d
   2a10a:	49 83 f9 07          	cmp    $0x7,%r9
   2a10e:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a113:	49 0f 42 c9          	cmovb  %r9,%rcx
   2a117:	41 d2 ed             	shr    %cl,%r13b
   2a11a:	44 88 6c 24 10       	mov    %r13b,0x10(%rsp)
   2a11f:	49 83 f9 07          	cmp    $0x7,%r9
   2a123:	75 0f                	jne    2a134 <<verified_tiff::deflate::Cursor>::step+0xb34>
   2a125:	48 ff c0             	inc    %rax
   2a128:	48 89 87 c0 88 00 00 	mov    %rax,0x88c0(%rdi)
   2a12f:	45 31 c9             	xor    %r9d,%r9d
   2a132:	eb 03                	jmp    2a137 <<verified_tiff::deflate::Cursor>::step+0xb37>
   2a134:	49 ff c1             	inc    %r9
   2a137:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2a13e:	4c 39 d8             	cmp    %r11,%rax
   2a141:	44 0f b6 1c 24       	movzbl (%rsp),%r11d
   2a146:	75 21                	jne    2a169 <<verified_tiff::deflate::Cursor>::step+0xb69>
   2a148:	45 31 e4             	xor    %r12d,%r12d
   2a14b:	41 0f b6 cc          	movzbl %r12b,%ecx
   2a14f:	c1 e1 08             	shl    $0x8,%ecx
   2a152:	0f b7 c5             	movzwl %bp,%eax
   2a155:	09 c8                	or     %ecx,%eax
   2a157:	48 81 c4 68 1a 00 00 	add    $0x1a68,%rsp
   2a15e:	5b                   	pop    %rbx
   2a15f:	41 5c                	pop    %r12
   2a161:	41 5d                	pop    %r13
   2a163:	41 5e                	pop    %r14
   2a165:	41 5f                	pop    %r15
   2a167:	5d                   	pop    %rbp
   2a168:	c3                   	ret
   2a169:	48 39 d0             	cmp    %rdx,%rax
   2a16c:	0f 83 8a 00 00 00    	jae    2a1fc <<verified_tiff::deflate::Cursor>::step+0xbfc>
   2a172:	45 00 f6             	add    %r14b,%r14b
   2a175:	41 80 e6 02          	and    $0x2,%r14b
   2a179:	44 08 f3             	or     %r14b,%bl
   2a17c:	41 c0 e7 02          	shl    $0x2,%r15b
   2a180:	41 80 e7 04          	and    $0x4,%r15b
   2a184:	41 08 df             	or     %bl,%r15b
   2a187:	41 c0 e4 03          	shl    $0x3,%r12b
   2a18b:	41 80 e4 08          	and    $0x8,%r12b
   2a18f:	45 08 fc             	or     %r15b,%r12b
   2a192:	41 c0 e3 04          	shl    $0x4,%r11b
   2a196:	41 80 e3 10          	and    $0x10,%r11b
   2a19a:	45 08 e3             	or     %r12b,%r11b
   2a19d:	0f b6 4c 24 08       	movzbl 0x8(%rsp),%ecx
   2a1a2:	c0 e1 05             	shl    $0x5,%cl
   2a1a5:	80 e1 20             	and    $0x20,%cl
   2a1a8:	44 08 d9             	or     %r11b,%cl
   2a1ab:	0f b6 54 24 10       	movzbl 0x10(%rsp),%edx
   2a1b0:	c0 e2 06             	shl    $0x6,%dl
   2a1b3:	80 e2 40             	and    $0x40,%dl
   2a1b6:	08 ca                	or     %cl,%dl
   2a1b8:	44 0f b6 24 06       	movzbl (%rsi,%rax,1),%r12d
   2a1bd:	49 83 f9 07          	cmp    $0x7,%r9
   2a1c1:	b9 07 00 00 00       	mov    $0x7,%ecx
   2a1c6:	49 0f 42 c9          	cmovb  %r9,%rcx
   2a1ca:	41 d2 ec             	shr    %cl,%r12b
   2a1cd:	41 c0 e4 07          	shl    $0x7,%r12b
   2a1d1:	41 08 d4             	or     %dl,%r12b
   2a1d4:	49 ff c1             	inc    %r9
   2a1d7:	4c 89 8f c8 88 00 00 	mov    %r9,0x88c8(%rdi)
   2a1de:	4c 89 d1             	mov    %r10,%rcx
   2a1e1:	e9 9d f9 ff ff       	jmp    29b83 <<verified_tiff::deflate::Cursor>::step+0x583>
   2a1e6:	48 8d 0d 63 bc 05 00 	lea    0x5bc63(%rip),%rcx        # 85e50 <anon.7d3d7a5a00fe104142489088ec6a6e3f.2.llvm.691752062989596110>
   2a1ed:	48 89 c7             	mov    %rax,%rdi
   2a1f0:	48 89 d6             	mov    %rdx,%rsi
   2a1f3:	48 89 ca             	mov    %rcx,%rdx
   2a1f6:	ff 15 74 f3 05 00    	call   *0x5f374(%rip)        # 89570 <_DYNAMIC+0x350>
   2a1fc:	48 89 d6             	mov    %rdx,%rsi
   2a1ff:	48 8d 15 4a bc 05 00 	lea    0x5bc4a(%rip),%rdx        # 85e50 <anon.7d3d7a5a00fe104142489088ec6a6e3f.2.llvm.691752062989596110>
   2a206:	48 89 c7             	mov    %rax,%rdi
   2a209:	ff 15 61 f3 05 00    	call   *0x5f361(%rip)        # 89570 <_DYNAMIC+0x350>
   2a20f:	48 8d 05 3a bc 05 00 	lea    0x5bc3a(%rip),%rax        # 85e50 <anon.7d3d7a5a00fe104142489088ec6a6e3f.2.llvm.691752062989596110>
   2a216:	48 89 cf             	mov    %rcx,%rdi
   2a219:	48 89 d6             	mov    %rdx,%rsi
   2a21c:	48 89 c2             	mov    %rax,%rdx
   2a21f:	ff 15 4b f3 05 00    	call   *0x5f34b(%rip)        # 89570 <_DYNAMIC+0x350>
   2a225:	48 8d 15 d4 bd 05 00 	lea    0x5bdd4(%rip),%rdx        # 86000 <anon.7d3d7a5a00fe104142489088ec6a6e3f.26.llvm.691752062989596110>
   2a22c:	be 20 01 00 00       	mov    $0x120,%esi
   2a231:	4c 89 c7             	mov    %r8,%rdi
   2a234:	ff 15 36 f3 05 00    	call   *0x5f336(%rip)        # 89570 <_DYNAMIC+0x350>
   2a23a:	48 8d 0d 8f ba 05 00 	lea    0x5ba8f(%rip),%rcx        # 85cd0 <anon.35f1884359c0d9048fdad1d4cf8af759.18.llvm.17727380151100635305>
   2a241:	48 89 c7             	mov    %rax,%rdi
   2a244:	48 89 d6             	mov    %rdx,%rsi
   2a247:	48 89 ca             	mov    %rcx,%rdx
   2a24a:	ff 15 20 f3 05 00    	call   *0x5f320(%rip)        # 89570 <_DYNAMIC+0x350>
   2a250:	48 8d 05 91 ba 05 00 	lea    0x5ba91(%rip),%rax        # 85ce8 <anon.35f1884359c0d9048fdad1d4cf8af759.19.llvm.17727380151100635305>
   2a257:	48 89 cf             	mov    %rcx,%rdi
   2a25a:	48 89 d6             	mov    %rdx,%rsi
   2a25d:	48 89 c2             	mov    %rax,%rdx
   2a260:	ff 15 0a f3 05 00    	call   *0x5f30a(%rip)        # 89570 <_DYNAMIC+0x350>
   2a266:	48 8d 0d cb bb 05 00 	lea    0x5bbcb(%rip),%rcx        # 85e38 <anon.35f1884359c0d9048fdad1d4cf8af759.24.llvm.17727380151100635305+0xd8>
   2a26d:	48 89 c7             	mov    %rax,%rdi
   2a270:	48 89 d6             	mov    %rdx,%rsi
   2a273:	48 89 ca             	mov    %rcx,%rdx
   2a276:	ff 15 f4 f2 05 00    	call   *0x5f2f4(%rip)        # 89570 <_DYNAMIC+0x350>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
