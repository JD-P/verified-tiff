
target/release/examples/benchmark:     file format elf64-x86-64


Disassembly of section .text:

000000000003de10 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch>:
   3de10:	55                   	push   %rbp
   3de11:	41 57                	push   %r15
   3de13:	41 56                	push   %r14
   3de15:	41 55                	push   %r13
   3de17:	41 54                	push   %r12
   3de19:	53                   	push   %rbx
   3de1a:	48 81 ec 98 01 00 00 	sub    $0x198,%rsp
   3de21:	49 89 cd             	mov    %rcx,%r13
   3de24:	48 8b 8f b8 03 00 00 	mov    0x3b8(%rdi),%rcx
   3de2b:	b0 04                	mov    $0x4,%al
   3de2d:	48 89 8c 24 c0 00 00 	mov    %rcx,0xc0(%rsp)
   3de34:	00 
   3de35:	48 39 ca             	cmp    %rcx,%rdx
   3de38:	0f 82 cd 07 00 00    	jb     3e60b <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7fb>
   3de3e:	4d 89 c6             	mov    %r8,%r14
   3de41:	49 89 fa             	mov    %rdi,%r10
   3de44:	4c 3b 47 48          	cmp    0x48(%rdi),%r8
   3de48:	0f 82 bd 07 00 00    	jb     3e60b <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7fb>
   3de4e:	49 8b 09             	mov    (%r9),%rcx
   3de51:	b0 03                	mov    $0x3,%al
   3de53:	49 2b 4a 50          	sub    0x50(%r10),%rcx
   3de57:	0f 82 ae 07 00 00    	jb     3e60b <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7fb>
   3de5d:	48 89 d5             	mov    %rdx,%rbp
   3de60:	49 89 09             	mov    %rcx,(%r9)
   3de63:	49 8b 92 b0 03 00 00 	mov    0x3b0(%r10),%rdx
   3de6a:	b0 ff                	mov    $0xff,%al
   3de6c:	48 85 d2             	test   %rdx,%rdx
   3de6f:	0f 84 96 07 00 00    	je     3e60b <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7fb>
   3de75:	49 89 f7             	mov    %rsi,%r15
   3de78:	49 8b 8a 78 03 00 00 	mov    0x378(%r10),%rcx
   3de7f:	49 8b 82 80 03 00 00 	mov    0x380(%r10),%rax
   3de86:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   3de8b:	49 8b 9a 88 03 00 00 	mov    0x388(%r10),%rbx
   3de92:	49 8b 82 90 03 00 00 	mov    0x390(%r10),%rax
   3de99:	48 89 84 24 b8 00 00 	mov    %rax,0xb8(%rsp)
   3dea0:	00 
   3dea1:	49 8b 82 98 03 00 00 	mov    0x398(%r10),%rax
   3dea8:	49 8b b2 a0 03 00 00 	mov    0x3a0(%r10),%rsi
   3deaf:	48 89 74 24 18       	mov    %rsi,0x18(%rsp)
   3deb4:	49 8b b2 a8 03 00 00 	mov    0x3a8(%r10),%rsi
   3debb:	49 8b ba c0 03 00 00 	mov    0x3c0(%r10),%rdi
   3dec2:	48 89 bc 24 b0 00 00 	mov    %rdi,0xb0(%rsp)
   3dec9:	00 
   3deca:	49 8b ba c8 03 00 00 	mov    0x3c8(%r10),%rdi
   3ded1:	48 89 bc 24 a8 00 00 	mov    %rdi,0xa8(%rsp)
   3ded8:	00 
   3ded9:	41 0f b7 ba d0 03 00 	movzwl 0x3d0(%r10),%edi
   3dee0:	00 
   3dee1:	66 89 7c 24 16       	mov    %di,0x16(%rsp)
   3dee6:	49 8d ba d2 03 00 00 	lea    0x3d2(%r10),%rdi
   3deed:	48 89 bc 24 a0 00 00 	mov    %rdi,0xa0(%rsp)
   3def4:	00 
   3def5:	49 8b ba d8 03 00 00 	mov    0x3d8(%r10),%rdi
   3defc:	48 89 bc 24 98 00 00 	mov    %rdi,0x98(%rsp)
   3df03:	00 
   3df04:	49 89 c4             	mov    %rax,%r12
   3df07:	4c 0f af e3          	imul   %rbx,%r12
   3df0b:	48 89 44 24 50       	mov    %rax,0x50(%rsp)
   3df10:	48 89 4c 24 30       	mov    %rcx,0x30(%rsp)
   3df15:	48 31 c8             	xor    %rcx,%rax
   3df18:	48 89 f1             	mov    %rsi,%rcx
   3df1b:	48 83 f1 01          	xor    $0x1,%rcx
   3df1f:	48 09 c1             	or     %rax,%rcx
   3df22:	49 8b 82 e0 03 00 00 	mov    0x3e0(%r10),%rax
   3df29:	48 89 84 24 90 00 00 	mov    %rax,0x90(%rsp)
   3df30:	00 
   3df31:	41 0f b6 82 f0 03 00 	movzbl 0x3f0(%r10),%eax
   3df38:	00 
   3df39:	88 44 24 11          	mov    %al,0x11(%rsp)
   3df3d:	41 0f b6 82 f1 03 00 	movzbl 0x3f1(%r10),%eax
   3df44:	00 
   3df45:	88 44 24 10          	mov    %al,0x10(%rsp)
   3df49:	0f 94 44 24 12       	sete   0x12(%rsp)
   3df4e:	41 0f b7 82 e8 03 00 	movzwl 0x3e8(%r10),%eax
   3df55:	00 
   3df56:	66 89 44 24 14       	mov    %ax,0x14(%rsp)
   3df5b:	49 8d 82 ea 03 00 00 	lea    0x3ea(%r10),%rax
   3df62:	48 89 84 24 88 00 00 	mov    %rax,0x88(%rsp)
   3df69:	00 
   3df6a:	49 8d 82 f2 03 00 00 	lea    0x3f2(%r10),%rax
   3df71:	48 89 84 24 80 00 00 	mov    %rax,0x80(%rsp)
   3df78:	00 
   3df79:	49 8b 42 30          	mov    0x30(%r10),%rax
   3df7d:	48 89 44 24 78       	mov    %rax,0x78(%rsp)
   3df82:	49 8b 42 38          	mov    0x38(%r10),%rax
   3df86:	48 89 44 24 70       	mov    %rax,0x70(%rsp)
   3df8b:	48 c7 44 24 40 00 00 	movq   $0x0,0x40(%rsp)
   3df92:	00 00 
   3df94:	45 31 c0             	xor    %r8d,%r8d
   3df97:	48 89 6c 24 58       	mov    %rbp,0x58(%rsp)
   3df9c:	4c 89 b4 24 50 01 00 	mov    %r14,0x150(%rsp)
   3dfa3:	00 
   3dfa4:	4c 89 54 24 68       	mov    %r10,0x68(%rsp)
   3dfa9:	48 89 54 24 60       	mov    %rdx,0x60(%rsp)
   3dfae:	48 89 5c 24 48       	mov    %rbx,0x48(%rsp)
   3dfb3:	48 89 74 24 28       	mov    %rsi,0x28(%rsp)
   3dfb8:	4c 89 bc 24 58 01 00 	mov    %r15,0x158(%rsp)
   3dfbf:	00 
   3dfc0:	eb 62                	jmp    3e024 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x214>
   3dfc2:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   3dfc9:	1f 84 00 00 00 00 00 
   3dfd0:	48 39 4c 24 20       	cmp    %rcx,0x20(%rsp)
   3dfd5:	0f 96 c2             	setbe  %dl
   3dfd8:	08 d0                	or     %dl,%al
   3dfda:	0f 84 ec 04 00 00    	je     3e4cc <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x6bc>
   3dfe0:	4c 89 f5             	mov    %r14,%rbp
   3dfe3:	48 8b 4c 24 40       	mov    0x40(%rsp),%rcx
   3dfe8:	48 ff c1             	inc    %rcx
   3dfeb:	48 8b 74 24 28       	mov    0x28(%rsp),%rsi
   3dff0:	48 39 f1             	cmp    %rsi,%rcx
   3dff3:	b8 00 00 00 00       	mov    $0x0,%eax
   3dff8:	48 0f 44 c8          	cmove  %rax,%rcx
   3dffc:	48 89 4c 24 40       	mov    %rcx,0x40(%rsp)
   3e001:	4c 8b 44 24 38       	mov    0x38(%rsp),%r8
   3e006:	49 ff c0             	inc    %r8
   3e009:	48 8b 54 24 60       	mov    0x60(%rsp),%rdx
   3e00e:	49 39 d0             	cmp    %rdx,%r8
   3e011:	4c 8b b4 24 50 01 00 	mov    0x150(%rsp),%r14
   3e018:	00 
   3e019:	4c 8b 54 24 68       	mov    0x68(%rsp),%r10
   3e01e:	0f 84 e5 05 00 00    	je     3e609 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7f9>
   3e024:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   3e029:	48 89 84 24 d0 00 00 	mov    %rax,0xd0(%rsp)
   3e030:	00 
   3e031:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   3e036:	48 89 84 24 d8 00 00 	mov    %rax,0xd8(%rsp)
   3e03d:	00 
   3e03e:	48 89 9c 24 e0 00 00 	mov    %rbx,0xe0(%rsp)
   3e045:	00 
   3e046:	48 8b 84 24 b8 00 00 	mov    0xb8(%rsp),%rax
   3e04d:	00 
   3e04e:	48 89 84 24 e8 00 00 	mov    %rax,0xe8(%rsp)
   3e055:	00 
   3e056:	48 8b 44 24 50       	mov    0x50(%rsp),%rax
   3e05b:	48 89 84 24 f0 00 00 	mov    %rax,0xf0(%rsp)
   3e062:	00 
   3e063:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   3e068:	48 89 84 24 f8 00 00 	mov    %rax,0xf8(%rsp)
   3e06f:	00 
   3e070:	48 89 b4 24 00 01 00 	mov    %rsi,0x100(%rsp)
   3e077:	00 
   3e078:	48 89 94 24 08 01 00 	mov    %rdx,0x108(%rsp)
   3e07f:	00 
   3e080:	48 8b 84 24 c0 00 00 	mov    0xc0(%rsp),%rax
   3e087:	00 
   3e088:	48 89 84 24 10 01 00 	mov    %rax,0x110(%rsp)
   3e08f:	00 
   3e090:	48 8b 84 24 b0 00 00 	mov    0xb0(%rsp),%rax
   3e097:	00 
   3e098:	48 89 84 24 18 01 00 	mov    %rax,0x118(%rsp)
   3e09f:	00 
   3e0a0:	48 8b 84 24 a8 00 00 	mov    0xa8(%rsp),%rax
   3e0a7:	00 
   3e0a8:	48 89 84 24 20 01 00 	mov    %rax,0x120(%rsp)
   3e0af:	00 
   3e0b0:	0f b7 44 24 16       	movzwl 0x16(%rsp),%eax
   3e0b5:	66 89 84 24 28 01 00 	mov    %ax,0x128(%rsp)
   3e0bc:	00 
   3e0bd:	48 8b 8c 24 a0 00 00 	mov    0xa0(%rsp),%rcx
   3e0c4:	00 
   3e0c5:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   3e0c9:	48 8d 94 24 2a 01 00 	lea    0x12a(%rsp),%rdx
   3e0d0:	00 
   3e0d1:	66 89 42 04          	mov    %ax,0x4(%rdx)
   3e0d5:	8b 01                	mov    (%rcx),%eax
   3e0d7:	89 02                	mov    %eax,(%rdx)
   3e0d9:	48 8b 84 24 98 00 00 	mov    0x98(%rsp),%rax
   3e0e0:	00 
   3e0e1:	48 89 84 24 30 01 00 	mov    %rax,0x130(%rsp)
   3e0e8:	00 
   3e0e9:	48 8b 84 24 90 00 00 	mov    0x90(%rsp),%rax
   3e0f0:	00 
   3e0f1:	48 89 84 24 38 01 00 	mov    %rax,0x138(%rsp)
   3e0f8:	00 
   3e0f9:	0f b7 44 24 14       	movzwl 0x14(%rsp),%eax
   3e0fe:	66 89 84 24 40 01 00 	mov    %ax,0x140(%rsp)
   3e105:	00 
   3e106:	48 8b 8c 24 88 00 00 	mov    0x88(%rsp),%rcx
   3e10d:	00 
   3e10e:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   3e112:	66 89 42 1c          	mov    %ax,0x1c(%rdx)
   3e116:	8b 01                	mov    (%rcx),%eax
   3e118:	89 42 18             	mov    %eax,0x18(%rdx)
   3e11b:	0f b6 44 24 11       	movzbl 0x11(%rsp),%eax
   3e120:	88 84 24 48 01 00 00 	mov    %al,0x148(%rsp)
   3e127:	0f b6 44 24 10       	movzbl 0x10(%rsp),%eax
   3e12c:	88 84 24 49 01 00 00 	mov    %al,0x149(%rsp)
   3e133:	48 8b 8c 24 80 00 00 	mov    0x80(%rsp),%rcx
   3e13a:	00 
   3e13b:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   3e13f:	66 89 42 24          	mov    %ax,0x24(%rdx)
   3e143:	8b 01                	mov    (%rcx),%eax
   3e145:	89 42 20             	mov    %eax,0x20(%rdx)
   3e148:	48 8d bc 24 60 01 00 	lea    0x160(%rsp),%rdi
   3e14f:	00 
   3e150:	48 8b 74 24 78       	mov    0x78(%rsp),%rsi
   3e155:	48 8b 54 24 70       	mov    0x70(%rsp),%rdx
   3e15a:	48 8d 8c 24 d0 00 00 	lea    0xd0(%rsp),%rcx
   3e161:	00 
   3e162:	4c 89 44 24 38       	mov    %r8,0x38(%rsp)
   3e167:	4d 89 d1             	mov    %r10,%r9
   3e16a:	e8 51 3e ff ff       	call   31fc0 <verified_tiff::logluv_segment::read>
   3e16f:	80 bc 24 60 01 00 00 	cmpb   $0x0,0x160(%rsp)
   3e176:	00 
   3e177:	0f 85 82 04 00 00    	jne    3e5ff <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ef>
   3e17d:	48 8b 9c 24 80 01 00 	mov    0x180(%rsp),%rbx
   3e184:	00 
   3e185:	4c 39 f3             	cmp    %r14,%rbx
   3e188:	0f 87 8f 04 00 00    	ja     3e61d <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x80d>
   3e18e:	4d 89 e9             	mov    %r13,%r9
   3e191:	49 89 ef             	mov    %rbp,%r15
   3e194:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   3e199:	48 89 84 24 d0 00 00 	mov    %rax,0xd0(%rsp)
   3e1a0:	00 
   3e1a1:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   3e1a6:	48 89 84 24 d8 00 00 	mov    %rax,0xd8(%rsp)
   3e1ad:	00 
   3e1ae:	48 8b 44 24 48       	mov    0x48(%rsp),%rax
   3e1b3:	48 89 84 24 e0 00 00 	mov    %rax,0xe0(%rsp)
   3e1ba:	00 
   3e1bb:	48 8b 84 24 b8 00 00 	mov    0xb8(%rsp),%rax
   3e1c2:	00 
   3e1c3:	48 89 84 24 e8 00 00 	mov    %rax,0xe8(%rsp)
   3e1ca:	00 
   3e1cb:	48 8b 44 24 50       	mov    0x50(%rsp),%rax
   3e1d0:	48 89 c5             	mov    %rax,%rbp
   3e1d3:	48 89 84 24 f0 00 00 	mov    %rax,0xf0(%rsp)
   3e1da:	00 
   3e1db:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   3e1e0:	48 89 84 24 f8 00 00 	mov    %rax,0xf8(%rsp)
   3e1e7:	00 
   3e1e8:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
   3e1ed:	48 89 84 24 00 01 00 	mov    %rax,0x100(%rsp)
   3e1f4:	00 
   3e1f5:	48 8b 44 24 60       	mov    0x60(%rsp),%rax
   3e1fa:	48 89 84 24 08 01 00 	mov    %rax,0x108(%rsp)
   3e201:	00 
   3e202:	48 8b 84 24 c0 00 00 	mov    0xc0(%rsp),%rax
   3e209:	00 
   3e20a:	48 89 84 24 10 01 00 	mov    %rax,0x110(%rsp)
   3e211:	00 
   3e212:	48 8b 84 24 b0 00 00 	mov    0xb0(%rsp),%rax
   3e219:	00 
   3e21a:	48 89 84 24 18 01 00 	mov    %rax,0x118(%rsp)
   3e221:	00 
   3e222:	48 8b 84 24 a8 00 00 	mov    0xa8(%rsp),%rax
   3e229:	00 
   3e22a:	48 89 84 24 20 01 00 	mov    %rax,0x120(%rsp)
   3e231:	00 
   3e232:	0f b7 44 24 16       	movzwl 0x16(%rsp),%eax
   3e237:	66 89 84 24 28 01 00 	mov    %ax,0x128(%rsp)
   3e23e:	00 
   3e23f:	48 8b 8c 24 a0 00 00 	mov    0xa0(%rsp),%rcx
   3e246:	00 
   3e247:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   3e24b:	48 8d 94 24 2a 01 00 	lea    0x12a(%rsp),%rdx
   3e252:	00 
   3e253:	66 89 42 04          	mov    %ax,0x4(%rdx)
   3e257:	8b 01                	mov    (%rcx),%eax
   3e259:	89 02                	mov    %eax,(%rdx)
   3e25b:	48 8b 84 24 98 00 00 	mov    0x98(%rsp),%rax
   3e262:	00 
   3e263:	48 89 84 24 30 01 00 	mov    %rax,0x130(%rsp)
   3e26a:	00 
   3e26b:	48 8b 84 24 90 00 00 	mov    0x90(%rsp),%rax
   3e272:	00 
   3e273:	48 89 84 24 38 01 00 	mov    %rax,0x138(%rsp)
   3e27a:	00 
   3e27b:	0f b7 44 24 14       	movzwl 0x14(%rsp),%eax
   3e280:	66 89 84 24 40 01 00 	mov    %ax,0x140(%rsp)
   3e287:	00 
   3e288:	48 8b 8c 24 88 00 00 	mov    0x88(%rsp),%rcx
   3e28f:	00 
   3e290:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   3e294:	66 89 42 1c          	mov    %ax,0x1c(%rdx)
   3e298:	8b 01                	mov    (%rcx),%eax
   3e29a:	89 42 18             	mov    %eax,0x18(%rdx)
   3e29d:	0f b6 44 24 11       	movzbl 0x11(%rsp),%eax
   3e2a2:	88 84 24 48 01 00 00 	mov    %al,0x148(%rsp)
   3e2a9:	0f b6 44 24 10       	movzbl 0x10(%rsp),%eax
   3e2ae:	88 84 24 49 01 00 00 	mov    %al,0x149(%rsp)
   3e2b5:	48 8b 8c 24 80 00 00 	mov    0x80(%rsp),%rcx
   3e2bc:	00 
   3e2bd:	0f b7 41 04          	movzwl 0x4(%rcx),%eax
   3e2c1:	66 89 42 24          	mov    %ax,0x24(%rdx)
   3e2c5:	8b 01                	mov    (%rcx),%eax
   3e2c7:	89 42 20             	mov    %eax,0x20(%rdx)
   3e2ca:	4c 8b b4 24 88 01 00 	mov    0x188(%rsp),%r14
   3e2d1:	00 
   3e2d2:	48 89 1c 24          	mov    %rbx,(%rsp)
   3e2d6:	48 8b 7c 24 78       	mov    0x78(%rsp),%rdi
   3e2db:	48 8b 74 24 70       	mov    0x70(%rsp),%rsi
   3e2e0:	48 8d 94 24 d0 00 00 	lea    0xd0(%rsp),%rdx
   3e2e7:	00 
   3e2e8:	48 8b 4c 24 38       	mov    0x38(%rsp),%rcx
   3e2ed:	4c 8b 44 24 68       	mov    0x68(%rsp),%r8
   3e2f2:	e8 f9 3d ff ff       	call   320f0 <verified_tiff::logluv_segment::decode>
   3e2f7:	3c ff                	cmp    $0xff,%al
   3e2f9:	0f 85 0c 03 00 00    	jne    3e60b <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7fb>
   3e2ff:	49 39 de             	cmp    %rbx,%r14
   3e302:	0f 87 2e 03 00 00    	ja     3e636 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x826>
   3e308:	4c 89 f7             	mov    %r14,%rdi
   3e30b:	4d 85 f6             	test   %r14,%r14
   3e30e:	48 8b 5c 24 48       	mov    0x48(%rsp),%rbx
   3e313:	4d 89 fe             	mov    %r15,%r14
   3e316:	4c 8b bc 24 58 01 00 	mov    0x158(%rsp),%r15
   3e31d:	00 
   3e31e:	0f 84 bc fc ff ff    	je     3dfe0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1d0>
   3e324:	48 8b 44 24 38       	mov    0x38(%rsp),%rax
   3e329:	48 f7 64 24 18       	mulq   0x18(%rsp)
   3e32e:	48 89 c1             	mov    %rax,%rcx
   3e331:	0f 90 c0             	seto   %al
   3e334:	4d 85 e4             	test   %r12,%r12
   3e337:	0f 84 31 03 00 00    	je     3e66e <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x85e>
   3e33d:	80 7c 24 12 00       	cmpb   $0x0,0x12(%rsp)
   3e342:	48 89 bc 24 c8 00 00 	mov    %rdi,0xc8(%rsp)
   3e349:	00 
   3e34a:	0f 85 80 fc ff ff    	jne    3dfd0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1c0>
   3e350:	48 83 7c 24 28 00    	cmpq   $0x0,0x28(%rsp)
   3e356:	0f 84 06 02 00 00    	je     3e562 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x752>
   3e35c:	45 31 db             	xor    %r11d,%r11d
   3e35f:	31 c9                	xor    %ecx,%ecx
   3e361:	eb 2d                	jmp    3e390 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x580>
   3e363:	0f b6 44 24 13       	movzbl 0x13(%rsp),%eax
   3e368:	41 88 04 3f          	mov    %al,(%r15,%rdi,1)
   3e36c:	0f 1f 40 00          	nopl   0x0(%rax)
   3e370:	49 ff c3             	inc    %r11
   3e373:	4d 39 e3             	cmp    %r12,%r11
   3e376:	b8 00 00 00 00       	mov    $0x0,%eax
   3e37b:	4c 0f 44 d8          	cmove  %rax,%r11
   3e37f:	48 ff c1             	inc    %rcx
   3e382:	48 39 8c 24 c8 00 00 	cmp    %rcx,0xc8(%rsp)
   3e389:	00 
   3e38a:	0f 84 50 fc ff ff    	je     3dfe0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1d0>
   3e390:	41 0f b6 44 0d 00    	movzbl 0x0(%r13,%rcx,1),%eax
   3e396:	88 44 24 13          	mov    %al,0x13(%rsp)
   3e39a:	48 89 c8             	mov    %rcx,%rax
   3e39d:	4c 09 e0             	or     %r12,%rax
   3e3a0:	48 c1 e8 20          	shr    $0x20,%rax
   3e3a4:	74 1a                	je     3e3c0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5b0>
   3e3a6:	48 89 c8             	mov    %rcx,%rax
   3e3a9:	31 d2                	xor    %edx,%edx
   3e3ab:	49 f7 f4             	div    %r12
   3e3ae:	48 89 c6             	mov    %rax,%rsi
   3e3b1:	4c 89 da             	mov    %r11,%rdx
   3e3b4:	48 3b 74 24 18       	cmp    0x18(%rsp),%rsi
   3e3b9:	73 b5                	jae    3e370 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x560>
   3e3bb:	eb 13                	jmp    3e3d0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5c0>
   3e3bd:	0f 1f 00             	nopl   (%rax)
   3e3c0:	89 c8                	mov    %ecx,%eax
   3e3c2:	31 d2                	xor    %edx,%edx
   3e3c4:	41 f7 f4             	div    %r12d
   3e3c7:	89 c6                	mov    %eax,%esi
   3e3c9:	48 3b 74 24 18       	cmp    0x18(%rsp),%rsi
   3e3ce:	73 a0                	jae    3e370 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x560>
   3e3d0:	48 89 d0             	mov    %rdx,%rax
   3e3d3:	48 09 d8             	or     %rbx,%rax
   3e3d6:	48 c1 e8 20          	shr    $0x20,%rax
   3e3da:	74 15                	je     3e3f1 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5e1>
   3e3dc:	48 89 d0             	mov    %rdx,%rax
   3e3df:	31 d2                	xor    %edx,%edx
   3e3e1:	48 f7 f3             	div    %rbx
   3e3e4:	48 89 d7             	mov    %rdx,%rdi
   3e3e7:	49 89 c0             	mov    %rax,%r8
   3e3ea:	49 39 e8             	cmp    %rbp,%r8
   3e3ed:	73 81                	jae    3e370 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x560>
   3e3ef:	eb 14                	jmp    3e405 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x5f5>
   3e3f1:	89 d0                	mov    %edx,%eax
   3e3f3:	31 d2                	xor    %edx,%edx
   3e3f5:	f7 f3                	div    %ebx
   3e3f7:	89 d7                	mov    %edx,%edi
   3e3f9:	41 89 c0             	mov    %eax,%r8d
   3e3fc:	49 39 e8             	cmp    %rbp,%r8
   3e3ff:	0f 83 6b ff ff ff    	jae    3e370 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x560>
   3e405:	48 8b 44 24 38       	mov    0x38(%rsp),%rax
   3e40a:	48 89 c2             	mov    %rax,%rdx
   3e40d:	4c 8b 4c 24 28       	mov    0x28(%rsp),%r9
   3e412:	4c 09 ca             	or     %r9,%rdx
   3e415:	48 c1 ea 20          	shr    $0x20,%rdx
   3e419:	74 1a                	je     3e435 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x625>
   3e41b:	31 d2                	xor    %edx,%edx
   3e41d:	49 f7 f1             	div    %r9
   3e420:	49 89 c2             	mov    %rax,%r10
   3e423:	48 8b 54 24 40       	mov    0x40(%rsp),%rdx
   3e428:	48 89 d0             	mov    %rdx,%rax
   3e42b:	48 f7 e5             	mul    %rbp
   3e42e:	71 19                	jno    3e449 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x639>
   3e430:	e9 3b ff ff ff       	jmp    3e370 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x560>
   3e435:	31 d2                	xor    %edx,%edx
   3e437:	41 f7 f1             	div    %r9d
   3e43a:	41 89 c2             	mov    %eax,%r10d
   3e43d:	48 89 d0             	mov    %rdx,%rax
   3e440:	48 f7 e5             	mul    %rbp
   3e443:	0f 80 27 ff ff ff    	jo     3e370 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x560>
   3e449:	49 89 c1             	mov    %rax,%r9
   3e44c:	4c 89 eb             	mov    %r13,%rbx
   3e44f:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   3e454:	4c 29 c8             	sub    %r9,%rax
   3e457:	41 0f 96 c6          	setbe  %r14b
   3e45b:	49 39 c0             	cmp    %rax,%r8
   3e45e:	40 0f 93 c5          	setae  %bpl
   3e462:	4c 89 d0             	mov    %r10,%rax
   3e465:	48 f7 64 24 18       	mulq   0x18(%rsp)
   3e46a:	41 0f 90 c5          	seto   %r13b
   3e46e:	4c 8b 54 24 20       	mov    0x20(%rsp),%r10
   3e473:	49 29 c2             	sub    %rax,%r10
   3e476:	0f 96 c2             	setbe  %dl
   3e479:	4c 39 d6             	cmp    %r10,%rsi
   3e47c:	41 0f 93 c2          	setae  %r10b
   3e480:	45 08 f5             	or     %r14b,%r13b
   3e483:	75 42                	jne    3e4c7 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x6b7>
   3e485:	44 08 d2             	or     %r10b,%dl
   3e488:	40 08 ea             	or     %bpl,%dl
   3e48b:	4c 8b 74 24 58       	mov    0x58(%rsp),%r14
   3e490:	49 89 dd             	mov    %rbx,%r13
   3e493:	48 8b 5c 24 48       	mov    0x48(%rsp),%rbx
   3e498:	48 8b 6c 24 50       	mov    0x50(%rsp),%rbp
   3e49d:	0f 85 cd fe ff ff    	jne    3e370 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x560>
   3e4a3:	4d 01 c1             	add    %r8,%r9
   3e4a6:	48 01 f0             	add    %rsi,%rax
   3e4a9:	48 0f af 44 24 30    	imul   0x30(%rsp),%rax
   3e4af:	4c 01 c8             	add    %r9,%rax
   3e4b2:	48 0f af c3          	imul   %rbx,%rax
   3e4b6:	48 01 c7             	add    %rax,%rdi
   3e4b9:	4c 39 f7             	cmp    %r14,%rdi
   3e4bc:	0f 82 a1 fe ff ff    	jb     3e363 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x553>
   3e4c2:	e9 88 01 00 00       	jmp    3e64f <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x83f>
   3e4c7:	40 b5 01             	mov    $0x1,%bpl
   3e4ca:	eb b9                	jmp    3e485 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x675>
   3e4cc:	4c 8b 44 24 20       	mov    0x20(%rsp),%r8
   3e4d1:	49 29 c8             	sub    %rcx,%r8
   3e4d4:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   3e4d9:	4c 39 c0             	cmp    %r8,%rax
   3e4dc:	4c 0f 42 c0          	cmovb  %rax,%r8
   3e4e0:	45 31 c9             	xor    %r9d,%r9d
   3e4e3:	31 f6                	xor    %esi,%esi
   3e4e5:	eb 31                	jmp    3e518 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x708>
   3e4e7:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   3e4ee:	00 00 
   3e4f0:	45 88 14 3f          	mov    %r10b,(%r15,%rdi,1)
   3e4f4:	4c 89 c8             	mov    %r9,%rax
   3e4f7:	48 ff c0             	inc    %rax
   3e4fa:	4c 39 e0             	cmp    %r12,%rax
   3e4fd:	41 b9 00 00 00 00    	mov    $0x0,%r9d
   3e503:	4c 0f 45 c8          	cmovne %rax,%r9
   3e507:	48 ff c6             	inc    %rsi
   3e50a:	48 39 b4 24 c8 00 00 	cmp    %rsi,0xc8(%rsp)
   3e511:	00 
   3e512:	0f 84 c8 fa ff ff    	je     3dfe0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1d0>
   3e518:	45 0f b6 54 35 00    	movzbl 0x0(%r13,%rsi,1),%r10d
   3e51e:	48 89 f0             	mov    %rsi,%rax
   3e521:	4c 09 e0             	or     %r12,%rax
   3e524:	48 c1 e8 20          	shr    $0x20,%rax
   3e528:	74 16                	je     3e540 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x730>
   3e52a:	48 89 f0             	mov    %rsi,%rax
   3e52d:	31 d2                	xor    %edx,%edx
   3e52f:	49 f7 f4             	div    %r12
   3e532:	4c 89 cf             	mov    %r9,%rdi
   3e535:	4c 39 c0             	cmp    %r8,%rax
   3e538:	73 ba                	jae    3e4f4 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x6e4>
   3e53a:	eb 12                	jmp    3e54e <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x73e>
   3e53c:	0f 1f 40 00          	nopl   0x0(%rax)
   3e540:	89 f0                	mov    %esi,%eax
   3e542:	31 d2                	xor    %edx,%edx
   3e544:	41 f7 f4             	div    %r12d
   3e547:	89 d7                	mov    %edx,%edi
   3e549:	4c 39 c0             	cmp    %r8,%rax
   3e54c:	73 a6                	jae    3e4f4 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x6e4>
   3e54e:	48 01 c8             	add    %rcx,%rax
   3e551:	49 0f af c4          	imul   %r12,%rax
   3e555:	48 01 c7             	add    %rax,%rdi
   3e558:	4c 39 f7             	cmp    %r14,%rdi
   3e55b:	72 93                	jb     3e4f0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x6e0>
   3e55d:	e9 ed 00 00 00       	jmp    3e64f <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x83f>
   3e562:	31 f6                	xor    %esi,%esi
   3e564:	31 c9                	xor    %ecx,%ecx
   3e566:	eb 28                	jmp    3e590 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x780>
   3e568:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   3e56f:	00 
   3e570:	48 89 d0             	mov    %rdx,%rax
   3e573:	31 d2                	xor    %edx,%edx
   3e575:	48 f7 f3             	div    %rbx
   3e578:	48 39 e8             	cmp    %rbp,%rax
   3e57b:	73 63                	jae    3e5e0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7d0>
   3e57d:	e9 df 00 00 00       	jmp    3e661 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x851>
   3e582:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   3e589:	1f 84 00 00 00 00 00 
   3e590:	48 89 c8             	mov    %rcx,%rax
   3e593:	4c 09 e0             	or     %r12,%rax
   3e596:	48 c1 e8 20          	shr    $0x20,%rax
   3e59a:	74 14                	je     3e5b0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7a0>
   3e59c:	48 89 c8             	mov    %rcx,%rax
   3e59f:	31 d2                	xor    %edx,%edx
   3e5a1:	49 f7 f4             	div    %r12
   3e5a4:	48 89 f2             	mov    %rsi,%rdx
   3e5a7:	48 3b 44 24 18       	cmp    0x18(%rsp),%rax
   3e5ac:	72 10                	jb     3e5be <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7ae>
   3e5ae:	eb 30                	jmp    3e5e0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7d0>
   3e5b0:	89 c8                	mov    %ecx,%eax
   3e5b2:	31 d2                	xor    %edx,%edx
   3e5b4:	41 f7 f4             	div    %r12d
   3e5b7:	48 3b 44 24 18       	cmp    0x18(%rsp),%rax
   3e5bc:	73 22                	jae    3e5e0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7d0>
   3e5be:	48 89 d0             	mov    %rdx,%rax
   3e5c1:	48 09 d8             	or     %rbx,%rax
   3e5c4:	48 c1 e8 20          	shr    $0x20,%rax
   3e5c8:	75 a6                	jne    3e570 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x760>
   3e5ca:	89 d0                	mov    %edx,%eax
   3e5cc:	31 d2                	xor    %edx,%edx
   3e5ce:	f7 f3                	div    %ebx
   3e5d0:	48 39 e8             	cmp    %rbp,%rax
   3e5d3:	0f 82 88 00 00 00    	jb     3e661 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x851>
   3e5d9:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   3e5e0:	48 89 f0             	mov    %rsi,%rax
   3e5e3:	48 ff c0             	inc    %rax
   3e5e6:	4c 39 e0             	cmp    %r12,%rax
   3e5e9:	be 00 00 00 00       	mov    $0x0,%esi
   3e5ee:	48 0f 45 f0          	cmovne %rax,%rsi
   3e5f2:	48 ff c1             	inc    %rcx
   3e5f5:	48 39 cf             	cmp    %rcx,%rdi
   3e5f8:	75 96                	jne    3e590 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x780>
   3e5fa:	e9 e1 f9 ff ff       	jmp    3dfe0 <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x1d0>
   3e5ff:	0f b6 84 24 61 01 00 	movzbl 0x161(%rsp),%eax
   3e606:	00 
   3e607:	eb 02                	jmp    3e60b <<verified_tiff::logluv_tiff::LogluvImage>::decode_with_scratch+0x7fb>
   3e609:	b0 ff                	mov    $0xff,%al
   3e60b:	48 81 c4 98 01 00 00 	add    $0x198,%rsp
   3e612:	5b                   	pop    %rbx
   3e613:	41 5c                	pop    %r12
   3e615:	41 5d                	pop    %r13
   3e617:	41 5e                	pop    %r14
   3e619:	41 5f                	pop    %r15
   3e61b:	5d                   	pop    %rbp
   3e61c:	c3                   	ret
   3e61d:	48 8d 3d 8c c5 fc ff 	lea    -0x33a74(%rip),%rdi        # abb0 <anon.97691c58cb47cc7f1bcd0c1575eb4e44.30.llvm.7495961015551628049+0x88>
   3e624:	48 8d 15 2d 99 04 00 	lea    0x4992d(%rip),%rdx        # 87f58 <anon.97691c58cb47cc7f1bcd0c1575eb4e44.35.llvm.7495961015551628049+0x90>
   3e62b:	be 13 00 00 00       	mov    $0x13,%esi
   3e630:	ff 15 e2 bf 04 00    	call   *0x4bfe2(%rip)        # 8a618 <_DYNAMIC+0x268>
   3e636:	48 8d 3d 73 c5 fc ff 	lea    -0x33a8d(%rip),%rdi        # abb0 <anon.97691c58cb47cc7f1bcd0c1575eb4e44.30.llvm.7495961015551628049+0x88>
   3e63d:	48 8d 15 2c 99 04 00 	lea    0x4992c(%rip),%rdx        # 87f70 <anon.97691c58cb47cc7f1bcd0c1575eb4e44.35.llvm.7495961015551628049+0xa8>
   3e644:	be 13 00 00 00       	mov    $0x13,%esi
   3e649:	ff 15 c9 bf 04 00    	call   *0x4bfc9(%rip)        # 8a618 <_DYNAMIC+0x268>
   3e64f:	48 8d 15 8a 98 04 00 	lea    0x4988a(%rip),%rdx        # 87ee0 <anon.97691c58cb47cc7f1bcd0c1575eb4e44.35.llvm.7495961015551628049+0x18>
   3e656:	48 8b 74 24 58       	mov    0x58(%rsp),%rsi
   3e65b:	ff 15 9f c0 04 00    	call   *0x4c09f(%rip)        # 8a700 <_DYNAMIC+0x350>
   3e661:	48 8d 3d c0 98 04 00 	lea    0x498c0(%rip),%rdi        # 87f28 <anon.97691c58cb47cc7f1bcd0c1575eb4e44.35.llvm.7495961015551628049+0x60>
   3e668:	ff 15 9a c0 04 00    	call   *0x4c09a(%rip)        # 8a708 <_DYNAMIC+0x358>
   3e66e:	48 8d 3d 83 98 04 00 	lea    0x49883(%rip),%rdi        # 87ef8 <anon.97691c58cb47cc7f1bcd0c1575eb4e44.35.llvm.7495961015551628049+0x30>
   3e675:	ff 15 95 c0 04 00    	call   *0x4c095(%rip)        # 8a710 <_DYNAMIC+0x360>

Disassembly of section .init:

Disassembly of section .fini:

Disassembly of section .plt:
