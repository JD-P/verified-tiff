
target/baseline-v18/verified-tiff/target/release/examples/benchmark:     file format elf64-x86-64


Disassembly of section .text:

000000000002fb50 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch>:
   2fb50:	55                   	push   %rbp
   2fb51:	41 57                	push   %r15
   2fb53:	41 56                	push   %r14
   2fb55:	41 55                	push   %r13
   2fb57:	41 54                	push   %r12
   2fb59:	53                   	push   %rbx
   2fb5a:	48 81 ec 78 01 00 00 	sub    $0x178,%rsp
   2fb61:	48 89 4c 24 40       	mov    %rcx,0x40(%rsp)
   2fb66:	48 89 b4 24 a8 00 00 	mov    %rsi,0xa8(%rsp)
   2fb6d:	00 
   2fb6e:	48 8b 8f b8 03 00 00 	mov    0x3b8(%rdi),%rcx
   2fb75:	b0 04                	mov    $0x4,%al
   2fb77:	48 89 54 24 38       	mov    %rdx,0x38(%rsp)
   2fb7c:	48 89 8c 24 b0 00 00 	mov    %rcx,0xb0(%rsp)
   2fb83:	00 
   2fb84:	48 39 ca             	cmp    %rcx,%rdx
   2fb87:	0f 82 6e 07 00 00    	jb     302fb <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ab>
   2fb8d:	4d 89 c7             	mov    %r8,%r15
   2fb90:	49 89 fa             	mov    %rdi,%r10
   2fb93:	4c 3b 47 48          	cmp    0x48(%rdi),%r8
   2fb97:	0f 82 5e 07 00 00    	jb     302fb <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ab>
   2fb9d:	49 8b 09             	mov    (%r9),%rcx
   2fba0:	b0 03                	mov    $0x3,%al
   2fba2:	49 2b 4a 50          	sub    0x50(%r10),%rcx
   2fba6:	0f 82 4f 07 00 00    	jb     302fb <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ab>
   2fbac:	49 89 09             	mov    %rcx,(%r9)
   2fbaf:	49 8b 92 b0 03 00 00 	mov    0x3b0(%r10),%rdx
   2fbb6:	b0 ff                	mov    $0xff,%al
   2fbb8:	48 85 d2             	test   %rdx,%rdx
   2fbbb:	0f 84 3a 07 00 00    	je     302fb <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ab>
   2fbc1:	49 8b 8a 78 03 00 00 	mov    0x378(%r10),%rcx
   2fbc8:	49 8b 82 80 03 00 00 	mov    0x380(%r10),%rax
   2fbcf:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   2fbd4:	49 8b b2 88 03 00 00 	mov    0x388(%r10),%rsi
   2fbdb:	49 8b 82 90 03 00 00 	mov    0x390(%r10),%rax
   2fbe2:	48 89 84 24 a0 00 00 	mov    %rax,0xa0(%rsp)
   2fbe9:	00 
   2fbea:	49 8b 82 98 03 00 00 	mov    0x398(%r10),%rax
   2fbf1:	49 8b 9a a0 03 00 00 	mov    0x3a0(%r10),%rbx
   2fbf8:	49 8b ba a8 03 00 00 	mov    0x3a8(%r10),%rdi
   2fbff:	4d 8b 82 c0 03 00 00 	mov    0x3c0(%r10),%r8
   2fc06:	4c 89 84 24 98 00 00 	mov    %r8,0x98(%rsp)
   2fc0d:	00 
   2fc0e:	4d 8b 82 c8 03 00 00 	mov    0x3c8(%r10),%r8
   2fc15:	4c 89 84 24 90 00 00 	mov    %r8,0x90(%rsp)
   2fc1c:	00 
   2fc1d:	45 0f b7 82 d0 03 00 	movzwl 0x3d0(%r10),%r8d
   2fc24:	00 
   2fc25:	66 44 89 44 24 0e    	mov    %r8w,0xe(%rsp)
   2fc2b:	4d 8d 82 d2 03 00 00 	lea    0x3d2(%r10),%r8
   2fc32:	4c 89 84 24 88 00 00 	mov    %r8,0x88(%rsp)
   2fc39:	00 
   2fc3a:	4d 8b 82 d8 03 00 00 	mov    0x3d8(%r10),%r8
   2fc41:	4c 89 84 24 80 00 00 	mov    %r8,0x80(%rsp)
   2fc48:	00 
   2fc49:	48 89 c5             	mov    %rax,%rbp
   2fc4c:	48 0f af ee          	imul   %rsi,%rbp
   2fc50:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   2fc55:	48 89 4c 24 30       	mov    %rcx,0x30(%rsp)
   2fc5a:	48 31 c8             	xor    %rcx,%rax
   2fc5d:	48 89 f9             	mov    %rdi,%rcx
   2fc60:	48 83 f1 01          	xor    $0x1,%rcx
   2fc64:	48 09 c1             	or     %rax,%rcx
   2fc67:	49 8b 82 e0 03 00 00 	mov    0x3e0(%r10),%rax
   2fc6e:	48 89 44 24 78       	mov    %rax,0x78(%rsp)
   2fc73:	41 0f b6 82 f0 03 00 	movzbl 0x3f0(%r10),%eax
   2fc7a:	00 
   2fc7b:	88 44 24 0a          	mov    %al,0xa(%rsp)
   2fc7f:	41 0f b6 82 f1 03 00 	movzbl 0x3f1(%r10),%eax
   2fc86:	00 
   2fc87:	88 44 24 09          	mov    %al,0x9(%rsp)
   2fc8b:	0f 94 44 24 0b       	sete   0xb(%rsp)
   2fc90:	41 0f b7 82 e8 03 00 	movzwl 0x3e8(%r10),%eax
   2fc97:	00 
   2fc98:	66 89 44 24 0c       	mov    %ax,0xc(%rsp)
   2fc9d:	49 8d 82 ea 03 00 00 	lea    0x3ea(%r10),%rax
   2fca4:	48 89 44 24 70       	mov    %rax,0x70(%rsp)
   2fca9:	49 8d 82 f2 03 00 00 	lea    0x3f2(%r10),%rax
   2fcb0:	48 89 44 24 68       	mov    %rax,0x68(%rsp)
   2fcb5:	49 8b 42 30          	mov    0x30(%r10),%rax
   2fcb9:	48 89 44 24 60       	mov    %rax,0x60(%rsp)
   2fcbe:	49 8b 42 38          	mov    0x38(%r10),%rax
   2fcc2:	48 89 44 24 58       	mov    %rax,0x58(%rsp)
   2fcc7:	45 31 ed             	xor    %r13d,%r13d
   2fcca:	45 31 e4             	xor    %r12d,%r12d
   2fccd:	4c 89 bc 24 38 01 00 	mov    %r15,0x138(%rsp)
   2fcd4:	00 
   2fcd5:	4c 89 54 24 50       	mov    %r10,0x50(%rsp)
   2fcda:	48 89 54 24 48       	mov    %rdx,0x48(%rsp)
   2fcdf:	48 89 74 24 10       	mov    %rsi,0x10(%rsp)
   2fce4:	48 89 7c 24 28       	mov    %rdi,0x28(%rsp)
   2fce9:	eb 3c                	jmp    2fd27 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1d7>
   2fceb:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   2fcf0:	49 ff c5             	inc    %r13
   2fcf3:	48 8b 7c 24 28       	mov    0x28(%rsp),%rdi
   2fcf8:	49 39 fd             	cmp    %rdi,%r13
   2fcfb:	b8 00 00 00 00       	mov    $0x0,%eax
   2fd00:	4c 0f 44 e8          	cmove  %rax,%r13
   2fd04:	49 ff c4             	inc    %r12
   2fd07:	48 8b 54 24 48       	mov    0x48(%rsp),%rdx
   2fd0c:	49 39 d4             	cmp    %rdx,%r12
   2fd0f:	4c 8b bc 24 38 01 00 	mov    0x138(%rsp),%r15
   2fd16:	00 
   2fd17:	4c 8b 54 24 50       	mov    0x50(%rsp),%r10
   2fd1c:	48 8b 74 24 10       	mov    0x10(%rsp),%rsi
   2fd21:	0f 84 d2 05 00 00    	je     302f9 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7a9>
   2fd27:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   2fd2c:	48 89 84 24 b8 00 00 	mov    %rax,0xb8(%rsp)
   2fd33:	00 
   2fd34:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   2fd39:	48 89 84 24 c0 00 00 	mov    %rax,0xc0(%rsp)
   2fd40:	00 
   2fd41:	48 89 b4 24 c8 00 00 	mov    %rsi,0xc8(%rsp)
   2fd48:	00 
   2fd49:	48 8b 84 24 a0 00 00 	mov    0xa0(%rsp),%rax
   2fd50:	00 
   2fd51:	48 89 84 24 d0 00 00 	mov    %rax,0xd0(%rsp)
   2fd58:	00 
   2fd59:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   2fd5e:	48 89 84 24 d8 00 00 	mov    %rax,0xd8(%rsp)
   2fd65:	00 
   2fd66:	48 89 9c 24 e0 00 00 	mov    %rbx,0xe0(%rsp)
   2fd6d:	00 
   2fd6e:	48 89 bc 24 e8 00 00 	mov    %rdi,0xe8(%rsp)
   2fd75:	00 
   2fd76:	48 89 94 24 f0 00 00 	mov    %rdx,0xf0(%rsp)
   2fd7d:	00 
   2fd7e:	48 8b 84 24 b0 00 00 	mov    0xb0(%rsp),%rax
   2fd85:	00 
   2fd86:	48 89 84 24 f8 00 00 	mov    %rax,0xf8(%rsp)
   2fd8d:	00 
   2fd8e:	48 8b 84 24 98 00 00 	mov    0x98(%rsp),%rax
   2fd95:	00 
   2fd96:	48 89 84 24 00 01 00 	mov    %rax,0x100(%rsp)
   2fd9d:	00 
   2fd9e:	48 8b 84 24 90 00 00 	mov    0x90(%rsp),%rax
   2fda5:	00 
   2fda6:	48 89 84 24 08 01 00 	mov    %rax,0x108(%rsp)
   2fdad:	00 
   2fdae:	0f b7 44 24 0e       	movzwl 0xe(%rsp),%eax
   2fdb3:	66 89 84 24 10 01 00 	mov    %ax,0x110(%rsp)
   2fdba:	00 
   2fdbb:	48 8b 8c 24 88 00 00 	mov    0x88(%rsp),%rcx
   2fdc2:	00 
   2fdc3:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   2fdc7:	48 8d 94 24 12 01 00 	lea    0x112(%rsp),%rdx
   2fdce:	00 
   2fdcf:	66 89 42 04          	mov    %ax,0x4(%rdx)
   2fdd3:	8b 01                	mov    (%rcx),%eax
   2fdd5:	89 02                	mov    %eax,(%rdx)
   2fdd7:	48 8b 84 24 80 00 00 	mov    0x80(%rsp),%rax
   2fdde:	00 
   2fddf:	48 89 84 24 18 01 00 	mov    %rax,0x118(%rsp)
   2fde6:	00 
   2fde7:	48 8b 44 24 78       	mov    0x78(%rsp),%rax
   2fdec:	48 89 84 24 20 01 00 	mov    %rax,0x120(%rsp)
   2fdf3:	00 
   2fdf4:	0f b7 44 24 0c       	movzwl 0xc(%rsp),%eax
   2fdf9:	66 89 84 24 28 01 00 	mov    %ax,0x128(%rsp)
   2fe00:	00 
   2fe01:	48 8b 4c 24 70       	mov    0x70(%rsp),%rcx
   2fe06:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   2fe0a:	66 89 42 1c          	mov    %ax,0x1c(%rdx)
   2fe0e:	8b 01                	mov    (%rcx),%eax
   2fe10:	89 42 18             	mov    %eax,0x18(%rdx)
   2fe13:	0f b6 44 24 0a       	movzbl 0xa(%rsp),%eax
   2fe18:	88 84 24 30 01 00 00 	mov    %al,0x130(%rsp)
   2fe1f:	0f b6 44 24 09       	movzbl 0x9(%rsp),%eax
   2fe24:	88 84 24 31 01 00 00 	mov    %al,0x131(%rsp)
   2fe2b:	48 8b 4c 24 68       	mov    0x68(%rsp),%rcx
   2fe30:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   2fe34:	66 89 42 24          	mov    %ax,0x24(%rdx)
   2fe38:	8b 01                	mov    (%rcx),%eax
   2fe3a:	89 42 20             	mov    %eax,0x20(%rdx)
   2fe3d:	48 8d bc 24 40 01 00 	lea    0x140(%rsp),%rdi
   2fe44:	00 
   2fe45:	48 8b 74 24 60       	mov    0x60(%rsp),%rsi
   2fe4a:	48 8b 54 24 58       	mov    0x58(%rsp),%rdx
   2fe4f:	48 8d 8c 24 b8 00 00 	lea    0xb8(%rsp),%rcx
   2fe56:	00 
   2fe57:	4d 89 e0             	mov    %r12,%r8
   2fe5a:	4d 89 d1             	mov    %r10,%r9
   2fe5d:	e8 ce ae ff ff       	call   2ad30 <verified_tiff::logluv_segment::read>
   2fe62:	80 bc 24 40 01 00 00 	cmpb   $0x0,0x140(%rsp)
   2fe69:	00 
   2fe6a:	0f 85 7f 04 00 00    	jne    302ef <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x79f>
   2fe70:	4c 8b b4 24 60 01 00 	mov    0x160(%rsp),%r14
   2fe77:	00 
   2fe78:	4d 39 fe             	cmp    %r15,%r14
   2fe7b:	0f 87 8c 04 00 00    	ja     3030d <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7bd>
   2fe81:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   2fe86:	48 89 84 24 b8 00 00 	mov    %rax,0xb8(%rsp)
   2fe8d:	00 
   2fe8e:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   2fe93:	48 89 84 24 c0 00 00 	mov    %rax,0xc0(%rsp)
   2fe9a:	00 
   2fe9b:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   2fea0:	48 89 84 24 c8 00 00 	mov    %rax,0xc8(%rsp)
   2fea7:	00 
   2fea8:	48 8b 84 24 a0 00 00 	mov    0xa0(%rsp),%rax
   2feaf:	00 
   2feb0:	48 89 84 24 d0 00 00 	mov    %rax,0xd0(%rsp)
   2feb7:	00 
   2feb8:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   2febd:	48 89 84 24 d8 00 00 	mov    %rax,0xd8(%rsp)
   2fec4:	00 
   2fec5:	48 89 9c 24 e0 00 00 	mov    %rbx,0xe0(%rsp)
   2fecc:	00 
   2fecd:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
   2fed2:	48 89 84 24 e8 00 00 	mov    %rax,0xe8(%rsp)
   2fed9:	00 
   2feda:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
   2fedf:	48 89 84 24 f0 00 00 	mov    %rax,0xf0(%rsp)
   2fee6:	00 
   2fee7:	48 8b 84 24 b0 00 00 	mov    0xb0(%rsp),%rax
   2feee:	00 
   2feef:	48 89 84 24 f8 00 00 	mov    %rax,0xf8(%rsp)
   2fef6:	00 
   2fef7:	48 8b 84 24 98 00 00 	mov    0x98(%rsp),%rax
   2fefe:	00 
   2feff:	48 89 84 24 00 01 00 	mov    %rax,0x100(%rsp)
   2ff06:	00 
   2ff07:	48 8b 84 24 90 00 00 	mov    0x90(%rsp),%rax
   2ff0e:	00 
   2ff0f:	48 89 84 24 08 01 00 	mov    %rax,0x108(%rsp)
   2ff16:	00 
   2ff17:	0f b7 44 24 0e       	movzwl 0xe(%rsp),%eax
   2ff1c:	66 89 84 24 10 01 00 	mov    %ax,0x110(%rsp)
   2ff23:	00 
   2ff24:	48 8b 8c 24 88 00 00 	mov    0x88(%rsp),%rcx
   2ff2b:	00 
   2ff2c:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   2ff30:	48 8d 94 24 12 01 00 	lea    0x112(%rsp),%rdx
   2ff37:	00 
   2ff38:	66 89 42 04          	mov    %ax,0x4(%rdx)
   2ff3c:	8b 01                	mov    (%rcx),%eax
   2ff3e:	89 02                	mov    %eax,(%rdx)
   2ff40:	48 8b 84 24 80 00 00 	mov    0x80(%rsp),%rax
   2ff47:	00 
   2ff48:	48 89 84 24 18 01 00 	mov    %rax,0x118(%rsp)
   2ff4f:	00 
   2ff50:	48 8b 44 24 78       	mov    0x78(%rsp),%rax
   2ff55:	48 89 84 24 20 01 00 	mov    %rax,0x120(%rsp)
   2ff5c:	00 
   2ff5d:	0f b7 44 24 0c       	movzwl 0xc(%rsp),%eax
   2ff62:	66 89 84 24 28 01 00 	mov    %ax,0x128(%rsp)
   2ff69:	00 
   2ff6a:	48 8b 4c 24 70       	mov    0x70(%rsp),%rcx
   2ff6f:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   2ff73:	66 89 42 1c          	mov    %ax,0x1c(%rdx)
   2ff77:	8b 01                	mov    (%rcx),%eax
   2ff79:	89 42 18             	mov    %eax,0x18(%rdx)
   2ff7c:	0f b6 44 24 0a       	movzbl 0xa(%rsp),%eax
   2ff81:	88 84 24 30 01 00 00 	mov    %al,0x130(%rsp)
   2ff88:	0f b6 44 24 09       	movzbl 0x9(%rsp),%eax
   2ff8d:	88 84 24 31 01 00 00 	mov    %al,0x131(%rsp)
   2ff94:	48 8b 4c 24 68       	mov    0x68(%rsp),%rcx
   2ff99:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   2ff9d:	66 89 42 24          	mov    %ax,0x24(%rdx)
   2ffa1:	8b 01                	mov    (%rcx),%eax
   2ffa3:	89 42 20             	mov    %eax,0x20(%rdx)
   2ffa6:	4c 8b bc 24 68 01 00 	mov    0x168(%rsp),%r15
   2ffad:	00 
   2ffae:	4c 89 34 24          	mov    %r14,(%rsp)
   2ffb2:	48 8b 7c 24 60       	mov    0x60(%rsp),%rdi
   2ffb7:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
   2ffbc:	48 8d 94 24 b8 00 00 	lea    0xb8(%rsp),%rdx
   2ffc3:	00 
   2ffc4:	4c 89 e1             	mov    %r12,%rcx
   2ffc7:	4c 8b 44 24 50       	mov    0x50(%rsp),%r8
   2ffcc:	4c 8b 4c 24 40       	mov    0x40(%rsp),%r9
   2ffd1:	e8 8a ae ff ff       	call   2ae60 <verified_tiff::logluv_segment::decode>
   2ffd6:	3c ff                	cmp    $0xff,%al
   2ffd8:	0f 85 1d 03 00 00    	jne    302fb <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ab>
   2ffde:	4d 39 f7             	cmp    %r14,%r15
   2ffe1:	0f 87 3f 03 00 00    	ja     30326 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7d6>
   2ffe7:	4d 85 ff             	test   %r15,%r15
   2ffea:	0f 84 00 fd ff ff    	je     2fcf0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1a0>
   2fff0:	4c 89 e0             	mov    %r12,%rax
   2fff3:	48 f7 e3             	mul    %rbx
   2fff6:	48 89 c1             	mov    %rax,%rcx
   2fff9:	0f 90 c0             	seto   %al
   2fffc:	4c 8b 44 24 20       	mov    0x20(%rsp),%r8
   30001:	49 29 c8             	sub    %rcx,%r8
   30004:	0f 96 c2             	setbe  %dl
   30007:	48 85 ed             	test   %rbp,%rbp
   3000a:	0f 84 4e 03 00 00    	je     3035e <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x80e>
   30010:	80 7c 24 0b 00       	cmpb   $0x0,0xb(%rsp)
   30015:	0f 84 b5 00 00 00    	je     300d0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x580>
   3001b:	08 d0                	or     %dl,%al
   3001d:	0f 85 cd fc ff ff    	jne    2fcf0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1a0>
   30023:	4c 39 c3             	cmp    %r8,%rbx
   30026:	4c 0f 42 c3          	cmovb  %rbx,%r8
   3002a:	45 31 c9             	xor    %r9d,%r9d
   3002d:	31 f6                	xor    %esi,%esi
   3002f:	eb 3a                	jmp    3006b <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x51b>
   30031:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   30038:	0f 1f 84 00 00 00 00 
   3003f:	00 
   30040:	48 8b 84 24 a8 00 00 	mov    0xa8(%rsp),%rax
   30047:	00 
   30048:	44 88 14 38          	mov    %r10b,(%rax,%rdi,1)
   3004c:	4c 89 c8             	mov    %r9,%rax
   3004f:	48 ff c0             	inc    %rax
   30052:	48 39 e8             	cmp    %rbp,%rax
   30055:	41 b9 00 00 00 00    	mov    $0x0,%r9d
   3005b:	4c 0f 45 c8          	cmovne %rax,%r9
   3005f:	48 ff c6             	inc    %rsi
   30062:	49 39 f7             	cmp    %rsi,%r15
   30065:	0f 84 85 fc ff ff    	je     2fcf0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1a0>
   3006b:	48 8b 44 24 40       	mov    0x40(%rsp),%rax
   30070:	44 0f b6 14 30       	movzbl (%rax,%rsi,1),%r10d
   30075:	48 89 f0             	mov    %rsi,%rax
   30078:	48 09 e8             	or     %rbp,%rax
   3007b:	48 c1 e8 20          	shr    $0x20,%rax
   3007f:	74 1f                	je     300a0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x550>
   30081:	48 89 f0             	mov    %rsi,%rax
   30084:	31 d2                	xor    %edx,%edx
   30086:	48 f7 f5             	div    %rbp
   30089:	4c 89 cf             	mov    %r9,%rdi
   3008c:	4c 39 c0             	cmp    %r8,%rax
   3008f:	73 bb                	jae    3004c <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x4fc>
   30091:	eb 1a                	jmp    300ad <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x55d>
   30093:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   3009a:	84 00 00 00 00 00 
   300a0:	89 f0                	mov    %esi,%eax
   300a2:	31 d2                	xor    %edx,%edx
   300a4:	f7 f5                	div    %ebp
   300a6:	89 d7                	mov    %edx,%edi
   300a8:	4c 39 c0             	cmp    %r8,%rax
   300ab:	73 9f                	jae    3004c <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x4fc>
   300ad:	48 01 c8             	add    %rcx,%rax
   300b0:	48 0f af c5          	imul   %rbp,%rax
   300b4:	48 01 c7             	add    %rax,%rdi
   300b7:	48 3b 7c 24 38       	cmp    0x38(%rsp),%rdi
   300bc:	72 82                	jb     30040 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x4f0>
   300be:	e9 89 02 00 00       	jmp    3034c <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7fc>
   300c3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   300ca:	84 00 00 00 00 00 
   300d0:	48 83 7c 24 28 00    	cmpq   $0x0,0x28(%rsp)
   300d6:	0f 84 6b 01 00 00    	je     30247 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x6f7>
   300dc:	45 31 db             	xor    %r11d,%r11d
   300df:	31 c9                	xor    %ecx,%ecx
   300e1:	eb 28                	jmp    3010b <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5bb>
   300e3:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   300ea:	84 00 00 00 00 00 
   300f0:	49 ff c3             	inc    %r11
   300f3:	49 39 eb             	cmp    %rbp,%r11
   300f6:	b8 00 00 00 00       	mov    $0x0,%eax
   300fb:	4c 0f 44 d8          	cmove  %rax,%r11
   300ff:	48 ff c1             	inc    %rcx
   30102:	49 39 cf             	cmp    %rcx,%r15
   30105:	0f 84 e5 fb ff ff    	je     2fcf0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1a0>
   3010b:	48 8b 44 24 40       	mov    0x40(%rsp),%rax
   30110:	44 0f b6 34 08       	movzbl (%rax,%rcx,1),%r14d
   30115:	48 89 c8             	mov    %rcx,%rax
   30118:	48 09 e8             	or     %rbp,%rax
   3011b:	48 c1 e8 20          	shr    $0x20,%rax
   3011f:	74 1f                	je     30140 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5f0>
   30121:	48 89 c8             	mov    %rcx,%rax
   30124:	31 d2                	xor    %edx,%edx
   30126:	48 f7 f5             	div    %rbp
   30129:	48 89 c6             	mov    %rax,%rsi
   3012c:	4c 89 da             	mov    %r11,%rdx
   3012f:	48 39 de             	cmp    %rbx,%rsi
   30132:	73 bc                	jae    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   30134:	eb 17                	jmp    3014d <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5fd>
   30136:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   3013d:	00 00 00 
   30140:	89 c8                	mov    %ecx,%eax
   30142:	31 d2                	xor    %edx,%edx
   30144:	f7 f5                	div    %ebp
   30146:	89 c6                	mov    %eax,%esi
   30148:	48 39 de             	cmp    %rbx,%rsi
   3014b:	73 a3                	jae    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   3014d:	48 89 d0             	mov    %rdx,%rax
   30150:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   30155:	48 09 f8             	or     %rdi,%rax
   30158:	48 c1 e8 20          	shr    $0x20,%rax
   3015c:	74 1b                	je     30179 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x629>
   3015e:	48 89 d0             	mov    %rdx,%rax
   30161:	31 d2                	xor    %edx,%edx
   30163:	48 f7 f7             	div    %rdi
   30166:	48 89 d7             	mov    %rdx,%rdi
   30169:	49 89 c0             	mov    %rax,%r8
   3016c:	4c 3b 44 24 18       	cmp    0x18(%rsp),%r8
   30171:	0f 83 79 ff ff ff    	jae    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   30177:	eb 16                	jmp    3018f <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x63f>
   30179:	89 d0                	mov    %edx,%eax
   3017b:	31 d2                	xor    %edx,%edx
   3017d:	f7 f7                	div    %edi
   3017f:	89 d7                	mov    %edx,%edi
   30181:	41 89 c0             	mov    %eax,%r8d
   30184:	4c 3b 44 24 18       	cmp    0x18(%rsp),%r8
   30189:	0f 83 61 ff ff ff    	jae    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   3018f:	4c 89 e0             	mov    %r12,%rax
   30192:	4c 8b 4c 24 28       	mov    0x28(%rsp),%r9
   30197:	4c 09 c8             	or     %r9,%rax
   3019a:	48 c1 e8 20          	shr    $0x20,%rax
   3019e:	74 1d                	je     301bd <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x66d>
   301a0:	4c 89 e0             	mov    %r12,%rax
   301a3:	31 d2                	xor    %edx,%edx
   301a5:	49 f7 f1             	div    %r9
   301a8:	49 89 c2             	mov    %rax,%r10
   301ab:	4c 89 ea             	mov    %r13,%rdx
   301ae:	48 89 d0             	mov    %rdx,%rax
   301b1:	48 f7 64 24 18       	mulq   0x18(%rsp)
   301b6:	71 1e                	jno    301d6 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x686>
   301b8:	e9 33 ff ff ff       	jmp    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   301bd:	44 89 e0             	mov    %r12d,%eax
   301c0:	31 d2                	xor    %edx,%edx
   301c2:	41 f7 f1             	div    %r9d
   301c5:	41 89 c2             	mov    %eax,%r10d
   301c8:	48 89 d0             	mov    %rdx,%rax
   301cb:	48 f7 64 24 18       	mulq   0x18(%rsp)
   301d0:	0f 80 1a ff ff ff    	jo     300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   301d6:	49 89 c1             	mov    %rax,%r9
   301d9:	4c 89 d0             	mov    %r10,%rax
   301dc:	48 f7 e3             	mul    %rbx
   301df:	0f 80 0b ff ff ff    	jo     300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   301e5:	48 8b 54 24 30       	mov    0x30(%rsp),%rdx
   301ea:	4c 29 ca             	sub    %r9,%rdx
   301ed:	0f 86 fd fe ff ff    	jbe    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   301f3:	49 39 d0             	cmp    %rdx,%r8
   301f6:	0f 83 f4 fe ff ff    	jae    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   301fc:	48 8b 54 24 20       	mov    0x20(%rsp),%rdx
   30201:	48 29 c2             	sub    %rax,%rdx
   30204:	0f 86 e6 fe ff ff    	jbe    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   3020a:	48 39 d6             	cmp    %rdx,%rsi
   3020d:	0f 83 dd fe ff ff    	jae    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   30213:	4d 01 c1             	add    %r8,%r9
   30216:	48 01 f0             	add    %rsi,%rax
   30219:	48 0f af 44 24 30    	imul   0x30(%rsp),%rax
   3021f:	4c 01 c8             	add    %r9,%rax
   30222:	48 0f af 44 24 10    	imul   0x10(%rsp),%rax
   30228:	48 01 c7             	add    %rax,%rdi
   3022b:	48 3b 7c 24 38       	cmp    0x38(%rsp),%rdi
   30230:	0f 83 16 01 00 00    	jae    3034c <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7fc>
   30236:	48 8b 84 24 a8 00 00 	mov    0xa8(%rsp),%rax
   3023d:	00 
   3023e:	44 88 34 38          	mov    %r14b,(%rax,%rdi,1)
   30242:	e9 a9 fe ff ff       	jmp    300f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5a0>
   30247:	31 f6                	xor    %esi,%esi
   30249:	31 c9                	xor    %ecx,%ecx
   3024b:	eb 23                	jmp    30270 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x720>
   3024d:	0f 1f 00             	nopl   (%rax)
   30250:	48 89 d0             	mov    %rdx,%rax
   30253:	31 d2                	xor    %edx,%edx
   30255:	48 f7 f7             	div    %rdi
   30258:	48 3b 44 24 18       	cmp    0x18(%rsp),%rax
   3025d:	73 71                	jae    302d0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x780>
   3025f:	e9 db 00 00 00       	jmp    3033f <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ef>
   30264:	66 66 66 2e 0f 1f 84 	data16 data16 cs nopw 0x0(%rax,%rax,1)
   3026b:	00 00 00 00 00 
   30270:	48 89 c8             	mov    %rcx,%rax
   30273:	48 09 e8             	or     %rbp,%rax
   30276:	48 c1 e8 20          	shr    $0x20,%rax
   3027a:	74 24                	je     302a0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x750>
   3027c:	48 89 c8             	mov    %rcx,%rax
   3027f:	31 d2                	xor    %edx,%edx
   30281:	48 f7 f5             	div    %rbp
   30284:	48 89 f2             	mov    %rsi,%rdx
   30287:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   3028c:	48 39 d8             	cmp    %rbx,%rax
   3028f:	72 1f                	jb     302b0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x760>
   30291:	eb 3d                	jmp    302d0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x780>
   30293:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   3029a:	84 00 00 00 00 00 
   302a0:	89 c8                	mov    %ecx,%eax
   302a2:	31 d2                	xor    %edx,%edx
   302a4:	f7 f5                	div    %ebp
   302a6:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
   302ab:	48 39 d8             	cmp    %rbx,%rax
   302ae:	73 20                	jae    302d0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x780>
   302b0:	48 89 d0             	mov    %rdx,%rax
   302b3:	48 09 f8             	or     %rdi,%rax
   302b6:	48 c1 e8 20          	shr    $0x20,%rax
   302ba:	75 94                	jne    30250 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x700>
   302bc:	89 d0                	mov    %edx,%eax
   302be:	31 d2                	xor    %edx,%edx
   302c0:	f7 f7                	div    %edi
   302c2:	48 3b 44 24 18       	cmp    0x18(%rsp),%rax
   302c7:	72 76                	jb     3033f <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ef>
   302c9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   302d0:	48 89 f0             	mov    %rsi,%rax
   302d3:	48 ff c0             	inc    %rax
   302d6:	48 39 e8             	cmp    %rbp,%rax
   302d9:	be 00 00 00 00       	mov    $0x0,%esi
   302de:	48 0f 45 f0          	cmovne %rax,%rsi
   302e2:	48 ff c1             	inc    %rcx
   302e5:	49 39 cf             	cmp    %rcx,%r15
   302e8:	75 86                	jne    30270 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x720>
   302ea:	e9 01 fa ff ff       	jmp    2fcf0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1a0>
   302ef:	0f b6 84 24 41 01 00 	movzbl 0x141(%rsp),%eax
   302f6:	00 
   302f7:	eb 02                	jmp    302fb <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ab>
   302f9:	b0 ff                	mov    $0xff,%al
   302fb:	48 81 c4 78 01 00 00 	add    $0x178,%rsp
   30302:	5b                   	pop    %rbx
   30303:	41 5c                	pop    %r12
   30305:	41 5d                	pop    %r13
   30307:	41 5e                	pop    %r14
   30309:	41 5f                	pop    %r15
   3030b:	5d                   	pop    %rbp
   3030c:	c3                   	ret
   3030d:	48 8d 3d 60 a0 fd ff 	lea    -0x25fa0(%rip),%rdi        # a374 <anon.ac36df6000eed6ce22b87f572999359a.13.llvm.17988912158883924058+0x2c>
   30314:	48 8d 15 95 62 05 00 	lea    0x56295(%rip),%rdx        # 865b0 <anon.ac36df6000eed6ce22b87f572999359a.32.llvm.17988912158883924058+0x228>
   3031b:	be 13 00 00 00       	mov    $0x13,%esi
   30320:	ff 15 12 94 05 00    	call   *0x59412(%rip)        # 89738 <_DYNAMIC+0x268>
   30326:	48 8d 3d 47 a0 fd ff 	lea    -0x25fb9(%rip),%rdi        # a374 <anon.ac36df6000eed6ce22b87f572999359a.13.llvm.17988912158883924058+0x2c>
   3032d:	48 8d 15 94 62 05 00 	lea    0x56294(%rip),%rdx        # 865c8 <anon.ac36df6000eed6ce22b87f572999359a.32.llvm.17988912158883924058+0x240>
   30334:	be 13 00 00 00       	mov    $0x13,%esi
   30339:	ff 15 f9 93 05 00    	call   *0x593f9(%rip)        # 89738 <_DYNAMIC+0x268>
   3033f:	48 8d 3d 1a 61 05 00 	lea    0x5611a(%rip),%rdi        # 86460 <anon.ac36df6000eed6ce22b87f572999359a.32.llvm.17988912158883924058+0xd8>
   30346:	ff 15 34 95 05 00    	call   *0x59534(%rip)        # 89880 <_DYNAMIC+0x3b0>
   3034c:	48 8d 15 dd 60 05 00 	lea    0x560dd(%rip),%rdx        # 86430 <anon.ac36df6000eed6ce22b87f572999359a.32.llvm.17988912158883924058+0xa8>
   30353:	48 8b 74 24 38       	mov    0x38(%rsp),%rsi
   30358:	ff 15 fa 94 05 00    	call   *0x594fa(%rip)        # 89858 <_DYNAMIC+0x388>
   3035e:	48 8d 3d e3 60 05 00 	lea    0x560e3(%rip),%rdi        # 86448 <anon.ac36df6000eed6ce22b87f572999359a.32.llvm.17988912158883924058+0xc0>
   30365:	ff 15 1d 95 05 00    	call   *0x5951d(%rip)        # 89888 <_DYNAMIC+0x3b8>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
