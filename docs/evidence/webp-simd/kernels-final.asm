Binary: target/compat/libtiff.so.6
000000000006c9f0 <verified_tiff::native_vp8::render_rgba>:
   6c9f0:	push   %rbp
   6c9f1:	push   %r15
   6c9f3:	push   %r14
   6c9f5:	push   %r13
   6c9f7:	push   %r12
   6c9f9:	push   %rbx
   6c9fa:	sub    $0x198,%rsp
   6ca01:	mov    %rdx,%r10
   6ca04:	mov    %rdi,0x68(%rsp)
   6ca09:	lea    -0x4000(%rdx),%rax
   6ca10:	cmp    $0xffffffffffffc001,%rax
   6ca16:	setb   %al
   6ca19:	lea    -0x4000(%rcx),%rdx
   6ca20:	cmp    $0xffffffffffffc001,%rdx
   6ca27:	setb   %dl
   6ca2a:	or     %al,%dl
   6ca2c:	mov    $0x1,%al
   6ca2e:	je     6ca42 <verified_tiff::native_vp8::render_rgba+0x52>
   6ca30:	add    $0x198,%rsp
   6ca37:	pop    %rbx
   6ca38:	pop    %r12
   6ca3a:	pop    %r13
   6ca3c:	pop    %r14
   6ca3e:	pop    %r15
   6ca40:	pop    %rbp
   6ca41:	ret
   6ca42:	mov    %rcx,%rdx
   6ca45:	imul   %r10,%rdx
   6ca49:	mov    $0x4,%al
   6ca4b:	mov    %rdx,0xc8(%rsp)
   6ca53:	cmp    %rdx,%r9
   6ca56:	jb     6ca30 <verified_tiff::native_vp8::render_rgba+0x40>
   6ca58:	lea    0xf(%r10),%edi
   6ca5c:	and    $0x7ff0,%edi
   6ca62:	lea    0xf(%rcx),%r13d
   6ca66:	and    $0x7ff0,%r13d
   6ca6d:	mov    %r13d,%edx
   6ca70:	shr    $1,%edx
   6ca72:	mov    %rdx,%r11
   6ca75:	add    %r13,%rdx
   6ca78:	mov    %rdi,0xc0(%rsp)
   6ca80:	imul   %rdi,%rdx
   6ca84:	cmp    %rdx,%rsi
   6ca87:	jb     6ca30 <verified_tiff::native_vp8::render_rgba+0x40>
   6ca89:	mov    0x1d0(%rsp),%rdi
   6ca91:	mov    %rsi,0x8(%rsp)
   6ca96:	imul   $0x3e8,0xc8(%rsp),%rsi
   6caa2:	mov    (%rdi),%rdx
   6caa5:	mov    $0x3,%al
   6caa7:	sub    %rsi,%rdx
   6caaa:	mov    0x8(%rsp),%rsi
   6caaf:	jb     6ca30 <verified_tiff::native_vp8::render_rgba+0x40>
   6cab5:	mov    %rdx,(%rdi)
   6cab8:	mov    0xc0(%rsp),%rax
   6cac0:	mov    %eax,%ebx
   6cac2:	shr    $1,%ebx
   6cac4:	imul   %rbx,%r11
   6cac8:	imul   %rax,%r13
   6cacc:	lea    -0x1(%r10),%r15
   6cad0:	shr    $1,%r15
   6cad3:	dec    %rcx
   6cad6:	shr    $1,%rcx
   6cad9:	add    %r13,%r11
   6cadc:	mov    %r11,0x48(%rsp)
   6cae1:	movdqa 0xda817(%rip),%xmm0        # 147300 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x220>
   6cae9:	pxor   %xmm3,%xmm3
   6caed:	movdqa 0xda85b(%rip),%xmm6        # 147350 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x270>
   6caf5:	movdqa 0xda863(%rip),%xmm7        # 147360 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x280>
   6cafd:	movdqa 0xda86a(%rip),%xmm8        # 147370 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x290>
   6cb06:	movdqa 0xda3c1(%rip),%xmm9        # 146ed0 <identity_bits+0x470>
   6cb0f:	movdqa 0xda868(%rip),%xmm10        # 147380 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x2a0>
   6cb18:	movdqa 0xda86f(%rip),%xmm11        # 147390 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x2b0>
   6cb21:	movdqa 0xda876(%rip),%xmm12        # 1473a0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x2c0>
   6cb2a:	movdqa 0xda7bd(%rip),%xmm13        # 1472f0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x210>
   6cb33:	xor    %r12d,%r12d
   6cb36:	mov    %r8,0xb8(%rsp)
   6cb3e:	mov    %rcx,0xb0(%rsp)
   6cb46:	mov    %r10,0xa8(%rsp)
   6cb4e:	mov    %r13,0xa0(%rsp)
   6cb56:	mov    %rbx,0x190(%rsp)
   6cb5e:	mov    %r15,0x28(%rsp)
   6cb63:	mov    %rsi,%r8
   6cb66:	mov    %r9,0x100(%rsp)
   6cb6e:	jmp    6cb9d <verified_tiff::native_vp8::render_rgba+0x1ad>
   6cb70:	or     %edx,%eax
   6cb72:	or     %esi,%eax
   6cb74:	mov    $0x1,%edx
   6cb79:	mov    %r12,%rdi
   6cb7c:	mov    0x8(%rsp),%r8
   6cb81:	mov    %eax,(%rbx,%rdi,4)
   6cb84:	add    %rdx,%r12
   6cb87:	cmp    0xc8(%rsp),%r12
   6cb8f:	mov    0x190(%rsp),%rbx
   6cb97:	jae    6d8b3 <verified_tiff::native_vp8::render_rgba+0xec3>
   6cb9d:	mov    %r12,%rax
   6cba0:	or     %r10,%rax
   6cba3:	shr    $0x20,%rax
   6cba7:	je     6cbc0 <verified_tiff::native_vp8::render_rgba+0x1d0>
   6cba9:	mov    %r12,%rax
   6cbac:	xor    %edx,%edx
   6cbae:	div    %r10
   6cbb1:	mov    %rdx,%r11
   6cbb4:	jmp    6cbcb <verified_tiff::native_vp8::render_rgba+0x1db>
   6cbb6:	cs nopw 0x0(%rax,%rax,1)
   6cbc0:	mov    %r12d,%eax
   6cbc3:	xor    %edx,%edx
   6cbc5:	div    %r10d
   6cbc8:	mov    %edx,%r11d
   6cbcb:	mov    %r10,%rdx
   6cbce:	sub    %r11,%rdx
   6cbd1:	mov    %rax,%rdi
   6cbd4:	imul   0xc0(%rsp),%rdi
   6cbdd:	cmp    $0x3,%rdx
   6cbe1:	jbe    6d5f0 <verified_tiff::native_vp8::render_rgba+0xc00>
   6cbe7:	lea    -0x1(%rax),%rdx
   6cbeb:	shr    $1,%rdx
   6cbee:	imul   %rbx,%rdx
   6cbf2:	mov    %r12,0x188(%rsp)
   6cbfa:	cmp    %r12,%r10
   6cbfd:	mov    $0x0,%esi
   6cc02:	cmovbe %rdx,%rsi
   6cc06:	lea    0x1(%rax),%r12
   6cc0a:	shr    $1,%r12
   6cc0d:	cmp    %rcx,%r12
   6cc10:	cmovae %rcx,%r12
   6cc14:	lea    (%rdi,%r11,1),%r14
   6cc18:	cmp    %r8,%r14
   6cc1b:	jae    6d90e <verified_tiff::native_vp8::render_rgba+0xf1e>
   6cc21:	lea    (%rsi,%r13,1),%r9
   6cc25:	lea    -0x1(%r11),%rcx
   6cc29:	shr    $1,%rcx
   6cc2c:	cmp    $0x1,%r11
   6cc30:	mov    $0x0,%r10d
   6cc36:	cmovae %rcx,%r10
   6cc3a:	lea    0x1(%r11),%rcx
   6cc3e:	shr    $1,%rcx
   6cc41:	cmp    %r15,%rcx
   6cc44:	mov    %rcx,%rdx
   6cc47:	cmovb  %rcx,%r15
   6cc4b:	lea    (%r9,%r10,1),%rcx
   6cc4f:	mov    %rcx,0x98(%rsp)
   6cc57:	cmp    %r8,%rcx
   6cc5a:	jae    6dae5 <verified_tiff::native_vp8::render_rgba+0x10f5>
   6cc60:	mov    %r9,0x10(%rsp)
   6cc65:	lea    (%r9,%r15,1),%rcx
   6cc69:	cmp    %r8,%rcx
   6cc6c:	jae    6d8f9 <verified_tiff::native_vp8::render_rgba+0xf09>
   6cc72:	mov    %rcx,0x180(%rsp)
   6cc7a:	imul   %rbx,%r12
   6cc7e:	lea    (%r12,%r13,1),%rcx
   6cc82:	lea    (%rcx,%r10,1),%rbx
   6cc86:	cmp    %r8,%rbx
   6cc89:	jae    6d8e4 <verified_tiff::native_vp8::render_rgba+0xef4>
   6cc8f:	mov    %r15,%rbp
   6cc92:	mov    %r10,%r15
   6cc95:	lea    (%rcx,%rbp,1),%r10
   6cc99:	cmp    %r8,%r10
   6cc9c:	jae    6d9b5 <verified_tiff::native_vp8::render_rgba+0xfc5>
   6cca2:	mov    %rdx,%r13
   6cca5:	add    0x48(%rsp),%rsi
   6ccaa:	lea    (%rsi,%r15,1),%rdx
   6ccae:	mov    %rdx,0x90(%rsp)
   6ccb6:	cmp    %r8,%rdx
   6ccb9:	jae    6dacb <verified_tiff::native_vp8::render_rgba+0x10db>
   6ccbf:	lea    (%rsi,%rbp,1),%r9
   6ccc3:	cmp    %r8,%r9
   6ccc6:	jae    6d8ba <verified_tiff::native_vp8::render_rgba+0xeca>
   6cccc:	add    0x48(%rsp),%r12
   6ccd1:	add    %r12,%r15
   6ccd4:	mov    %r15,0x20(%rsp)
   6ccd9:	cmp    %r8,%r15
   6ccdc:	jae    6db7a <verified_tiff::native_vp8::render_rgba+0x118a>
   6cce2:	add    %r12,%rbp
   6cce5:	cmp    %r8,%rbp
   6cce8:	jae    6d8cf <verified_tiff::native_vp8::render_rgba+0xedf>
   6ccee:	mov    %rbp,0x168(%rsp)
   6ccf6:	mov    %rax,0x60(%rsp)
   6ccfb:	mov    %r9,0x160(%rsp)
   6cd03:	mov    %rbx,0x170(%rsp)
   6cd0b:	mov    %r10,0x178(%rsp)
   6cd13:	mov    %rdi,0x40(%rsp)
   6cd18:	lea    0x1(%r11),%rax
   6cd1c:	add    %rdi,%rax
   6cd1f:	mov    %rax,%rdi
   6cd22:	cmp    %r8,%rax
   6cd25:	jae    6db5e <verified_tiff::native_vp8::render_rgba+0x116e>
   6cd2b:	mov    %r14,0x58(%rsp)
   6cd30:	mov    %r11,%rdx
   6cd33:	shr    $1,%rdx
   6cd36:	mov    %r11,%r15
   6cd39:	add    $0x2,%r11
   6cd3d:	mov    %r11,%rbp
   6cd40:	shr    $1,%rbp
   6cd43:	mov    0x28(%rsp),%r10
   6cd48:	cmp    %r10,%rbp
   6cd4b:	mov    %rbp,0x18(%rsp)
   6cd50:	cmovb  %rbp,%r10
   6cd54:	mov    0x10(%rsp),%rax
   6cd59:	lea    (%rax,%rdx,1),%rbp
   6cd5d:	cmp    %r8,%rbp
   6cd60:	jae    6d8cf <verified_tiff::native_vp8::render_rgba+0xedf>
   6cd66:	lea    (%rax,%r10,1),%r9
   6cd6a:	cmp    %r8,%r9
   6cd6d:	jae    6d8ba <verified_tiff::native_vp8::render_rgba+0xeca>
   6cd73:	mov    %rbp,0x150(%rsp)
   6cd7b:	mov    %r9,0x158(%rsp)
   6cd83:	lea    (%rcx,%rdx,1),%r9
   6cd87:	cmp    %r8,%r9
   6cd8a:	jae    6d8ba <verified_tiff::native_vp8::render_rgba+0xeca>
   6cd90:	mov    %r10,%r14
   6cd93:	mov    %rcx,%rbx
   6cd96:	lea    (%rcx,%r10,1),%rbp
   6cd9a:	cmp    %r8,%rbp
   6cd9d:	jae    6d8cf <verified_tiff::native_vp8::render_rgba+0xedf>
   6cda3:	mov    %rdx,%r10
   6cda6:	mov    %rbx,%rdx
   6cda9:	lea    (%rsi,%r10,1),%rbx
   6cdad:	mov    %rbx,0x88(%rsp)
   6cdb5:	cmp    %r8,%rbx
   6cdb8:	jae    6d99b <verified_tiff::native_vp8::render_rgba+0xfab>
   6cdbe:	mov    %r11,%rcx
   6cdc1:	mov    %rsi,%r11
   6cdc4:	mov    %rdi,0x50(%rsp)
   6cdc9:	add    %r14,%rsi
   6cdcc:	mov    %rsi,0x80(%rsp)
   6cdd4:	cmp    %r8,%rsi
   6cdd7:	jae    6dab1 <verified_tiff::native_vp8::render_rgba+0x10c1>
   6cddd:	add    %r12,%r10
   6cde0:	cmp    %r8,%r10
   6cde3:	mov    0x40(%rsp),%rdi
   6cde8:	jae    6d97f <verified_tiff::native_vp8::render_rgba+0xf8f>
   6cdee:	mov    %r14,%rbx
   6cdf1:	mov    %rbp,0x148(%rsp)
   6cdf9:	add    %r12,%rbx
   6cdfc:	cmp    %r8,%rbx
   6cdff:	jae    6d8e4 <verified_tiff::native_vp8::render_rgba+0xef4>
   6ce05:	mov    %rcx,%rsi
   6ce08:	add    %rdi,%rsi
   6ce0b:	mov    %rsi,0x38(%rsp)
   6ce10:	cmp    %r8,%rsi
   6ce13:	jae    6d968 <verified_tiff::native_vp8::render_rgba+0xf78>
   6ce19:	mov    %r11,%r14
   6ce1c:	mov    %r15,%r11
   6ce1f:	mov    %rbx,0x138(%rsp)
   6ce27:	mov    %r10,0x140(%rsp)
   6ce2f:	lea    0x3(%r15),%r10
   6ce33:	mov    %r10,%rbp
   6ce36:	shr    $1,%rbp
   6ce39:	mov    0x28(%rsp),%rsi
   6ce3e:	cmp    %rsi,%rbp
   6ce41:	cmovae %rsi,%rbp
   6ce45:	lea    (%rax,%r13,1),%rsi
   6ce49:	cmp    %r8,%rsi
   6ce4c:	jae    6da9c <verified_tiff::native_vp8::render_rgba+0x10ac>
   6ce52:	lea    (%rax,%rbp,1),%rbx
   6ce56:	mov    %rbx,0x30(%rsp)
   6ce5b:	cmp    %r8,%rbx
   6ce5e:	jae    6d951 <verified_tiff::native_vp8::render_rgba+0xf61>
   6ce64:	mov    %rdx,%rcx
   6ce67:	mov    %rsi,0x130(%rsp)
   6ce6f:	lea    (%rdx,%r13,1),%rax
   6ce73:	cmp    %r8,%rax
   6ce76:	jae    6da87 <verified_tiff::native_vp8::render_rgba+0x1097>
   6ce7c:	mov    %r9,0x128(%rsp)
   6ce84:	lea    (%rcx,%rbp,1),%r9
   6ce88:	mov    %r9,0x78(%rsp)
   6ce8d:	cmp    %r8,%r9
   6ce90:	jae    6d93a <verified_tiff::native_vp8::render_rgba+0xf4a>
   6ce96:	lea    (%r14,%r13,1),%r9
   6ce9a:	mov    %r9,0x70(%rsp)
   6ce9f:	cmp    %r8,%r9
   6cea2:	jae    6da70 <verified_tiff::native_vp8::render_rgba+0x1080>
   6cea8:	lea    (%r14,%rbp,1),%r9
   6ceac:	cmp    %r8,%r9
   6ceaf:	jae    6d8ba <verified_tiff::native_vp8::render_rgba+0xeca>
   6ceb5:	add    %r12,%r13
   6ceb8:	cmp    %r8,%r13
   6cebb:	jae    6da54 <verified_tiff::native_vp8::render_rgba+0x1064>
   6cec1:	add    %r12,%rbp
   6cec4:	cmp    %r8,%rbp
   6cec7:	jae    6d8cf <verified_tiff::native_vp8::render_rgba+0xedf>
   6cecd:	mov    %rax,0x118(%rsp)
   6ced5:	mov    %r13,0x120(%rsp)
   6cedd:	add    %r10,%rdi
   6cee0:	cmp    %r8,%rdi
   6cee3:	jae    6db5e <verified_tiff::native_vp8::render_rgba+0x116e>
   6cee9:	lea    0x4(%r11),%r13
   6ceed:	shr    $1,%r13
   6cef0:	mov    0x28(%rsp),%rsi
   6cef5:	cmp    %rsi,%r13
   6cef8:	cmovae %rsi,%r13
   6cefc:	mov    0x18(%rsp),%rax
   6cf01:	mov    0x10(%rsp),%rsi
   6cf06:	lea    (%rsi,%rax,1),%rbx
   6cf0a:	cmp    %r8,%rbx
   6cf0d:	jae    6d8e4 <verified_tiff::native_vp8::render_rgba+0xef4>
   6cf13:	mov    %rsi,%rax
   6cf16:	add    %r13,%rax
   6cf19:	mov    %rax,0x10(%rsp)
   6cf1e:	cmp    %r8,%rax
   6cf21:	jae    6d923 <verified_tiff::native_vp8::render_rgba+0xf33>
   6cf27:	mov    0x18(%rsp),%rax
   6cf2c:	lea    (%rcx,%rax,1),%r15
   6cf30:	cmp    %r8,%r15
   6cf33:	jae    6da3f <verified_tiff::native_vp8::render_rgba+0x104f>
   6cf39:	add    %r13,%rcx
   6cf3c:	cmp    %r8,%rcx
   6cf3f:	jae    6d8f9 <verified_tiff::native_vp8::render_rgba+0xf09>
   6cf45:	mov    0x18(%rsp),%rax
   6cf4a:	add    %r14,%rax
   6cf4d:	mov    %rax,0x40(%rsp)
   6cf52:	cmp    %r8,%rax
   6cf55:	jae    6da28 <verified_tiff::native_vp8::render_rgba+0x1038>
   6cf5b:	mov    %r9,0x110(%rsp)
   6cf63:	add    %r13,%r14
   6cf66:	cmp    %r8,%r14
   6cf69:	jae    6d90e <verified_tiff::native_vp8::render_rgba+0xf1e>
   6cf6f:	mov    %rdi,%rdx
   6cf72:	mov    %r11,%rsi
   6cf75:	mov    %r15,0x108(%rsp)
   6cf7d:	mov    0x18(%rsp),%r11
   6cf82:	add    %r12,%r11
   6cf85:	mov    %r11,0x18(%rsp)
   6cf8a:	cmp    %r8,%r11
   6cf8d:	mov    0x50(%rsp),%rdi
   6cf92:	jae    6db70 <verified_tiff::native_vp8::render_rgba+0x1180>
   6cf98:	add    %r13,%r12
   6cf9b:	cmp    %r8,%r12
   6cf9e:	jae    6db5b <verified_tiff::native_vp8::render_rgba+0x116b>
   6cfa4:	mov    %rbx,%r11
   6cfa7:	mov    0x68(%rsp),%r15
   6cfac:	mov    0x58(%rsp),%rax
   6cfb1:	movzbl (%r15,%rax,1),%eax
   6cfb6:	movzbl (%r15,%rdi,1),%r8d
   6cfbb:	shl    $0x8,%r8d
   6cfbf:	or     %eax,%r8d
   6cfc2:	mov    0x38(%rsp),%rax
   6cfc7:	movzbl (%r15,%rax,1),%ebx
   6cfcc:	shl    $0x10,%ebx
   6cfcf:	or     %ebx,%r8d
   6cfd2:	movzbl (%r15,%rdx,1),%edi
   6cfd7:	shl    $0x18,%edi
   6cfda:	or     %edi,%r8d
   6cfdd:	mov    %r8d,0x58(%rsp)
   6cfe2:	mov    0x60(%rsp),%rax
   6cfe7:	and    $0x1,%eax
   6cfea:	lea    0x1(,%rax,2),%ebx
   6cff1:	mov    $0x4,%r9d
   6cff7:	sub    %ebx,%r9d
   6cffa:	xor    %edi,%edi
   6cffc:	test   $0x1,%sil
   6d000:	sete   %dil
   6d004:	and    $0x1,%esi
   6d007:	lea    0x1(,%rsi,2),%r13d
   6d00f:	and    $0x1,%r10d
   6d013:	lea    0x1(,%r10,2),%eax
   6d01b:	mov    %ebx,%edx
   6d01d:	imul   %r13d,%edx
   6d021:	mov    %edx,%r10d
   6d024:	mov    %edx,0x50(%rsp)
   6d028:	movd   %eax,%xmm15
   6d02d:	pinsrw $0x1,%r13d,%xmm15
   6d033:	mov    %ebx,%edx
   6d035:	imul   %eax,%edx
   6d038:	mov    %edx,0x60(%rsp)
   6d03c:	mov    0x10(%rsp),%rax
   6d041:	movzbl (%r15,%rax,1),%eax
   6d046:	movdqa %xmm0,%xmm14
   6d04b:	psubw  %xmm15,%xmm14
   6d050:	movd   %ebx,%xmm1
   6d054:	pshuflw $0x0,%xmm1,%xmm1
   6d059:	pmullw %xmm14,%xmm1
   6d05e:	movzwl %ax,%r8d
   6d062:	movd   %xmm1,%eax
   6d066:	imul   %eax,%r8d
   6d06a:	movzbl (%r15,%r11,1),%eax
   6d06f:	movzwl %ax,%eax
   6d072:	imul   %edx,%eax
   6d075:	add    %eax,%r8d
   6d078:	mov    0x108(%rsp),%rax
   6d080:	movzbl (%r15,%rax,1),%eax
   6d085:	movd   %r9d,%xmm2
   6d08a:	pshuflw $0x0,%xmm2,%xmm2
   6d08f:	pmullw %xmm2,%xmm15
   6d094:	movzwl %ax,%r13d
   6d098:	movd   %xmm15,%eax
   6d09d:	imul   %eax,%r13d
   6d0a1:	movzbl (%r15,%rcx,1),%eax
   6d0a6:	pmullw %xmm2,%xmm14
   6d0ab:	movzwl %ax,%eax
   6d0ae:	movd   %xmm14,%ecx
   6d0b3:	imul   %ecx,%eax
   6d0b6:	add    %r13d,%eax
   6d0b9:	add    %r8d,%eax
   6d0bc:	mov    0x130(%rsp),%rcx
   6d0c4:	movzbl (%r15,%rcx,1),%ecx
   6d0c9:	mov    0x30(%rsp),%rsi
   6d0ce:	movzbl (%r15,%rsi,1),%r8d
   6d0d3:	movzwl %cx,%ecx
   6d0d6:	imul   %r10d,%ecx
   6d0da:	movzwl %r8w,%r8d
   6d0de:	pextrw $0x1,%xmm1,%edx
   6d0e3:	mov    %edx,0x10(%rsp)
   6d0e7:	imul   %edx,%r8d
   6d0eb:	add    %ecx,%r8d
   6d0ee:	mov    0x118(%rsp),%rcx
   6d0f6:	movzbl (%r15,%rcx,1),%ecx
   6d0fb:	mov    0x78(%rsp),%rsi
   6d100:	movzbl (%r15,%rsi,1),%r13d
   6d105:	movzwl %cx,%esi
   6d108:	pextrw $0x1,%xmm15,%ecx
   6d10e:	mov    %ecx,0x30(%rsp)
   6d112:	imul   %ecx,%esi
   6d115:	movzwl %r13w,%ecx
   6d119:	pextrw $0x1,%xmm14,%edx
   6d11f:	mov    %edx,0x38(%rsp)
   6d123:	imul   %edx,%ecx
   6d126:	add    %esi,%ecx
   6d128:	add    %r8d,%ecx
   6d12b:	mov    0x88(%rsp),%rsi
   6d133:	movzbl (%r15,%rsi,1),%esi
   6d138:	mov    0x80(%rsp),%rdx
   6d140:	movzbl (%r15,%rdx,1),%r11d
   6d145:	lea    0x1(,%rdi,2),%edi
   6d14c:	mov    %ebx,%r8d
   6d14f:	imul   %edi,%r8d
   6d153:	movzwl %si,%esi
   6d156:	imul   %r8d,%esi
   6d15a:	mov    $0x4,%r13d
   6d160:	sub    %edi,%r13d
   6d163:	imul   %r13d,%ebx
   6d167:	movzwl %r11w,%r11d
   6d16b:	imul   %ebx,%r11d
   6d16f:	add    %esi,%r11d
   6d172:	imul   %r9d,%edi
   6d176:	imul   %r9d,%r13d
   6d17a:	mov    0x140(%rsp),%rdx
   6d182:	movzbl (%r15,%rdx,1),%edx
   6d187:	mov    0x138(%rsp),%rsi
   6d18f:	movzbl (%r15,%rsi,1),%esi
   6d194:	movzwl %dx,%r10d
   6d198:	imul   %edi,%r10d
   6d19c:	movzwl %si,%edx
   6d19f:	imul   %r13d,%edx
   6d1a3:	add    %r10d,%edx
   6d1a6:	add    %r11d,%edx
   6d1a9:	mov    0x150(%rsp),%rsi
   6d1b1:	movzbl (%r15,%rsi,1),%esi
   6d1b6:	movzwl %si,%esi
   6d1b9:	imul   %r8d,%esi
   6d1bd:	mov    0x158(%rsp),%r8
   6d1c5:	movzbl (%r15,%r8,1),%r8d
   6d1ca:	movzwl %r8w,%r10d
   6d1ce:	imul   %ebx,%r10d
   6d1d2:	add    %esi,%r10d
   6d1d5:	mov    0x128(%rsp),%rsi
   6d1dd:	movzbl (%r15,%rsi,1),%esi
   6d1e2:	movzwl %si,%esi
   6d1e5:	imul   %edi,%esi
   6d1e8:	mov    0x148(%rsp),%rdi
   6d1f0:	movzbl (%r15,%rdi,1),%edi
   6d1f5:	movzwl %di,%r8d
   6d1f9:	imul   %r13d,%r8d
   6d1fd:	add    %esi,%r8d
   6d200:	add    %r10d,%r8d
   6d203:	mov    0x90(%rsp),%rsi
   6d20b:	movzbl (%r15,%rsi,1),%esi
   6d210:	mov    0x160(%rsp),%rdi
   6d218:	movzbl (%r15,%rdi,1),%edi
   6d21d:	movzwl %si,%esi
   6d220:	mov    0x50(%rsp),%r9d
   6d225:	imul   %r9d,%esi
   6d229:	movzwl %di,%edi
   6d22c:	mov    0x10(%rsp),%ebx
   6d230:	imul   %ebx,%edi
   6d233:	add    %esi,%edi
   6d235:	mov    0x20(%rsp),%rsi
   6d23a:	movzbl (%r15,%rsi,1),%esi
   6d23f:	movzwl %si,%esi
   6d242:	mov    0x30(%rsp),%r13d
   6d247:	imul   %r13d,%esi
   6d24b:	add    %edi,%esi
   6d24d:	mov    0x168(%rsp),%rdi
   6d255:	movzbl (%r15,%rdi,1),%edi
   6d25a:	movzwl %di,%edi
   6d25d:	mov    0x38(%rsp),%r11d
   6d262:	imul   %r11d,%edi
   6d266:	lea    (%rdi,%rsi,1),%r10d
   6d26a:	add    $0x8,%r10d
   6d26e:	mov    0x180(%rsp),%rsi
   6d276:	movzbl (%r15,%rsi,1),%esi
   6d27b:	movzwl %si,%esi
   6d27e:	imul   %ebx,%esi
   6d281:	mov    0x98(%rsp),%rdi
   6d289:	movzbl (%r15,%rdi,1),%edi
   6d28e:	movzwl %di,%edi
   6d291:	imul   %r9d,%edi
   6d295:	add    %edi,%esi
   6d297:	mov    0x170(%rsp),%rdi
   6d29f:	movzbl (%r15,%rdi,1),%edi
   6d2a4:	movzwl %di,%edi
   6d2a7:	imul   %r13d,%edi
   6d2ab:	add    %esi,%edi
   6d2ad:	mov    0x178(%rsp),%rsi
   6d2b5:	movzbl (%r15,%rsi,1),%esi
   6d2ba:	movzwl %si,%esi
   6d2bd:	imul   %r11d,%esi
   6d2c1:	add    %esi,%edi
   6d2c3:	add    $0x8,%edi
   6d2c6:	movzbl (%r15,%r14,1),%esi
   6d2cb:	movd   %esi,%xmm2
   6d2cf:	mov    0x110(%rsp),%rsi
   6d2d7:	movzbl (%r15,%rsi,1),%esi
   6d2dc:	pinsrw $0x1,%esi,%xmm2
   6d2e1:	mov    0x70(%rsp),%rsi
   6d2e6:	movzbl (%r15,%rsi,1),%esi
   6d2eb:	movzwl %si,%esi
   6d2ee:	imul   %r9d,%esi
   6d2f2:	mov    0x18(%rsp),%r9
   6d2f7:	movzbl (%r15,%r9,1),%r9d
   6d2fc:	movd   %r9d,%xmm4
   6d301:	mov    0x120(%rsp),%r9
   6d309:	movzbl (%r15,%r9,1),%r9d
   6d30e:	pinsrw $0x1,%r9d,%xmm4
   6d314:	mov    0x40(%rsp),%r9
   6d319:	movzbl (%r15,%r9,1),%r9d
   6d31e:	movzwl %r9w,%r9d
   6d322:	imul   0x60(%rsp),%r9d
   6d328:	pmullw %xmm1,%xmm2
   6d32c:	movzbl (%r15,%r12,1),%r11d
   6d331:	movd   %r11d,%xmm5
   6d336:	movzbl (%r15,%rbp,1),%r11d
   6d33b:	pinsrw $0x1,%r11d,%xmm5
   6d341:	pmullw %xmm15,%xmm4
   6d346:	movd   %r9d,%xmm1
   6d34b:	pinsrw $0x1,%esi,%xmm1
   6d350:	shl    $0x14,%eax
   6d353:	shl    $0xc,%ecx
   6d356:	shl    $0x4,%edx
   6d359:	shl    $0x4,%r8d
   6d35d:	shr    $0x4,%r10d
   6d361:	shr    $0x4,%edi
   6d364:	pmullw %xmm14,%xmm5
   6d369:	paddw  %xmm4,%xmm5
   6d36d:	paddw  %xmm2,%xmm1
   6d371:	paddw  %xmm5,%xmm1
   6d375:	paddw  0xd9f93(%rip),%xmm1        # 147310 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x230>
   6d37d:	psrlw  $0x4,%xmm1
   6d382:	add    $0x800000,%eax
   6d387:	and    $0xff000000,%eax
   6d38c:	add    $0x8000,%ecx
   6d392:	and    $0xff0000,%ecx
   6d398:	or     %eax,%ecx
   6d39a:	sub    $0xffffff80,%r8d
   6d39e:	and    $0xff00,%r8d
   6d3a5:	or     %ecx,%r8d
   6d3a8:	movzbl %dil,%eax
   6d3ac:	or     %r8d,%eax
   6d3af:	movdqa %xmm1,%xmm2
   6d3b3:	pslld  $0x18,%xmm2
   6d3b8:	pand   0xd9f60(%rip),%xmm1        # 147320 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x240>
   6d3c0:	por    %xmm2,%xmm1
   6d3c4:	movd   %xmm1,%ecx
   6d3c8:	sub    $0xffffff80,%edx
   6d3cb:	and    $0xff00,%edx
   6d3d1:	or     %ecx,%edx
   6d3d3:	movzbl %r10b,%ecx
   6d3d7:	or     %edx,%ecx
   6d3d9:	movd   %eax,%xmm1
   6d3dd:	pxor   %xmm4,%xmm4
   6d3e1:	punpcklbw %xmm1,%xmm4
   6d3e5:	movdqa %xmm4,%xmm2
   6d3e9:	movd   0x58(%rsp),%xmm5
   6d3ef:	pxor   %xmm1,%xmm1
   6d3f3:	punpcklbw %xmm5,%xmm1
   6d3f7:	pmulhuw 0xd9f41(%rip),%xmm1        # 147340 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x260>
   6d3ff:	punpcklwd %xmm3,%xmm1
   6d403:	pmulhuw %xmm6,%xmm4
   6d407:	punpcklwd %xmm3,%xmm4
   6d40b:	movd   %ecx,%xmm5
   6d40f:	pxor   %xmm14,%xmm14
   6d414:	punpcklbw %xmm5,%xmm14
   6d419:	movdqa %xmm14,%xmm5
   6d41e:	pmulhuw %xmm7,%xmm5
   6d422:	punpcklwd %xmm3,%xmm5
   6d426:	paddd  %xmm4,%xmm5
   6d42a:	movdqa %xmm1,%xmm15
   6d42f:	psubd  %xmm5,%xmm15
   6d434:	pmulhuw 0xd9ef4(%rip),%xmm2        # 147330 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x250>
   6d43c:	punpcklwd %xmm3,%xmm2
   6d440:	pmulhuw %xmm10,%xmm14
   6d445:	punpcklwd %xmm3,%xmm14
   6d44a:	paddd  %xmm1,%xmm14
   6d44f:	paddd  %xmm2,%xmm1
   6d453:	paddd  %xmm8,%xmm15
   6d458:	psrad  $0x6,%xmm15
   6d45e:	packssdw %xmm3,%xmm15
   6d463:	pmaxsw %xmm3,%xmm15
   6d468:	pminsw %xmm9,%xmm15
   6d46d:	paddd  %xmm11,%xmm14
   6d472:	psrad  $0x6,%xmm14
   6d478:	packssdw %xmm3,%xmm14
   6d47d:	pmaxsw %xmm3,%xmm14
   6d482:	pminsw %xmm9,%xmm14
   6d487:	movdqa %xmm15,0xf0(%rsp)
   6d491:	movzbl 0xf6(%rsp),%ebp
   6d499:	movzbl 0xf4(%rsp),%esi
   6d4a1:	movdqa %xmm14,0xe0(%rsp)
   6d4ab:	movzbl 0xe6(%rsp),%edx
   6d4b3:	movzbl 0xe4(%rsp),%r8d
   6d4bc:	paddd  %xmm12,%xmm1
   6d4c1:	psrad  $0x6,%xmm1
   6d4c6:	packssdw %xmm3,%xmm1
   6d4ca:	pmaxsw %xmm3,%xmm1
   6d4ce:	pminsw %xmm9,%xmm1
   6d4d3:	movdqa %xmm1,0xd0(%rsp)
   6d4dc:	mov    0xd0(%rsp),%edi
   6d4e3:	movzbl 0xf0(%rsp),%eax
   6d4eb:	shl    $0x8,%eax
   6d4ee:	movzbl 0xe0(%rsp),%r10d
   6d4f7:	or     %eax,%r10d
   6d4fa:	mov    0xd4(%rsp),%r11d
   6d502:	shl    $0x10,%edi
   6d505:	or     %r10d,%edi
   6d508:	movzbl 0xd6(%rsp),%eax
   6d510:	or     $0xff000000,%edi
   6d516:	mov    0xb8(%rsp),%rbx
   6d51e:	mov    0x188(%rsp),%r12
   6d526:	mov    %edi,(%rbx,%r12,4)
   6d52a:	lea    0x1(%r12),%rdi
   6d52f:	mov    0x100(%rsp),%r9
   6d537:	cmp    %r9,%rdi
   6d53a:	jae    6dba1 <verified_tiff::native_vp8::render_rgba+0x11b1>
   6d540:	movd   %xmm15,%edi
   6d545:	shr    $0x8,%edi
   6d548:	and    $0xff00,%edi
   6d54e:	movd   %xmm14,%r10d
   6d553:	shr    $0x10,%r10d
   6d557:	movzbl %r10b,%r10d
   6d55b:	or     %edi,%r10d
   6d55e:	movd   %xmm1,%edi
   6d562:	and    $0xffff0000,%edi
   6d568:	or     %r10d,%edi
   6d56b:	or     $0xff000000,%edi
   6d571:	mov    %edi,0x4(%rbx,%r12,4)
   6d576:	lea    0x2(%r12),%rdi
   6d57b:	cmp    %r9,%rdi
   6d57e:	mov    0xb0(%rsp),%rcx
   6d586:	mov    0xa8(%rsp),%r10
   6d58e:	mov    0xa0(%rsp),%r13
   6d596:	mov    0x28(%rsp),%r15
   6d59b:	jae    6db4b <verified_tiff::native_vp8::render_rgba+0x115b>
   6d5a1:	shl    $0x8,%esi
   6d5a4:	or     %r8d,%esi
   6d5a7:	shl    $0x10,%r11d
   6d5ab:	or     %esi,%r11d
   6d5ae:	or     $0xff000000,%r11d
   6d5b5:	mov    %r11d,0x8(%rbx,%r12,4)
   6d5ba:	lea    0x3(%r12),%rdi
   6d5bf:	cmp    %r9,%rdi
   6d5c2:	jae    6db91 <verified_tiff::native_vp8::render_rgba+0x11a1>
   6d5c8:	shl    $0x8,%ebp
   6d5cb:	or     %edx,%ebp
   6d5cd:	shl    $0x10,%eax
   6d5d0:	or     %ebp,%eax
   6d5d2:	or     $0xff000000,%eax
   6d5d7:	mov    $0x4,%edx
   6d5dc:	jmp    6cb7c <verified_tiff::native_vp8::render_rgba+0x18c>
   6d5e1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6d5f0:	add    %r11,%rdi
   6d5f3:	cmp    %r8,%rdi
   6d5f6:	jae    6da18 <verified_tiff::native_vp8::render_rgba+0x1028>
   6d5fc:	lea    -0x1(%r11),%r14
   6d600:	shr    $1,%r14
   6d603:	cmp    $0x1,%r11
   6d607:	mov    $0x0,%edx
   6d60c:	cmovb  %rdx,%r14
   6d610:	lea    0x1(%r11),%rsi
   6d614:	shr    $1,%rsi
   6d617:	cmp    %r15,%rsi
   6d61a:	cmovae %r15,%rsi
   6d61e:	lea    -0x1(%rax),%r15
   6d622:	shr    $1,%r15
   6d625:	imul   %rbx,%r15
   6d629:	mov    %r12,%r9
   6d62c:	cmp    %r12,%r10
   6d62f:	cmova  %rdx,%r15
   6d633:	lea    0x1(%rax),%r12
   6d637:	shr    $1,%r12
   6d63a:	cmp    %rcx,%r12
   6d63d:	cmovae %rcx,%r12
   6d641:	lea    (%r15,%r13,1),%rcx
   6d645:	lea    (%rcx,%r14,1),%rbp
   6d649:	cmp    %r8,%rbp
   6d64c:	jae    6db38 <verified_tiff::native_vp8::render_rgba+0x1148>
   6d652:	add    %rsi,%rcx
   6d655:	cmp    %r8,%rcx
   6d658:	jae    6da05 <verified_tiff::native_vp8::render_rgba+0x1015>
   6d65e:	imul   %rbx,%r12
   6d662:	lea    (%r12,%r13,1),%r10
   6d666:	lea    (%r10,%r14,1),%rbx
   6d66a:	cmp    %r8,%rbx
   6d66d:	jae    6db25 <verified_tiff::native_vp8::render_rgba+0x1135>
   6d673:	add    %rsi,%r10
   6d676:	cmp    %r8,%r10
   6d679:	jae    6d9f2 <verified_tiff::native_vp8::render_rgba+0x1002>
   6d67f:	add    0x48(%rsp),%r15
   6d684:	lea    (%r15,%r14,1),%r13
   6d688:	cmp    %r8,%r13
   6d68b:	jae    6db12 <verified_tiff::native_vp8::render_rgba+0x1122>
   6d691:	add    %rsi,%r15
   6d694:	cmp    %r8,%r15
   6d697:	jae    6d9df <verified_tiff::native_vp8::render_rgba+0xfef>
   6d69d:	add    0x48(%rsp),%r12
   6d6a2:	add    %r12,%r14
   6d6a5:	cmp    %r8,%r14
   6d6a8:	jae    6daff <verified_tiff::native_vp8::render_rgba+0x110f>
   6d6ae:	add    %rsi,%r12
   6d6b1:	cmp    %r8,%r12
   6d6b4:	jae    6d9ca <verified_tiff::native_vp8::render_rgba+0xfda>
   6d6ba:	and    $0x1,%r11d
   6d6be:	lea    0x1(,%r11,2),%esi
   6d6c6:	and    $0x1,%eax
   6d6c9:	lea    0x1(,%rax,2),%edx
   6d6d0:	movdqa %xmm13,%xmm2
   6d6d5:	pinsrw $0x0,%edx,%xmm2
   6d6da:	pxor   %xmm1,%xmm1
   6d6de:	pinsrw $0x1,%esi,%xmm1
   6d6e3:	pinsrw $0x2,%edx,%xmm1
   6d6e8:	mov    0x68(%rsp),%r8
   6d6ed:	movzbl (%r8,%rdi,1),%eax
   6d6f2:	pshuflw $0x64,%xmm1,%xmm14
   6d6f8:	psubw  %xmm14,%xmm2
   6d6fd:	punpcklwd %xmm2,%xmm1
   6d701:	pshufd $0xe6,%xmm1,%xmm1
   6d706:	pshuflw $0x62,%xmm1,%xmm1
   6d70b:	pmullw %xmm2,%xmm1
   6d70f:	movzbl (%r8,%r13,1),%edx
   6d714:	movzbl (%r8,%r15,1),%esi
   6d719:	movd   %edx,%xmm2
   6d71d:	pinsrw $0x1,%esi,%xmm2
   6d722:	movzbl (%r8,%r14,1),%edx
   6d727:	pinsrw $0x2,%edx,%xmm2
   6d72c:	movzbl (%r8,%r12,1),%edx
   6d731:	pinsrw $0x3,%edx,%xmm2
   6d736:	pmullw %xmm1,%xmm2
   6d73a:	pshufd $0x55,%xmm2,%xmm14
   6d740:	paddw  %xmm2,%xmm14
   6d745:	movdqa %xmm14,%xmm2
   6d74a:	psrld  $0x10,%xmm2
   6d74f:	paddw  %xmm14,%xmm2
   6d754:	movd   %xmm2,%edx
   6d758:	add    $0x8,%edx
   6d75b:	shr    $0x4,%edx
   6d75e:	imul   $0x4a85,%eax,%esi
   6d764:	shr    $0x8,%esi
   6d767:	movzbl %dl,%edi
   6d76a:	imul   $0x6625,%edi,%r8d
   6d771:	shr    $0x8,%r8d
   6d775:	lea    (%r8,%rsi,1),%r11d
   6d779:	xor    %eax,%eax
   6d77b:	mov    $0x0,%edx
   6d780:	cmp    $0x379a,%r11d
   6d787:	jb     6d7a7 <verified_tiff::native_vp8::render_rgba+0xdb7>
   6d789:	mov    $0xff,%edx
   6d78e:	cmp    $0x7799,%r11d
   6d795:	ja     6d7a7 <verified_tiff::native_vp8::render_rgba+0xdb7>
   6d797:	lea    (%r8,%rsi,1),%edx
   6d79b:	add    $0x866,%edx
   6d7a1:	shr    $0x6,%edx
   6d7a4:	movzbl %dl,%edx
   6d7a7:	mov    0x68(%rsp),%r11
   6d7ac:	movzbl (%r11,%rbp,1),%r8d
   6d7b1:	movzbl (%r11,%rcx,1),%ecx
   6d7b6:	movzbl (%r11,%rbx,1),%ebp
   6d7bb:	movzbl (%r11,%r10,1),%r10d
   6d7c0:	pextrw $0x3,%xmm1,%r11d
   6d7c6:	pextrw $0x2,%xmm1,%ebx
   6d7cb:	imul   %r10d,%r11d
   6d7cf:	imul   %ebp,%ebx
   6d7d2:	pextrw $0x1,%xmm1,%ebp
   6d7d7:	movd   %xmm1,%r10d
   6d7dc:	movzwl %r8w,%r8d
   6d7e0:	imul   %r10d,%r8d
   6d7e4:	movzwl %cx,%ecx
   6d7e7:	imul   %ebp,%ecx
   6d7ea:	add    %r8d,%ecx
   6d7ed:	add    %ebx,%ecx
   6d7ef:	add    %r11d,%ecx
   6d7f2:	add    $0x8,%ecx
   6d7f5:	movzwl %cx,%ecx
   6d7f8:	shr    $0x4,%ecx
   6d7fb:	movzwl %cx,%ecx
   6d7fe:	movzbl %cl,%r11d
   6d802:	imul   $0x1913,%r11d,%r8d
   6d809:	shr    $0x8,%r8d
   6d80d:	imul   $0x3408,%edi,%r10d
   6d814:	shr    $0x8,%r10d
   6d818:	add    %r8d,%r10d
   6d81b:	mov    %esi,%edi
   6d81d:	sub    %r10d,%edi
   6d820:	cmp    $0xffffddfc,%edi
   6d826:	mov    0xb8(%rsp),%rbx
   6d82e:	mov    0xb0(%rsp),%rcx
   6d836:	mov    0xa8(%rsp),%r10
   6d83e:	mov    0xa0(%rsp),%r13
   6d846:	mov    0x28(%rsp),%r15
   6d84b:	mov    %r9,%r12
   6d84e:	jl     6d869 <verified_tiff::native_vp8::render_rgba+0xe79>
   6d850:	mov    $0xff00,%eax
   6d855:	cmp    $0x1dfb,%edi
   6d85b:	jg     6d869 <verified_tiff::native_vp8::render_rgba+0xe79>
   6d85d:	lea    0x8810(,%rdi,4),%eax
   6d864:	and    $0xff00,%eax
   6d869:	imul   $0x811a,%r11d,%edi
   6d870:	shr    $0x8,%edi
   6d873:	add    %esi,%edi
   6d875:	mov    $0xff000000,%esi
   6d87a:	cmp    $0x4515,%edi
   6d880:	jb     6cb70 <verified_tiff::native_vp8::render_rgba+0x180>
   6d886:	mov    $0xffff0000,%esi
   6d88b:	cmp    $0x8514,%edi
   6d891:	ja     6cb70 <verified_tiff::native_vp8::render_rgba+0x180>
   6d897:	shl    $0xa,%edi
   6d89a:	add    $0xebac00,%edi
   6d8a0:	and    $0xff0000,%edi
   6d8a6:	or     $0xff000000,%edi
   6d8ac:	mov    %edi,%esi
   6d8ae:	jmp    6cb70 <verified_tiff::native_vp8::render_rgba+0x180>
   6d8b3:	mov    $0xff,%al
   6d8b5:	jmp    6ca30 <verified_tiff::native_vp8::render_rgba+0x40>
   6d8ba:	mov    %r9,%rdi
   6d8bd:	lea    0x127dac(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d8c4:	mov    0x8(%rsp),%rsi
   6d8c9:	call   *0x130de1(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d8cf:	mov    %rbp,%rdi
   6d8d2:	lea    0x127d97(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d8d9:	mov    0x8(%rsp),%rsi
   6d8de:	call   *0x130dcc(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d8e4:	mov    %rbx,%rdi
   6d8e7:	lea    0x127d82(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d8ee:	mov    0x8(%rsp),%rsi
   6d8f3:	call   *0x130db7(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d8f9:	mov    %rcx,%rdi
   6d8fc:	lea    0x127d6d(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d903:	mov    0x8(%rsp),%rsi
   6d908:	call   *0x130da2(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d90e:	mov    %r14,%rdi
   6d911:	lea    0x127d58(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d918:	mov    0x8(%rsp),%rsi
   6d91d:	call   *0x130d8d(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d923:	mov    0x10(%rsp),%rdi
   6d928:	lea    0x127d41(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d92f:	mov    0x8(%rsp),%rsi
   6d934:	call   *0x130d76(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d93a:	mov    0x78(%rsp),%rdi
   6d93f:	lea    0x127d2a(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d946:	mov    0x8(%rsp),%rsi
   6d94b:	call   *0x130d5f(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d951:	mov    0x30(%rsp),%rdi
   6d956:	lea    0x127d13(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d95d:	mov    0x8(%rsp),%rsi
   6d962:	call   *0x130d48(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d968:	mov    0x38(%rsp),%rdi
   6d96d:	lea    0x127cfc(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d974:	mov    0x8(%rsp),%rsi
   6d979:	call   *0x130d31(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d97f:	mov    %r10,0x20(%rsp)
   6d984:	lea    0x127ce5(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d98b:	mov    0x20(%rsp),%rdi
   6d990:	mov    0x8(%rsp),%rsi
   6d995:	call   *0x130d15(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d99b:	mov    0x88(%rsp),%rdi
   6d9a3:	lea    0x127cc6(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d9aa:	mov    0x8(%rsp),%rsi
   6d9af:	call   *0x130cfb(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d9b5:	mov    %r10,%rdi
   6d9b8:	lea    0x127cb1(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d9bf:	mov    0x8(%rsp),%rsi
   6d9c4:	call   *0x130ce6(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d9ca:	lea    0x127c9f(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d9d1:	mov    %r12,%rdi
   6d9d4:	mov    0x8(%rsp),%rsi
   6d9d9:	call   *0x130cd1(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d9df:	lea    0x127c8a(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d9e6:	mov    %r15,%rdi
   6d9e9:	mov    %r8,%rsi
   6d9ec:	call   *0x130cbe(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6d9f2:	lea    0x127c77(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6d9f9:	mov    %r10,%rdi
   6d9fc:	mov    %r8,%rsi
   6d9ff:	call   *0x130cab(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6da05:	lea    0x127c64(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6da0c:	mov    %rcx,%rdi
   6da0f:	mov    %r8,%rsi
   6da12:	call   *0x130c98(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6da18:	lea    0x127c51(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6da1f:	mov    %r8,%rsi
   6da22:	call   *0x130c88(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6da28:	mov    0x40(%rsp),%rdi
   6da2d:	lea    0x127c3c(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6da34:	mov    0x8(%rsp),%rsi
   6da39:	call   *0x130c71(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6da3f:	mov    %r15,%rdi
   6da42:	lea    0x127c27(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6da49:	mov    0x8(%rsp),%rsi
   6da4e:	call   *0x130c5c(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6da54:	mov    %r13,0x20(%rsp)
   6da59:	lea    0x127c10(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6da60:	mov    0x20(%rsp),%rdi
   6da65:	mov    0x8(%rsp),%rsi
   6da6a:	call   *0x130c40(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6da70:	mov    0x70(%rsp),%rdi
   6da75:	lea    0x127bf4(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6da7c:	mov    0x8(%rsp),%rsi
   6da81:	call   *0x130c29(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6da87:	mov    %rax,%rdi
   6da8a:	lea    0x127bdf(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6da91:	mov    0x8(%rsp),%rsi
   6da96:	call   *0x130c14(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6da9c:	mov    %rsi,%rdi
   6da9f:	lea    0x127bca(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6daa6:	mov    0x8(%rsp),%rsi
   6daab:	call   *0x130bff(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6dab1:	mov    0x80(%rsp),%rdi
   6dab9:	lea    0x127bb0(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6dac0:	mov    0x8(%rsp),%rsi
   6dac5:	call   *0x130be5(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6dacb:	mov    0x90(%rsp),%rdi
   6dad3:	lea    0x127b96(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6dada:	mov    0x8(%rsp),%rsi
   6dadf:	call   *0x130bcb(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6dae5:	mov    0x98(%rsp),%rdi
   6daed:	lea    0x127b7c(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6daf4:	mov    0x8(%rsp),%rsi
   6daf9:	call   *0x130bb1(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6daff:	lea    0x127b6a(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6db06:	mov    %r14,%rdi
   6db09:	mov    %r8,%rsi
   6db0c:	call   *0x130b9e(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6db12:	lea    0x127b57(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6db19:	mov    %r13,%rdi
   6db1c:	mov    %r8,%rsi
   6db1f:	call   *0x130b8b(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6db25:	lea    0x127b44(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6db2c:	mov    %rbx,%rdi
   6db2f:	mov    %r8,%rsi
   6db32:	call   *0x130b78(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6db38:	lea    0x127b31(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6db3f:	mov    %rbp,%rdi
   6db42:	mov    %r8,%rsi
   6db45:	call   *0x130b65(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6db4b:	lea    0x12790e(%rip),%rdx        # 195460 <anon.971dd2fb0ac7b98caeb780421c768e9a.16.llvm.1273966575265019903+0x78>
   6db52:	mov    %r9,%rsi
   6db55:	call   *0x130b55(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6db5b:	mov    %r12,%rdi
   6db5e:	lea    0x127b0b(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6db65:	mov    0x8(%rsp),%rsi
   6db6a:	call   *0x130b40(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6db70:	mov    0x18(%rsp),%rax
   6db75:	mov    %rax,0x20(%rsp)
   6db7a:	lea    0x127aef(%rip),%rdx        # 195670 <anon.4f3d76123c243ca47d579c4aabd055e1.28.llvm.6207170786339603499+0x48>
   6db81:	mov    0x20(%rsp),%rdi
   6db86:	mov    0x8(%rsp),%rsi
   6db8b:	call   *0x130b1f(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6db91:	lea    0x1278e0(%rip),%rdx        # 195478 <anon.971dd2fb0ac7b98caeb780421c768e9a.16.llvm.1273966575265019903+0x90>
   6db98:	mov    %r9,%rsi
   6db9b:	call   *0x130b0f(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6dba1:	lea    0x1278a0(%rip),%rdx        # 195448 <anon.971dd2fb0ac7b98caeb780421c768e9a.16.llvm.1273966575265019903+0x60>
   6dba8:	mov    %r9,%rsi
   6dbab:	call   *0x130aff(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   6dbb1:	cs nopw 0x0(%rax,%rax,1)
   6dbbb:	nopl   0x0(%rax,%rax,1)


0000000000086d00 <verified_tiff::webp_spans::colors>:
   86d00:	push   %rbp
   86d01:	push   %r15
   86d03:	push   %r14
   86d05:	push   %r13
   86d07:	push   %r12
   86d09:	push   %rbx
   86d0a:	sub    $0x68,%rsp
   86d0e:	mov    0xa0(%rsp),%rbx
   86d16:	add    %r9,%rbx
   86d19:	cmp    %rbx,%r9
   86d1c:	jae    87366 <verified_tiff::webp_spans::colors+0x666>
   86d22:	mov    0xa8(%rsp),%r10d
   86d2a:	mov    %r10d,%eax
   86d2d:	shr    $0x8,%eax
   86d30:	mov    %r10d,%edx
   86d33:	shr    $0x10,%edx
   86d36:	mov    %edx,%r11d
   86d39:	movzbl %dl,%esi
   86d3c:	and    $0x7f,%edx
   86d3f:	or     $0xffffff00,%r11d
   86d46:	test   $0x800000,%r10d
   86d4d:	cmove  %edx,%r11d
   86d51:	mov    %r11d,0x28(%rsp)
   86d56:	mov    %eax,%r11d
   86d59:	movzbl %al,%edx
   86d5c:	and    $0x7f,%eax
   86d5f:	or     $0xffffff00,%r11d
   86d66:	test   %r10w,%r10w
   86d6a:	cmovns %eax,%r11d
   86d6e:	mov    %r11d,0x24(%rsp)
   86d73:	mov    (%rdi),%r14
   86d76:	mov    %r10d,%eax
   86d79:	mov    %r10d,%edi
   86d7c:	and    $0x7f,%edi
   86d7f:	or     $0x1f00,%r10d
   86d86:	test   %al,%al
   86d88:	cmovns %edi,%r10d
   86d8c:	mov    %r10d,0x2c(%rsp)
   86d91:	imul   $0x1010101,%esi,%esi
   86d97:	movd   %esi,%xmm0
   86d9b:	pxor   %xmm1,%xmm1
   86d9f:	pcmpgtb %xmm0,%xmm1
   86da3:	punpcklbw %xmm1,%xmm0
   86da7:	imul   $0x1010101,%edx,%edx
   86dad:	movd   %edx,%xmm2
   86db1:	pxor   %xmm1,%xmm1
   86db5:	pcmpgtb %xmm2,%xmm1
   86db9:	punpcklbw %xmm1,%xmm2
   86dbd:	movzbl %al,%eax
   86dc0:	imul   $0x1010101,%eax,%eax
   86dc6:	movd   %eax,%xmm3
   86dca:	pxor   %xmm1,%xmm1
   86dce:	pcmpgtb %xmm3,%xmm1
   86dd2:	punpcklbw %xmm1,%xmm3
   86dd6:	movdqa 0xc0a61(%rip),%xmm10        # 147840 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x760>
   86ddf:	movdqa 0xc0a68(%rip),%xmm11        # 147850 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x770>
   86de8:	movdqa 0xc0a6f(%rip),%xmm12        # 147860 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x780>
   86df1:	movdqa 0xc0a76(%rip),%xmm13        # 147870 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x790>
   86dfa:	movdqa 0xc0a8d(%rip),%xmm15        # 147890 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x7b0>
   86e03:	mov    %r8,0x60(%rsp)
   86e08:	mov    %rbx,0x58(%rsp)
   86e0d:	mov    %r14,0x50(%rsp)
   86e12:	movdqa 0xc09e5(%rip),%xmm9        # 147800 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x720>
   86e1b:	jmp    86e44 <verified_tiff::webp_spans::colors+0x144>
   86e1d:	nopl   (%rax)
   86e20:	mov    %sil,(%rcx,%r9,4)
   86e24:	mov    %dl,0x1(%rcx,%r9,4)
   86e29:	mov    %dil,0x2(%rcx,%r9,4)
   86e2e:	mov    %al,0x3(%rcx,%r9,4)
   86e33:	mov    $0x1,%eax
   86e38:	add    %rax,%r9
   86e3b:	cmp    %rbx,%r9
   86e3e:	jae    87366 <verified_tiff::webp_spans::colors+0x666>
   86e44:	mov    %rbx,%rax
   86e47:	sub    %r9,%rax
   86e4a:	cmp    $0x3,%rax
   86e4e:	jbe    870c0 <verified_tiff::webp_spans::colors+0x3c0>
   86e54:	cmp    %r8,%r9
   86e57:	jae    873a5 <verified_tiff::webp_spans::colors+0x6a5>
   86e5d:	lea    0x1(%r9),%rdi
   86e61:	cmp    %r8,%rdi
   86e64:	jae    87385 <verified_tiff::webp_spans::colors+0x685>
   86e6a:	lea    0x2(%r9),%rdi
   86e6e:	cmp    %r8,%rdi
   86e71:	jae    87395 <verified_tiff::webp_spans::colors+0x695>
   86e77:	lea    0x3(%r9),%rdi
   86e7b:	cmp    %r8,%rdi
   86e7e:	jae    87375 <verified_tiff::webp_spans::colors+0x675>
   86e84:	mov    (%rcx,%r9,4),%r13d
   86e88:	mov    0x4(%rcx,%r9,4),%r15d
   86e8d:	movq   0x8(%rcx,%r9,4),%xmm14
   86e94:	mov    0x8(%rcx,%r9,4),%ebx
   86e99:	mov    0xc(%rcx,%r9,4),%edi
   86e9e:	cmp    $0x2,%r14
   86ea2:	jne    87180 <verified_tiff::webp_spans::colors+0x480>
   86ea8:	movd   %r13d,%xmm6
   86ead:	movd   %r13d,%xmm4
   86eb2:	movd   %r15d,%xmm1
   86eb7:	movdqa %xmm1,%xmm5
   86ebb:	punpcklqdq %xmm6,%xmm5
   86ebf:	shufps $0x42,%xmm14,%xmm5
   86ec4:	pand   0xc0924(%rip),%xmm7        # 1477f0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x710>
   86ecc:	movq   %xmm4,%xmm6
   86ed0:	por    %xmm7,%xmm6
   86ed4:	movdqa %xmm9,%xmm7
   86ed9:	pandn  %xmm6,%xmm7
   86edd:	movdqa %xmm4,%xmm6
   86ee1:	psrld  $0x10,%xmm6
   86ee6:	pslld  $0x10,%xmm6
   86eeb:	movdqa 0xc091c(%rip),%xmm8        # 147810 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x730>
   86ef4:	pandn  %xmm6,%xmm8
   86ef9:	psrld  $0x8,%xmm5
   86efe:	por    %xmm7,%xmm8
   86f03:	movdqa %xmm5,%xmm14
   86f08:	psllw  $0x8,%xmm5
   86f0d:	pand   %xmm9,%xmm5
   86f12:	por    %xmm5,%xmm8
   86f17:	movdqa 0xc0901(%rip),%xmm5        # 147820 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x740>
   86f1f:	pand   %xmm5,%xmm8
   86f24:	psrld  $0x18,%xmm4
   86f29:	pslld  $0x18,%xmm4
   86f2e:	pandn  %xmm4,%xmm5
   86f32:	por    %xmm8,%xmm5
   86f37:	movdqa 0xc08f1(%rip),%xmm6        # 147830 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x750>
   86f3f:	pand   %xmm6,%xmm5
   86f43:	pshufd $0x50,%xmm1,%xmm4
   86f48:	pandn  %xmm4,%xmm6
   86f4c:	por    %xmm5,%xmm6
   86f50:	pand   %xmm10,%xmm6
   86f55:	movdqa %xmm1,%xmm4
   86f59:	psrld  $0x10,%xmm4
   86f5e:	psllq  $0x30,%xmm4
   86f63:	movdqa %xmm10,%xmm5
   86f68:	pandn  %xmm4,%xmm5
   86f6c:	por    %xmm6,%xmm5
   86f70:	pand   %xmm11,%xmm5
   86f75:	psrld  $0x18,%xmm1
   86f7a:	psllq  $0x38,%xmm1
   86f7f:	movdqa %xmm11,%xmm4
   86f84:	pandn  %xmm1,%xmm4
   86f88:	por    %xmm5,%xmm4
   86f8c:	pand   %xmm12,%xmm4
   86f91:	movd   %ebx,%xmm1
   86f95:	pshufd $0x44,%xmm1,%xmm5
   86f9a:	movdqa %xmm12,%xmm6
   86f9f:	pandn  %xmm5,%xmm6
   86fa3:	por    %xmm4,%xmm6
   86fa7:	pand   %xmm13,%xmm6
   86fac:	movdqa %xmm1,%xmm4
   86fb0:	psrld  $0x10,%xmm4
   86fb5:	pslldq $0xa,%xmm4
   86fba:	movdqa %xmm13,%xmm5
   86fbf:	pandn  %xmm4,%xmm5
   86fc3:	por    %xmm6,%xmm5
   86fc7:	pand   0xc08b1(%rip),%xmm5        # 147880 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x7a0>
   86fcf:	psrld  $0x18,%xmm1
   86fd4:	pslldq $0xb,%xmm1
   86fd9:	por    %xmm5,%xmm1
   86fdd:	movd   %edi,%xmm7
   86fe1:	pshufd $0x0,%xmm7,%xmm4
   86fe6:	movdqa %xmm15,%xmm5
   86feb:	pandn  %xmm4,%xmm5
   86fef:	pand   %xmm15,%xmm1
   86ff4:	por    %xmm1,%xmm5
   86ff8:	pand   0xc08a0(%rip),%xmm5        # 1478a0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x7c0>
   87000:	movdqa %xmm7,%xmm1
   87004:	psrld  $0x10,%xmm1
   87009:	pslldq $0xe,%xmm1
   8700e:	por    %xmm5,%xmm1
   87012:	pand   0xc0896(%rip),%xmm1        # 1478b0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x7d0>
   8701a:	psrld  $0x18,%xmm7
   8701f:	pslldq $0xf,%xmm7
   87024:	por    %xmm1,%xmm7
   87028:	pand   0xc039f(%rip),%xmm14        # 1473d0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x2f0>
   87031:	packuswb %xmm14,%xmm14
   87036:	packuswb %xmm14,%xmm14
   8703b:	punpcklbw %xmm14,%xmm14
   87040:	pxor   %xmm1,%xmm1
   87044:	punpcklbw %xmm1,%xmm14
   87049:	paddb  %xmm7,%xmm14
   8704e:	movdqa %xmm14,0x10(%rsp)
   87055:	movzbl 0x10(%rsp),%eax
   8705a:	movzbl 0x11(%rsp),%esi
   8705f:	movzbl 0x12(%rsp),%r14d
   87065:	movzbl 0x13(%rsp),%r13d
   8706b:	movzbl 0x14(%rsp),%r11d
   87071:	movzbl 0x15(%rsp),%r12d
   87077:	movzbl 0x16(%rsp),%edx
   8707c:	movzbl 0x17(%rsp),%r15d
   87082:	movzbl 0x18(%rsp),%edi
   87087:	mov    %dil,0xe(%rsp)
   8708c:	movzbl 0x19(%rsp),%edi
   87091:	mov    %edi,0x20(%rsp)
   87095:	movzbl 0x1a(%rsp),%r8d
   8709b:	movzbl 0x1b(%rsp),%ebx
   870a0:	movzbl 0x1c(%rsp),%edi
   870a5:	mov    %dil,0xf(%rsp)
   870aa:	movzbl 0x1d(%rsp),%ebp
   870af:	movzbl 0x1e(%rsp),%r10d
   870b5:	movzbl 0x1f(%rsp),%edi
   870ba:	jmp    872b3 <verified_tiff::webp_spans::colors+0x5b3>
   870bf:	nop
   870c0:	cmp    $0x2,%r14
   870c4:	jne    87100 <verified_tiff::webp_spans::colors+0x400>
   870c6:	cmp    %r8,%r9
   870c9:	jae    873cb <verified_tiff::webp_spans::colors+0x6cb>
   870cf:	movzbl 0x1(%rcx,%r9,4),%edx
   870d5:	movzbl 0x3(%rcx,%r9,4),%eax
   870db:	movzbl (%rcx,%r9,4),%esi
   870e0:	add    %dl,%sil
   870e3:	movzbl 0x2(%rcx,%r9,4),%edi
   870e9:	add    %dl,%dil
   870ec:	jmp    86e20 <verified_tiff::webp_spans::colors+0x120>
   870f1:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   87100:	cmp    %r8,%r9
   87103:	jae    873b8 <verified_tiff::webp_spans::colors+0x6b8>
   87109:	mov    (%rcx,%r9,4),%eax
   8710d:	mov    %eax,%edx
   8710f:	shr    $0x8,%edx
   87112:	mov    %edx,%esi
   87114:	and    $0x7f,%esi
   87117:	mov    %edx,%r11d
   8711a:	or     $0xffffff00,%r11d
   87121:	test   %ax,%ax
   87124:	cmovns %esi,%r11d
   87128:	mov    %r11d,%esi
   8712b:	imul   0x28(%rsp),%esi
   87130:	shr    $0x5,%esi
   87133:	add    %eax,%esi
   87135:	mov    %esi,%r10d
   87138:	and    $0x7f,%r10d
   8713c:	mov    %esi,%edi
   8713e:	or     $0x1f00,%edi
   87144:	test   %sil,%sil
   87147:	cmovns %r10d,%edi
   8714b:	imul   0x24(%rsp),%r11d
   87151:	shr    $0x5,%r11d
   87155:	mov    %eax,%r10d
   87158:	shr    $0x10,%r10d
   8715c:	add    %r11d,%r10d
   8715f:	imul   0x2c(%rsp),%edi
   87164:	shr    $0x5,%edi
   87167:	add    %r10d,%edi
   8716a:	shr    $0x18,%eax
   8716d:	jmp    86e20 <verified_tiff::webp_spans::colors+0x120>
   87172:	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   87180:	movd   %r13d,%xmm4
   87185:	mov    %r13d,%esi
   87188:	shr    $0x8,%esi
   8718b:	mov    %r15d,%r12d
   8718e:	shr    $0x8,%r12d
   87192:	mov    %ebx,%eax
   87194:	shr    $0x8,%eax
   87197:	mov    %eax,0x20(%rsp)
   8719b:	mov    %edi,%ebp
   8719d:	shr    $0x8,%ebp
   871a0:	movsd  0x8(%rcx,%r9,4),%xmm6
   871a7:	movd   %r13d,%xmm1
   871ac:	movd   %r15d,%xmm5
   871b1:	punpckldq %xmm5,%xmm1
   871b5:	punpcklqdq %xmm4,%xmm5
   871b9:	shufps $0x42,%xmm6,%xmm5
   871bd:	psrld  $0x10,%xmm5
   871c2:	mov    %ebp,%eax
   871c4:	shl    $0x18,%eax
   871c7:	mov    %ebx,%edx
   871c9:	shl    $0x8,%edx
   871cc:	and    $0xff0000,%edx
   871d2:	or     %eax,%edx
   871d4:	mov    %r15d,%eax
   871d7:	and    $0xff00,%eax
   871dc:	or     %edx,%eax
   871de:	movzbl %sil,%edx
   871e2:	or     %eax,%edx
   871e4:	movd   %edx,%xmm4
   871e8:	xorps  %xmm6,%xmm6
   871eb:	pcmpgtb %xmm4,%xmm6
   871ef:	punpcklbw %xmm6,%xmm4
   871f3:	movdqa %xmm4,%xmm6
   871f7:	pmullw %xmm0,%xmm6
   871fb:	psraw  $0x5,%xmm6
   87200:	punpcklwd %xmm6,%xmm6
   87204:	punpcklqdq %xmm14,%xmm1
   87209:	paddd  %xmm6,%xmm1
   8720d:	pshuflw $0xe8,%xmm1,%xmm1
   87212:	pshufhw $0xe8,%xmm1,%xmm1
   87217:	pshufd $0xe8,%xmm1,%xmm1
   8721c:	movdqa %xmm1,0x30(%rsp)
   87222:	pand   0xbfca6(%rip),%xmm1        # 146ed0 <identity_bits+0x470>
   8722a:	packuswb %xmm1,%xmm1
   8722e:	movzbl 0x30(%rsp),%eax
   87233:	movzbl 0x32(%rsp),%r11d
   87239:	movzbl 0x34(%rsp),%edx
   8723e:	mov    %dl,0xe(%rsp)
   87242:	movzbl 0x36(%rsp),%edx
   87247:	mov    %dl,0xf(%rsp)
   8724b:	pmullw %xmm2,%xmm4
   8724f:	psraw  $0x5,%xmm4
   87254:	punpcklwd %xmm4,%xmm4
   87258:	paddd  %xmm5,%xmm4
   8725c:	pxor   %xmm5,%xmm5
   87260:	pcmpgtb %xmm1,%xmm5
   87264:	punpcklbw %xmm5,%xmm1
   87268:	pmullw %xmm3,%xmm1
   8726c:	psrlw  $0x5,%xmm1
   87271:	punpcklwd %xmm1,%xmm1
   87275:	paddd  %xmm4,%xmm1
   87279:	pshuflw $0xe8,%xmm1,%xmm1
   8727e:	pshufhw $0xe8,%xmm1,%xmm1
   87283:	pshufd $0xe8,%xmm1,%xmm1
   87288:	movdqa %xmm1,0x40(%rsp)
   8728e:	movzbl 0x40(%rsp),%r14d
   87294:	movzbl 0x42(%rsp),%edx
   87299:	movzbl 0x44(%rsp),%r8d
   8729f:	movzbl 0x46(%rsp),%r10d
   872a5:	shr    $0x18,%r13d
   872a9:	shr    $0x18,%r15d
   872ad:	shr    $0x18,%ebx
   872b0:	shr    $0x18,%edi
   872b3:	movzbl %r13b,%r13d
   872b7:	shl    $0x18,%r13d
   872bb:	movzbl %r14b,%r14d
   872bf:	shl    $0x10,%r14d
   872c3:	or     %r13d,%r14d
   872c6:	movzbl %sil,%esi
   872ca:	shl    $0x8,%esi
   872cd:	or     %r14d,%esi
   872d0:	movzbl %al,%eax
   872d3:	or     %esi,%eax
   872d5:	mov    %eax,(%rcx,%r9,4)
   872d9:	movzbl %r15b,%eax
   872dd:	shl    $0x18,%eax
   872e0:	movzbl %dl,%edx
   872e3:	shl    $0x10,%edx
   872e6:	or     %eax,%edx
   872e8:	movzbl %r12b,%eax
   872ec:	shl    $0x8,%eax
   872ef:	or     %edx,%eax
   872f1:	movzbl %r11b,%edx
   872f5:	or     %eax,%edx
   872f7:	mov    %edx,0x4(%rcx,%r9,4)
   872fc:	movzbl %bl,%eax
   872ff:	shl    $0x18,%eax
   87302:	movzbl %r8b,%edx
   87306:	shl    $0x10,%edx
   87309:	or     %eax,%edx
   8730b:	movzbl 0x20(%rsp),%eax
   87310:	shl    $0x8,%eax
   87313:	or     %edx,%eax
   87315:	movzbl 0xe(%rsp),%edx
   8731a:	or     %eax,%edx
   8731c:	mov    %edx,0x8(%rcx,%r9,4)
   87321:	movzbl %dil,%eax
   87325:	shl    $0x18,%eax
   87328:	movzbl %r10b,%edx
   8732c:	shl    $0x10,%edx
   8732f:	or     %eax,%edx
   87331:	movzbl %bpl,%eax
   87335:	shl    $0x8,%eax
   87338:	or     %edx,%eax
   8733a:	movzbl 0xf(%rsp),%edx
   8733f:	or     %eax,%edx
   87341:	mov    %edx,0xc(%rcx,%r9,4)
   87346:	mov    $0x4,%eax
   8734b:	mov    0x60(%rsp),%r8
   87350:	mov    0x58(%rsp),%rbx
   87355:	mov    0x50(%rsp),%r14
   8735a:	add    %rax,%r9
   8735d:	cmp    %rbx,%r9
   87360:	jb     86e44 <verified_tiff::webp_spans::colors+0x144>
   87366:	add    $0x68,%rsp
   8736a:	pop    %rbx
   8736b:	pop    %r12
   8736d:	pop    %r13
   8736f:	pop    %r14
   87371:	pop    %r15
   87373:	pop    %rbp
   87374:	ret
   87375:	lea    0x10f1e4(%rip),%rdx        # 196560 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x198>
   8737c:	mov    %r8,%rsi
   8737f:	call   *0x11732b(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   87385:	lea    0x10f1a4(%rip),%rdx        # 196530 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x168>
   8738c:	mov    %r8,%rsi
   8738f:	call   *0x11731b(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   87395:	lea    0x10f1ac(%rip),%rdx        # 196548 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x180>
   8739c:	mov    %r8,%rsi
   8739f:	call   *0x11730b(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   873a5:	lea    0x10f16c(%rip),%rdx        # 196518 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x150>
   873ac:	mov    %r9,%rdi
   873af:	mov    %r8,%rsi
   873b2:	call   *0x1172f8(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   873b8:	lea    0x10f141(%rip),%rdx        # 196500 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x138>
   873bf:	mov    %r9,%rdi
   873c2:	mov    %r8,%rsi
   873c5:	call   *0x1172e5(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   873cb:	lea    0x10f116(%rip),%rdx        # 1964e8 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x120>
   873d2:	mov    %r9,%rdi
   873d5:	mov    %r8,%rsi
   873d8:	call   *0x1172d2(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   873de:	xchg   %ax,%ax


00000000000874d0 <verified_tiff::webp_spans::predict>:
   874d0:	push   %rbp
   874d1:	push   %r15
   874d3:	push   %r14
   874d5:	push   %r13
   874d7:	push   %r12
   874d9:	push   %rbx
   874da:	sub    $0x58,%rsp
   874de:	mov    %rdi,%rax
   874e1:	lea    -0x1(%r9),%rdi
   874e5:	cmp    %r8,%rdi
   874e8:	jae    87d0b <verified_tiff::webp_spans::predict+0x83b>
   874ee:	mov    0x10(%rax),%rsi
   874f2:	mov    %rsi,%rdi
   874f5:	not    %rdi
   874f8:	add    %r9,%rdi
   874fb:	cmp    %r8,%rdi
   874fe:	jae    87d1b <verified_tiff::webp_spans::predict+0x84b>
   87504:	mov    0x90(%rsp),%r11
   8750c:	lea    (%r11,%r9,1),%rax
   87510:	cmp    %rax,%r9
   87513:	jae    875d4 <verified_tiff::webp_spans::predict+0x104>
   87519:	mov    %rcx,%rdx
   8751c:	mov    0xa8(%rsp),%eax
   87523:	shr    $0x8,%eax
   87526:	and    $0xf,%eax
   87529:	mov    (%rcx,%rdi,4),%r15d
   8752d:	mov    -0x4(%rcx,%r9,4),%r12d
   87532:	cmp    $0xc,%eax
   87535:	mov    %r8,0x20(%rsp)
   8753a:	jne    875e3 <verified_tiff::webp_spans::predict+0x113>
   87540:	lea    (%rdx,%r9,4),%rax
   87544:	mov    %r8,%rcx
   87547:	sub    %r9,%rcx
   8754a:	sub    %rsi,%r9
   8754d:	shl    $0x2,%rsi
   87551:	mov    %rax,%rdx
   87554:	sub    %rsi,%rdx
   87557:	xor    %esi,%esi
   87559:	pxor   %xmm0,%xmm0
   8755d:	movdqa 0xbf96b(%rip),%xmm1        # 146ed0 <identity_bits+0x470>
   87565:	data16 cs nopw 0x0(%rax,%rax,1)
   87570:	lea    (%r9,%rsi,1),%rdi
   87574:	cmp    %r8,%rdi
   87577:	jae    87cc3 <verified_tiff::webp_spans::predict+0x7f3>
   8757d:	cmp    %rsi,%rcx
   87580:	je     87cd5 <verified_tiff::webp_spans::predict+0x805>
   87586:	movd   %r15d,%xmm2
   8758b:	mov    (%rdx,%rsi,4),%r15d
   8758f:	movd   %r12d,%xmm3
   87594:	punpcklbw %xmm0,%xmm3
   87598:	movd   (%rdx,%rsi,4),%xmm4
   8759d:	punpcklbw %xmm0,%xmm4
   875a1:	punpcklbw %xmm0,%xmm2
   875a5:	psubw  %xmm2,%xmm3
   875a9:	paddw  %xmm4,%xmm3
   875ad:	pmaxsw %xmm0,%xmm3
   875b1:	pminsw %xmm1,%xmm3
   875b5:	packuswb %xmm3,%xmm3
   875b9:	movd   (%rax,%rsi,4),%xmm2
   875be:	paddb  %xmm3,%xmm2
   875c2:	movd   %xmm2,%r12d
   875c7:	movd   %xmm2,(%rax,%rsi,4)
   875cc:	inc    %rsi
   875cf:	cmp    %rsi,%r11
   875d2:	jne    87570 <verified_tiff::webp_spans::predict+0xa0>
   875d4:	add    $0x58,%rsp
   875d8:	pop    %rbx
   875d9:	pop    %r12
   875db:	pop    %r13
   875dd:	pop    %r14
   875df:	pop    %r15
   875e1:	pop    %rbp
   875e2:	ret
   875e3:	lea    (%rdx,%r9,4),%r14
   875e7:	mov    %r8,%rdx
   875ea:	sub    %r9,%rdx
   875ed:	mov    %rdx,0x50(%rsp)
   875f2:	sub    %rsi,%r9
   875f5:	shl    $0x2,%rsi
   875f9:	mov    %r14,0x48(%rsp)
   875fe:	sub    %rsi,%r14
   87601:	add    $0x4,%r14
   87605:	xor    %ebx,%ebx
   87607:	mov    %eax,%ecx
   87609:	mov    %rcx,0x40(%rsp)
   8760e:	movq   0xc02aa(%rip),%xmm0        # 1478c0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x7e0>
   87616:	movd   0xc0c36(%rip),%xmm1        # 148254 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x1174>
   8761e:	movdqa 0xc02ba(%rip),%xmm2        # 1478e0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x800>
   87626:	pxor   %xmm3,%xmm3
   8762a:	mov    %r14,0x18(%rsp)
   8762f:	mov    %r9,0x30(%rsp)
   87634:	mov    %eax,0x3c(%rsp)
   87638:	jmp    876f8 <verified_tiff::webp_spans::predict+0x228>
   8763d:	movzbl %r12b,%eax
   87641:	mov    0x4(%rsp),%r15d
   87646:	movzbl %r15b,%r12d
   8764a:	add    %eax,%r12d
   8764d:	shr    $1,%r12d
   87650:	movzbl 0x14(%rsp),%eax
   87655:	movzbl %bpl,%edi
   87659:	add    %eax,%edi
   8765b:	shr    $1,%edi
   8765d:	movzbl %dl,%eax
   87660:	movzbl 0x8(%rsp),%edx
   87665:	add    %eax,%edx
   87667:	shr    $1,%edx
   87669:	mov    0xc(%rsp),%r13d
   8766e:	add    0x10(%rsp),%r13d
   87673:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   87680:	shr    $1,%r13d
   87683:	mov    0x20(%rsp),%r8
   87688:	mov    0x30(%rsp),%r9
   8768d:	mov    0x90(%rsp),%r11
   87695:	mov    0x18(%rsp),%r14
   8769a:	mov    0x48(%rsp),%rsi
   8769f:	mov    (%rsi,%rbx,4),%eax
   876a2:	movzbl %dl,%ecx
   876a5:	shl    $0x10,%ecx
   876a8:	mov    %eax,%edx
   876aa:	and    $0xff0000,%edx
   876b0:	add    %ecx,%edx
   876b2:	and    $0xff0000,%edx
   876b8:	shl    $0x18,%r13d
   876bc:	mov    %eax,%ecx
   876be:	and    $0xff000000,%ecx
   876c4:	add    %r13d,%ecx
   876c7:	or     %edx,%ecx
   876c9:	movzbl %dil,%edx
   876cd:	shl    $0x8,%edx
   876d0:	add    %eax,%edx
   876d2:	and    $0xff00,%edx
   876d8:	or     %ecx,%edx
   876da:	add    %r12b,%al
   876dd:	movzbl %al,%r12d
   876e1:	or     %edx,%r12d
   876e4:	mov    %r12d,(%rsi,%rbx,4)
   876e8:	inc    %rbx
   876eb:	cmp    %rbx,%r11
   876ee:	mov    0x3c(%rsp),%eax
   876f2:	je     875d4 <verified_tiff::webp_spans::predict+0x104>
   876f8:	lea    (%r9,%rbx,1),%rdi
   876fc:	cmp    %r8,%rdi
   876ff:	jae    87cc3 <verified_tiff::webp_spans::predict+0x7f3>
   87705:	cmp    %rbx,0x50(%rsp)
   8770a:	je     87ce8 <verified_tiff::webp_spans::predict+0x818>
   87710:	mov    %r9,%r10
   87713:	lea    (%r9,%rbx,1),%rdi
   87717:	inc    %rdi
   8771a:	mov    %r8,%r9
   8771d:	cmp    %r8,%rdi
   87720:	jae    87cfb <verified_tiff::webp_spans::predict+0x82b>
   87726:	mov    %r15d,%ecx
   87729:	mov    -0x4(%r14,%rbx,4),%r15d
   8772e:	mov    %r12d,%edx
   87731:	shr    $0x8,%edx
   87734:	mov    %edx,0x14(%rsp)
   87738:	mov    %r12d,%edx
   8773b:	shr    $0x10,%edx
   8773e:	mov    %r12d,%esi
   87741:	shr    $0x18,%esi
   87744:	mov    %esi,0x10(%rsp)
   87748:	mov    %r15d,%ebp
   8774b:	shr    $0x8,%ebp
   8774e:	mov    %r15d,%esi
   87751:	shr    $0x10,%esi
   87754:	mov    %esi,0x8(%rsp)
   87758:	mov    %r15d,%esi
   8775b:	shr    $0x18,%esi
   8775e:	mov    %esi,0xc(%rsp)
   87762:	mov    %ecx,%edi
   87764:	shr    $0x8,%edi
   87767:	mov    %ecx,%esi
   87769:	shr    $0x10,%esi
   8776c:	mov    %ecx,%r13d
   8776f:	shr    $0x18,%r13d
   87773:	mov    %r13d,0x2c(%rsp)
   87778:	mov    %r14,%r8
   8777b:	xor    %r14d,%r14d
   8777e:	cmp    $0xc,%eax
   87781:	ja     87a63 <verified_tiff::webp_spans::predict+0x593>
   87787:	mov    %r15d,0x4(%rsp)
   8778c:	mov    (%r8,%rbx,4),%r11d
   87790:	mov    %r11d,%r9d
   87793:	shr    $0x8,%r9d
   87797:	mov    %r11d,%r15d
   8779a:	shr    $0x10,%r15d
   8779e:	mov    %r11d,%r13d
   877a1:	shr    $0x18,%r13d
   877a5:	mov    0x40(%rsp),%rax
   877aa:	lea    0xc5943(%rip),%r8        # 14d0f4 <anon.4f3d76123c243ca47d579c4aabd055e1.0.llvm.6207170786339603499+0x1b46>
   877b1:	movslq (%r8,%rax,4),%rax
   877b5:	add    %r8,%rax
   877b8:	jmp    *%rax
   877ba:	mov    $0xff,%r13d
   877c0:	xor    %r12d,%r12d
   877c3:	xor    %edi,%edi
   877c5:	xor    %edx,%edx
   877c7:	jmp    87bcb <verified_tiff::webp_spans::predict+0x6fb>
   877cc:	movzbl 0x4(%rsp),%r14d
   877d2:	movzbl %cl,%ecx
   877d5:	mov    %r14d,%eax
   877d8:	sub    %ecx,%eax
   877da:	mov    %ecx,%r9d
   877dd:	sub    %r14d,%r9d
   877e0:	cmovb  %eax,%r9d
   877e4:	movzbl %r9b,%eax
   877e8:	movzbl %bpl,%r10d
   877ec:	movzbl %dil,%r11d
   877f0:	mov    %r10d,%edi
   877f3:	sub    %r11d,%edi
   877f6:	mov    %r11d,%r9d
   877f9:	sub    %r10d,%r9d
   877fc:	cmovb  %edi,%r9d
   87800:	movzbl %r9b,%r9d
   87804:	add    %rax,%r9
   87807:	movzbl 0x8(%rsp),%ebp
   8780c:	movzbl %sil,%esi
   87810:	mov    %ebp,%edi
   87812:	sub    %esi,%edi
   87814:	mov    %esi,%r8d
   87817:	sub    %ebp,%r8d
   8781a:	cmovb  %edi,%r8d
   8781e:	mov    0x2c(%rsp),%eax
   87822:	mov    %eax,%edi
   87824:	mov    0xc(%rsp),%r15d
   87829:	sub    %r15b,%dil
   8782c:	mov    %r15d,%r13d
   8782f:	sub    %al,%r13b
   87832:	cmp    %eax,%r15d
   87835:	movzbl %dil,%edi
   87839:	movzbl %r13b,%r13d
   8783d:	cmovb  %edi,%r13d
   87841:	movzbl %r8b,%r8d
   87845:	movzbl %r13b,%edi
   87849:	add    %r8,%rdi
   8784c:	movzbl %r12b,%r12d
   87850:	add    %r9,%rdi
   87853:	mov    %r12d,%r8d
   87856:	sub    %ecx,%r8d
   87859:	sub    %r12d,%ecx
   8785c:	cmovb  %r8d,%ecx
   87860:	movzbl 0x14(%rsp),%r9d
   87866:	mov    %r9d,%r8d
   87869:	sub    %r11d,%r8d
   8786c:	sub    %r9d,%r11d
   8786f:	cmovb  %r8d,%r11d
   87873:	movzbl %dl,%edx
   87876:	movzbl %cl,%ecx
   87879:	movzbl %r11b,%r8d
   8787d:	add    %rcx,%r8
   87880:	mov    %edx,%ecx
   87882:	sub    %esi,%ecx
   87884:	sub    %edx,%esi
   87886:	cmovb  %ecx,%esi
   87889:	mov    %eax,%ecx
   8788b:	mov    0x10(%rsp),%r13d
   87890:	sub    %r13b,%cl
   87893:	mov    %r13d,%r11d
   87896:	sub    %al,%r11b
   87899:	cmp    %eax,%r13d
   8789c:	movzbl %cl,%ecx
   8789f:	movzbl %r11b,%r11d
   878a3:	cmovb  %ecx,%r11d
   878a7:	movzbl %sil,%ecx
   878ab:	movzbl %r11b,%esi
   878af:	add    %rcx,%rsi
   878b2:	add    %r8,%rsi
   878b5:	cmp    %rsi,%rdi
   878b8:	cmovb  %r12d,%r14d
   878bc:	cmovb  %r9d,%r10d
   878c0:	cmovb  %edx,%ebp
   878c3:	cmp    %esi,%edi
   878c5:	cmovae %r15d,%r13d
   878c9:	mov    %r14d,%r12d
   878cc:	mov    0x4(%rsp),%r15d
   878d1:	mov    %r10d,%edi
   878d4:	mov    %ebp,%edx
   878d6:	jmp    87683 <verified_tiff::webp_spans::predict+0x1b3>
   878db:	mov    %ecx,%r12d
   878de:	mov    %esi,%edx
   878e0:	mov    0x2c(%rsp),%r13d
   878e5:	jmp    87bcb <verified_tiff::webp_spans::predict+0x6fb>
   878ea:	movzbl %r12b,%r12d
   878ee:	mov    0x4(%rsp),%r15d
   878f3:	movzbl %r15b,%eax
   878f7:	movzbl %cl,%ecx
   878fa:	sub    %ecx,%r12d
   878fd:	add    %eax,%r12d
   87900:	xor    %ecx,%ecx
   87902:	test   %r12d,%r12d
   87905:	cmovle %ecx,%r12d
   87909:	cmp    $0xff,%r12d
   87910:	mov    $0xff,%r8d
   87916:	cmovge %r8d,%r12d
   8791a:	mov    0x14(%rsp),%r9d
   8791f:	movzbl %r9b,%r9d
   87923:	movzbl %bpl,%eax
   87927:	movzbl %dil,%edi
   8792b:	sub    %edi,%r9d
   8792e:	mov    %r9d,%edi
   87931:	add    %eax,%edi
   87933:	test   %edi,%edi
   87935:	cmovle %ecx,%edi
   87938:	cmp    $0xff,%edi
   8793e:	cmovge %r8d,%edi
   87942:	movzbl %dl,%edx
   87945:	movzbl 0x8(%rsp),%eax
   8794a:	movzbl %sil,%esi
   8794e:	sub    %esi,%edx
   87950:	add    %eax,%edx
   87952:	test   %edx,%edx
   87954:	cmovle %ecx,%edx
   87957:	cmp    $0xff,%edx
   8795d:	cmovge %r8d,%edx
   87961:	mov    0x10(%rsp),%r13d
   87966:	sub    0x2c(%rsp),%r13d
   8796b:	add    0xc(%rsp),%r13d
   87970:	js     87cbb <verified_tiff::webp_spans::predict+0x7eb>
   87976:	cmp    $0xff,%r13d
   8797d:	cmovae %r8d,%r13d
   87981:	jmp    87683 <verified_tiff::webp_spans::predict+0x1b3>
   87986:	mov    0x4(%rsp),%r8d
   8798b:	movzbl %r8b,%eax
   8798f:	movzbl %r11b,%r12d
   87993:	add    %eax,%r12d
   87996:	shr    $1,%r12d
   87999:	movzbl %bpl,%eax
   8799d:	movzbl %r9b,%edi
   879a1:	add    %eax,%edi
   879a3:	shr    $1,%edi
   879a5:	movzbl 0x8(%rsp),%eax
   879aa:	movzbl %r15b,%edx
   879ae:	mov    %r8d,%r15d
   879b1:	add    %eax,%edx
   879b3:	shr    $1,%edx
   879b5:	jmp    87a59 <verified_tiff::webp_spans::predict+0x589>
   879ba:	mov    0x4(%rsp),%r15d
   879bf:	mov    %r15d,%r12d
   879c2:	mov    %ebp,%edi
   879c4:	mov    0x8(%rsp),%edx
   879c8:	mov    0xc(%rsp),%r13d
   879cd:	jmp    87683 <verified_tiff::webp_spans::predict+0x1b3>
   879d2:	mov    %r11d,%r12d
   879d5:	mov    %r9d,%edi
   879d8:	mov    %r15d,%edx
   879db:	jmp    87bcb <verified_tiff::webp_spans::predict+0x6fb>
   879e0:	mov    0x20(%rsp),%r8
   879e5:	mov    0x30(%rsp),%r9
   879ea:	mov    0x90(%rsp),%r11
   879f2:	mov    0x4(%rsp),%r15d
   879f7:	mov    0x18(%rsp),%r14
   879fc:	mov    0x14(%rsp),%edi
   87a00:	mov    0x10(%rsp),%r13d
   87a05:	jmp    8769a <verified_tiff::webp_spans::predict+0x1ca>
   87a0a:	movzbl %r12b,%eax
   87a0e:	movzbl %r11b,%ecx
   87a12:	add    %eax,%ecx
   87a14:	shr    $1,%ecx
   87a16:	mov    0x4(%rsp),%r8d
   87a1b:	movzbl %r8b,%r12d
   87a1f:	add    %ecx,%r12d
   87a22:	shr    $1,%r12d
   87a25:	movzbl 0x14(%rsp),%eax
   87a2a:	movzbl %r9b,%ecx
   87a2e:	add    %eax,%ecx
   87a30:	shr    $1,%ecx
   87a32:	movzbl %bpl,%edi
   87a36:	add    %ecx,%edi
   87a38:	shr    $1,%edi
   87a3a:	movzbl %dl,%eax
   87a3d:	movzbl %r15b,%ecx
   87a41:	mov    %r8d,%r15d
   87a44:	add    %eax,%ecx
   87a46:	shr    $1,%ecx
   87a48:	movzbl 0x8(%rsp),%edx
   87a4d:	add    %ecx,%edx
   87a4f:	shr    $1,%edx
   87a51:	add    0x10(%rsp),%r13d
   87a56:	shr    $1,%r13d
   87a59:	add    0xc(%rsp),%r13d
   87a5e:	jmp    87680 <verified_tiff::webp_spans::predict+0x1b0>
   87a63:	movd   %ecx,%xmm5
   87a67:	movd   %edi,%xmm4
   87a6b:	punpckldq %xmm5,%xmm4
   87a6f:	pand   %xmm0,%xmm4
   87a73:	movzbl %sil,%ecx
   87a77:	movzbl %dl,%edx
   87a7a:	movzbl 0x8(%rsp),%eax
   87a7f:	add    %edx,%eax
   87a81:	shr    $1,%eax
   87a83:	mov    0xc(%rsp),%edi
   87a87:	add    0x10(%rsp),%edi
   87a8b:	shr    $1,%edi
   87a8d:	shl    $0x8,%r12d
   87a91:	movzbl 0x14(%rsp),%edx
   87a96:	or     %r12d,%edx
   87a99:	movd   %edx,%xmm5
   87a9d:	pand   %xmm1,%xmm5
   87aa1:	mov    %r15d,%edx
   87aa4:	shl    $0x8,%edx
   87aa7:	movzbl %bpl,%esi
   87aab:	or     %edx,%esi
   87aad:	movd   %esi,%xmm6
   87ab1:	pand   %xmm1,%xmm6
   87ab5:	movdqa %xmm6,%xmm7
   87ab9:	pand   %xmm5,%xmm7
   87abd:	pxor   %xmm5,%xmm6
   87ac1:	psrlw  $0x1,%xmm6
   87ac6:	pand   %xmm2,%xmm6
   87aca:	paddb  %xmm7,%xmm6
   87ace:	punpcklbw %xmm3,%xmm6
   87ad2:	punpcklwd %xmm3,%xmm6
   87ad6:	movd   %edi,%xmm5
   87ada:	pinsrw $0x2,%eax,%xmm5
   87adf:	punpcklqdq %xmm6,%xmm5
   87ae3:	psubd  %xmm4,%xmm6
   87ae7:	sub    %ecx,%eax
   87ae9:	mov    %edi,%ecx
   87aeb:	sub    0x2c(%rsp),%ecx
   87aef:	movd   %ecx,%xmm4
   87af3:	movd   %eax,%xmm7
   87af7:	punpckldq %xmm7,%xmm4
   87afb:	punpcklqdq %xmm6,%xmm4
   87aff:	pxor   %xmm6,%xmm6
   87b03:	pcmpgtd %xmm4,%xmm6
   87b07:	pxor   %xmm7,%xmm7
   87b0b:	psubd  %xmm4,%xmm7
   87b0f:	psrld  $0x1,%xmm7
   87b14:	pxor   %xmm8,%xmm8
   87b19:	psubd  %xmm7,%xmm8
   87b1e:	psrld  $0x1,%xmm4
   87b23:	pand   %xmm6,%xmm8
   87b28:	pandn  %xmm4,%xmm6
   87b2c:	por    %xmm8,%xmm6
   87b31:	paddd  %xmm6,%xmm5
   87b35:	pshufd $0xff,%xmm5,%xmm4
   87b3a:	movd   %xmm4,%r12d
   87b3f:	cmp    $0xff,%r12d
   87b46:	mov    $0xff,%ecx
   87b4b:	cmovae %ecx,%r12d
   87b4f:	movmskps %xmm5,%eax
   87b52:	test   $0x8,%al
   87b54:	cmovne %r14d,%r12d
   87b58:	pshufd $0xee,%xmm5,%xmm4
   87b5d:	movd   %xmm4,%edi
   87b61:	cmp    $0xff,%edi
   87b67:	cmovae %ecx,%edi
   87b6a:	test   $0x4,%al
   87b6c:	cmovne %r14d,%edi
   87b70:	pshufd $0x55,%xmm5,%xmm4
   87b75:	movd   %xmm4,%edx
   87b79:	cmp    $0xff,%edx
   87b7f:	cmovae %ecx,%edx
   87b82:	test   $0x2,%al
   87b84:	cmovne %r14d,%edx
   87b88:	test   $0x1,%al
   87b8a:	je     87c9b <verified_tiff::webp_spans::predict+0x7cb>
   87b90:	xor    %r13d,%r13d
   87b93:	jmp    87cab <verified_tiff::webp_spans::predict+0x7db>
   87b98:	movzbl %r12b,%eax
   87b9c:	movzbl %cl,%r12d
   87ba0:	add    %eax,%r12d
   87ba3:	shr    $1,%r12d
   87ba6:	movzbl 0x14(%rsp),%eax
   87bab:	movzbl %dil,%edi
   87baf:	add    %eax,%edi
   87bb1:	shr    $1,%edi
   87bb3:	movzbl %dl,%eax
   87bb6:	movzbl %sil,%edx
   87bba:	add    %eax,%edx
   87bbc:	shr    $1,%edx
   87bbe:	mov    0x2c(%rsp),%r13d
   87bc3:	add    0x10(%rsp),%r13d
   87bc8:	shr    $1,%r13d
   87bcb:	mov    0x20(%rsp),%r8
   87bd0:	mov    0x30(%rsp),%r9
   87bd5:	mov    0x90(%rsp),%r11
   87bdd:	mov    0x4(%rsp),%r15d
   87be2:	jmp    87695 <verified_tiff::webp_spans::predict+0x1c5>
   87be7:	movzbl %r12b,%eax
   87beb:	movzbl %cl,%ecx
   87bee:	add    %eax,%ecx
   87bf0:	mov    0x4(%rsp),%r8d
   87bf5:	movzbl %r8b,%eax
   87bf9:	movzbl %r11b,%r12d
   87bfd:	add    %eax,%r12d
   87c00:	shr    $1,%ecx
   87c02:	shr    $1,%r12d
   87c05:	add    %ecx,%r12d
   87c08:	movzbl 0x14(%rsp),%eax
   87c0d:	movzbl %dil,%ecx
   87c11:	add    %eax,%ecx
   87c13:	movzbl %bpl,%eax
   87c17:	movzbl %r9b,%edi
   87c1b:	add    %eax,%edi
   87c1d:	shr    $1,%ecx
   87c1f:	shr    $1,%edi
   87c21:	add    %ecx,%edi
   87c23:	movzbl %dl,%eax
   87c26:	movzbl %sil,%ecx
   87c2a:	add    %eax,%ecx
   87c2c:	movzbl 0x8(%rsp),%eax
   87c31:	movzbl %r15b,%edx
   87c35:	mov    %r8d,%r15d
   87c38:	add    %eax,%edx
   87c3a:	shr    $1,%ecx
   87c3c:	shr    $1,%edx
   87c3e:	add    %ecx,%edx
   87c40:	mov    0x2c(%rsp),%eax
   87c44:	add    0x10(%rsp),%eax
   87c48:	add    0xc(%rsp),%r13d
   87c4d:	shr    $1,%eax
   87c4f:	shr    $1,%r13d
   87c52:	add    %eax,%r13d
   87c55:	shr    $1,%r12d
   87c58:	shr    $1,%edi
   87c5a:	shr    $1,%edx
   87c5c:	jmp    87680 <verified_tiff::webp_spans::predict+0x1b0>
   87c61:	movzbl %cl,%eax
   87c64:	mov    0x4(%rsp),%r15d
   87c69:	movzbl %r15b,%r12d
   87c6d:	add    %eax,%r12d
   87c70:	shr    $1,%r12d
   87c73:	movzbl %dil,%eax
   87c77:	movzbl %bpl,%edi
   87c7b:	add    %eax,%edi
   87c7d:	shr    $1,%edi
   87c7f:	movzbl %sil,%eax
   87c83:	movzbl 0x8(%rsp),%edx
   87c88:	add    %eax,%edx
   87c8a:	shr    $1,%edx
   87c8c:	mov    0xc(%rsp),%r13d
   87c91:	add    0x2c(%rsp),%r13d
   87c96:	jmp    87680 <verified_tiff::webp_spans::predict+0x1b0>
   87c9b:	movd   %xmm5,%r13d
   87ca0:	cmp    $0xff,%r13d
   87ca7:	cmovae %ecx,%r13d
   87cab:	mov    0x18(%rsp),%r14
   87cb0:	mov    %r9,%r8
   87cb3:	mov    %r10,%r9
   87cb6:	jmp    8769a <verified_tiff::webp_spans::predict+0x1ca>
   87cbb:	xor    %r13d,%r13d
   87cbe:	jmp    87683 <verified_tiff::webp_spans::predict+0x1b3>
   87cc3:	lea    0x10e8de(%rip),%rdx        # 1965a8 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x1e0>
   87cca:	mov    0x20(%rsp),%rsi
   87ccf:	call   *0x1169db(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   87cd5:	lea    0x10e8e4(%rip),%rdx        # 1965c0 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x1f8>
   87cdc:	mov    %r8,%rdi
   87cdf:	mov    %r8,%rsi
   87ce2:	call   *0x1169c8(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   87ce8:	lea    0x10e8e9(%rip),%rdx        # 1965d8 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x210>
   87cef:	mov    %r8,%rdi
   87cf2:	mov    %r8,%rsi
   87cf5:	call   *0x1169b5(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   87cfb:	lea    0x10e8ee(%rip),%rdx        # 1965f0 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x228>
   87d02:	mov    %r9,%rsi
   87d05:	call   *0x1169a5(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   87d0b:	lea    0x10e866(%rip),%rdx        # 196578 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x1b0>
   87d12:	mov    %r8,%rsi
   87d15:	call   *0x116995(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   87d1b:	lea    0x10e86e(%rip),%rdx        # 196590 <anon.a13c77783f134d05f2f4a548b6d73be7.12.llvm.12369979014551296565+0x1c8>
   87d22:	mov    %r8,%rsi
   87d25:	call   *0x116985(%rip)        # 19e6b0 <_GLOBAL_OFFSET_TABLE_+0xb68>
   87d2b:	nopl   0x0(%rax,%rax,1)


00000000000888e0 <verified_tiff::vp8_transform::transform>:
   888e0:	mov    %rdi,%rax
   888e3:	movdqu 0x4(%rsi),%xmm0
   888e8:	movdqu 0x14(%rsi),%xmm1
   888ed:	movdqu 0x24(%rsi),%xmm2
   888f2:	pxor   %xmm3,%xmm3
   888f6:	pcmpeqd %xmm3,%xmm1
   888fa:	packssdw %xmm1,%xmm1
   888fe:	pcmpeqd %xmm3,%xmm0
   88902:	pcmpeqd %xmm3,%xmm2
   88906:	pand   %xmm0,%xmm2
   8890a:	cmpl   $0x0,0x34(%rsi)
   8890e:	sete   %cl
   88911:	pshuflw $0xe8,%xmm2,%xmm0
   88916:	pshufhw $0xe8,%xmm0,%xmm0
   8891b:	pshufd $0x84,%xmm0,%xmm0
   88920:	punpckhqdq %xmm1,%xmm0
   88924:	psllw  $0xf,%xmm0
   88929:	packsswb %xmm0,%xmm0
   8892d:	pmovmskb %xmm0,%edi
   88931:	cmp    $0xff,%dil
   88935:	sete   %dil
   88939:	and    %cl,%dil
   8893c:	cmp    $0x1,%dil
   88940:	jne    8898d <verified_tiff::vp8_transform::transform+0xad>
   88942:	cmpl   $0x0,0x38(%rsi)
   88946:	jne    8898d <verified_tiff::vp8_transform::transform+0xad>
   88948:	cmpl   $0x0,0x3c(%rsi)
   8894c:	jne    8898d <verified_tiff::vp8_transform::transform+0xad>
   8894e:	movslq (%rsi),%rcx
   88951:	test   %dl,%dl
   88953:	je     88f7c <verified_tiff::vp8_transform::transform+0x69c>
   88959:	lea    0x40000003(%rcx),%rdx
   88960:	add    $0x4000000a,%rcx
   88967:	test   %rdx,%rdx
   8896a:	cmovns %rdx,%rcx
   8896e:	sar    $0x3,%rcx
   88972:	movzwl %cx,%edx
   88975:	cmp    $0x7fff,%rdx
   8897c:	jbe    88f9d <verified_tiff::vp8_transform::transform+0x6bd>
   88982:	or     $0xffff0000,%ecx
   88988:	jmp    88f9f <verified_tiff::vp8_transform::transform+0x6bf>
   8898d:	movdqu (%rsi),%xmm1
   88991:	movdqu 0x10(%rsi),%xmm2
   88996:	movdqu 0x20(%rsi),%xmm0
   8899b:	movdqu 0x30(%rsi),%xmm3
   889a0:	test   %dl,%dl
   889a2:	je     889e2 <verified_tiff::vp8_transform::transform+0x102>
   889a4:	movdqa %xmm2,%xmm6
   889a8:	paddd  %xmm1,%xmm6
   889ac:	movdqa %xmm3,%xmm4
   889b0:	paddd  %xmm0,%xmm4
   889b4:	psubd  %xmm2,%xmm1
   889b8:	psubd  %xmm3,%xmm0
   889bc:	movdqa %xmm4,%xmm2
   889c0:	paddd  %xmm6,%xmm2
   889c4:	psubd  %xmm4,%xmm6
   889c8:	movdqa %xmm1,%xmm3
   889cc:	psubd  %xmm0,%xmm3
   889d0:	paddd  %xmm1,%xmm0
   889d4:	movdqa %xmm3,%xmm1
   889d8:	movdqa %xmm0,%xmm8
   889dd:	jmp    88b8e <verified_tiff::vp8_transform::transform+0x2ae>
   889e2:	pxor   %xmm5,%xmm5
   889e6:	pxor   %xmm7,%xmm7
   889ea:	pcmpgtd %xmm2,%xmm7
   889ee:	movdqa 0xbeef9(%rip),%xmm8        # 1478f0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x810>
   889f7:	movdqa %xmm7,%xmm4
   889fb:	pand   %xmm8,%xmm4
   88a00:	movdqa 0xbeef8(%rip),%xmm6        # 147900 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x820>
   88a08:	movdqa %xmm2,%xmm9
   88a0d:	pmuludq %xmm6,%xmm9
   88a12:	pshufd $0xf5,%xmm4,%xmm10
   88a18:	psllq  $0x20,%xmm4
   88a1d:	psubq  %xmm4,%xmm9
   88a22:	pshufd $0xf5,%xmm2,%xmm11
   88a28:	movdqa %xmm11,%xmm12
   88a2d:	pmuludq %xmm6,%xmm12
   88a32:	psllq  $0x20,%xmm10
   88a38:	psubq  %xmm10,%xmm12
   88a3d:	psrlq  $0x10,%xmm9
   88a43:	psrlq  $0x10,%xmm12
   88a49:	pshufd $0xe8,%xmm9,%xmm4
   88a4f:	pshufd $0xe8,%xmm12,%xmm9
   88a55:	punpckldq %xmm9,%xmm4
   88a5a:	movdqa 0xbeead(%rip),%xmm10        # 147910 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x830>
   88a63:	pand   %xmm10,%xmm7
   88a68:	movdqa 0xbeeaf(%rip),%xmm9        # 147920 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x840>
   88a71:	movdqa %xmm2,%xmm12
   88a76:	pmuludq %xmm9,%xmm12
   88a7b:	pshufd $0xf5,%xmm7,%xmm13
   88a81:	psllq  $0x20,%xmm7
   88a86:	psubq  %xmm7,%xmm12
   88a8b:	pmuludq %xmm9,%xmm11
   88a90:	psllq  $0x20,%xmm13
   88a96:	psubq  %xmm13,%xmm11
   88a9b:	psrlq  $0x10,%xmm12
   88aa1:	psrlq  $0x10,%xmm11
   88aa7:	pshufd $0xe8,%xmm12,%xmm7
   88aad:	pshufd $0xe8,%xmm11,%xmm11
   88ab3:	punpckldq %xmm11,%xmm7
   88ab8:	pcmpgtd %xmm3,%xmm5
   88abc:	pand   %xmm5,%xmm8
   88ac1:	movdqa %xmm3,%xmm11
   88ac6:	pmuludq %xmm6,%xmm11
   88acb:	pshufd $0xf5,%xmm8,%xmm12
   88ad1:	psllq  $0x20,%xmm8
   88ad7:	psubq  %xmm8,%xmm11
   88adc:	pshufd $0xf5,%xmm3,%xmm8
   88ae2:	pmuludq %xmm8,%xmm6
   88ae7:	psllq  $0x20,%xmm12
   88aed:	psubq  %xmm12,%xmm6
   88af2:	psrlq  $0x10,%xmm11
   88af8:	psrlq  $0x10,%xmm6
   88afd:	pshufd $0xe8,%xmm11,%xmm11
   88b03:	pshufd $0xe8,%xmm6,%xmm6
   88b08:	punpckldq %xmm6,%xmm11
   88b0d:	pand   %xmm10,%xmm5
   88b12:	movdqa %xmm3,%xmm6
   88b16:	pmuludq %xmm9,%xmm6
   88b1b:	pshufd $0xf5,%xmm5,%xmm10
   88b21:	psllq  $0x20,%xmm5
   88b26:	psubq  %xmm5,%xmm6
   88b2a:	pmuludq %xmm9,%xmm8
   88b2f:	psllq  $0x20,%xmm10
   88b35:	psubq  %xmm10,%xmm8
   88b3a:	psrlq  $0x10,%xmm6
   88b3f:	psrlq  $0x10,%xmm8
   88b45:	pshufd $0xe8,%xmm6,%xmm5
   88b4a:	pshufd $0xe8,%xmm8,%xmm6
   88b50:	punpckldq %xmm6,%xmm5
   88b54:	paddd  %xmm3,%xmm11
   88b59:	psubd  %xmm11,%xmm7
   88b5e:	paddd  %xmm2,%xmm4
   88b62:	paddd  %xmm5,%xmm4
   88b66:	movdqa %xmm0,%xmm8
   88b6b:	paddd  %xmm1,%xmm8
   88b70:	psubd  %xmm0,%xmm1
   88b74:	movdqa %xmm4,%xmm2
   88b78:	paddd  %xmm8,%xmm2
   88b7d:	movdqa %xmm7,%xmm6
   88b81:	paddd  %xmm1,%xmm6
   88b85:	psubd  %xmm7,%xmm1
   88b89:	psubd  %xmm4,%xmm8
   88b8e:	movdqa %xmm2,%xmm5
   88b92:	punpckldq %xmm6,%xmm5
   88b96:	movdqa %xmm8,%xmm3
   88b9b:	punpcklqdq %xmm1,%xmm3
   88b9f:	movdqa %xmm5,%xmm0
   88ba3:	shufps $0x24,%xmm3,%xmm0
   88ba7:	movdqa %xmm8,%xmm3
   88bac:	shufps $0x55,%xmm1,%xmm3
   88bb0:	shufps $0x2e,%xmm3,%xmm5
   88bb4:	punpckhdq %xmm6,%xmm2
   88bb8:	movdqa %xmm8,%xmm4
   88bbd:	punpckhqdq %xmm1,%xmm4
   88bc1:	movdqa %xmm2,%xmm3
   88bc5:	shufps $0x24,%xmm4,%xmm3
   88bc9:	shufps $0xff,%xmm1,%xmm8
   88bce:	shufps $0x2e,%xmm8,%xmm2
   88bd3:	test   %dl,%dl
   88bd5:	je     88c2c <verified_tiff::vp8_transform::transform+0x34c>
   88bd7:	movdqa %xmm5,%xmm4
   88bdb:	paddd  %xmm0,%xmm4
   88bdf:	movdqa %xmm2,%xmm1
   88be3:	paddd  %xmm3,%xmm1
   88be7:	psubd  %xmm5,%xmm0
   88beb:	psubd  %xmm2,%xmm3
   88bef:	movdqa %xmm4,%xmm2
   88bf3:	paddd  %xmm1,%xmm2
   88bf7:	movdqa 0xbed31(%rip),%xmm5        # 147930 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x850>
   88bff:	paddd  %xmm5,%xmm2
   88c03:	psubd  %xmm1,%xmm4
   88c07:	paddd  %xmm5,%xmm4
   88c0b:	movdqa %xmm0,%xmm6
   88c0f:	psubd  %xmm3,%xmm6
   88c13:	paddd  %xmm5,%xmm6
   88c17:	paddd  %xmm0,%xmm3
   88c1b:	paddd  %xmm5,%xmm3
   88c1f:	movdqa %xmm3,%xmm1
   88c23:	movdqa %xmm6,%xmm0
   88c27:	jmp    88de3 <verified_tiff::vp8_transform::transform+0x503>
   88c2c:	pxor   %xmm1,%xmm1
   88c30:	pxor   %xmm7,%xmm7
   88c34:	pcmpgtd %xmm5,%xmm7
   88c38:	movdqa 0xbecaf(%rip),%xmm8        # 1478f0 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x810>
   88c41:	movdqa %xmm7,%xmm6
   88c45:	pand   %xmm8,%xmm6
   88c4a:	movdqa 0xbecae(%rip),%xmm4        # 147900 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x820>
   88c52:	movdqa %xmm5,%xmm9
   88c57:	pmuludq %xmm4,%xmm9
   88c5c:	pshufd $0xf5,%xmm6,%xmm10
   88c62:	psllq  $0x20,%xmm6
   88c67:	psubq  %xmm6,%xmm9
   88c6c:	pshufd $0xf5,%xmm5,%xmm11
   88c72:	movdqa %xmm11,%xmm12
   88c77:	pmuludq %xmm4,%xmm12
   88c7c:	psllq  $0x20,%xmm10
   88c82:	psubq  %xmm10,%xmm12
   88c87:	psrlq  $0x10,%xmm9
   88c8d:	psrlq  $0x10,%xmm12
   88c93:	pshufd $0xe8,%xmm9,%xmm6
   88c99:	pshufd $0xe8,%xmm12,%xmm9
   88c9f:	punpckldq %xmm9,%xmm6
   88ca4:	movdqa 0xbec63(%rip),%xmm10        # 147910 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x830>
   88cad:	pand   %xmm10,%xmm7
   88cb2:	movdqa 0xbec65(%rip),%xmm9        # 147920 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x840>
   88cbb:	movdqa %xmm5,%xmm12
   88cc0:	pmuludq %xmm9,%xmm12
   88cc5:	pshufd $0xf5,%xmm7,%xmm13
   88ccb:	psllq  $0x20,%xmm7
   88cd0:	psubq  %xmm7,%xmm12
   88cd5:	pmuludq %xmm9,%xmm11
   88cda:	psllq  $0x20,%xmm13
   88ce0:	psubq  %xmm13,%xmm11
   88ce5:	psrlq  $0x10,%xmm12
   88ceb:	psrlq  $0x10,%xmm11
   88cf1:	pshufd $0xe8,%xmm12,%xmm7
   88cf7:	pshufd $0xe8,%xmm11,%xmm11
   88cfd:	punpckldq %xmm11,%xmm7
   88d02:	pcmpgtd %xmm2,%xmm1
   88d06:	pand   %xmm1,%xmm8
   88d0b:	movdqa %xmm2,%xmm11
   88d10:	pmuludq %xmm4,%xmm11
   88d15:	pshufd $0xf5,%xmm8,%xmm12
   88d1b:	psllq  $0x20,%xmm8
   88d21:	psubq  %xmm8,%xmm11
   88d26:	pshufd $0xf5,%xmm2,%xmm8
   88d2c:	pmuludq %xmm8,%xmm4
   88d31:	psllq  $0x20,%xmm12
   88d37:	psubq  %xmm12,%xmm4
   88d3c:	psrlq  $0x10,%xmm11
   88d42:	psrlq  $0x10,%xmm4
   88d47:	pshufd $0xe8,%xmm11,%xmm11
   88d4d:	pshufd $0xe8,%xmm4,%xmm4
   88d52:	punpckldq %xmm4,%xmm11
   88d57:	pand   %xmm10,%xmm1
   88d5c:	movdqa %xmm2,%xmm4
   88d60:	pmuludq %xmm9,%xmm4
   88d65:	pshufd $0xf5,%xmm1,%xmm10
   88d6b:	psllq  $0x20,%xmm1
   88d70:	psubq  %xmm1,%xmm4
   88d74:	pmuludq %xmm9,%xmm8
   88d79:	psllq  $0x20,%xmm10
   88d7f:	psubq  %xmm10,%xmm8
   88d84:	psrlq  $0x10,%xmm4
   88d89:	psrlq  $0x10,%xmm8
   88d8f:	pshufd $0xe8,%xmm4,%xmm1
   88d94:	pshufd $0xe8,%xmm8,%xmm4
   88d9a:	punpckldq %xmm4,%xmm1
   88d9e:	paddd  %xmm2,%xmm11
   88da3:	psubd  %xmm11,%xmm7
   88da8:	paddd  %xmm5,%xmm6
   88dac:	paddd  %xmm1,%xmm6
   88db0:	movaps %xmm0,%xmm1
   88db3:	paddd  %xmm3,%xmm1
   88db7:	movdqa 0xbe0e1(%rip),%xmm2        # 146ea0 <identity_bits+0x440>
   88dbf:	paddd  %xmm2,%xmm1
   88dc3:	psubd  %xmm3,%xmm0
   88dc7:	paddd  %xmm2,%xmm0
   88dcb:	movdqa %xmm6,%xmm2
   88dcf:	paddd  %xmm1,%xmm2
   88dd3:	movdqa %xmm7,%xmm4
   88dd7:	paddd  %xmm0,%xmm4
   88ddb:	psubd  %xmm7,%xmm0
   88ddf:	psubd  %xmm6,%xmm1
   88de3:	psrad  $0x3,%xmm2
   88de8:	psrad  $0x3,%xmm4
   88ded:	psrad  $0x3,%xmm0
   88df2:	psrad  $0x3,%xmm1
   88df7:	movl   $0x0,0x8(%rax)
   88dfe:	movq   $0x0,(%rax)
   88e05:	test   %dl,%dl
   88e07:	je     88f1c <verified_tiff::vp8_transform::transform+0x63c>
   88e0d:	movdqa %xmm2,%xmm3
   88e11:	punpckldq %xmm4,%xmm3
   88e15:	movq   0xbeb23(%rip),%xmm5        # 147940 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x860>
   88e1d:	pand   %xmm3,%xmm5
   88e21:	pcmpgtd 0xbeb27(%rip),%xmm5        # 147950 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x870>
   88e29:	movdqa 0xbeb2f(%rip),%xmm6        # 147960 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x880>
   88e31:	pand   %xmm6,%xmm3
   88e35:	pandn  %xmm5,%xmm6
   88e39:	por    %xmm3,%xmm6
   88e3d:	movq   %xmm6,(%rax)
   88e41:	movd   %xmm0,%ecx
   88e45:	movzwl %cx,%ecx
   88e48:	xor    %edx,%edx
   88e4a:	cmp    $0x8000,%ecx
   88e50:	setb   %dl
   88e53:	shl    $0x10,%edx
   88e56:	add    %edx,%ecx
   88e58:	add    $0xffff0000,%ecx
   88e5e:	mov    %ecx,0x8(%rax)
   88e61:	pshufd $0x55,%xmm2,%xmm3
   88e66:	movdqa %xmm1,%xmm5
   88e6a:	punpckldq %xmm3,%xmm5
   88e6e:	movaps %xmm0,%xmm3
   88e71:	shufps $0x55,%xmm4,%xmm3
   88e75:	shufps $0x24,%xmm3,%xmm5
   88e79:	movdqa 0xbeaef(%rip),%xmm7        # 147970 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x890>
   88e81:	movaps 0xbeaf8(%rip),%xmm6        # 147980 <anon.a702f31897e79a7af26c50fde7144523.7.llvm.12130086535494275439+0x8a0>
   88e88:	andps  %xmm6,%xmm5
   88e8b:	movaps %xmm5,%xmm8
   88e8f:	pcmpgtd %xmm7,%xmm8
   88e94:	movaps %xmm6,%xmm3
   88e97:	andnps %xmm8,%xmm3
   88e9b:	orps   %xmm5,%xmm3
   88e9e:	pshufd $0xee,%xmm2,%xmm5
   88ea3:	pshufd $0x55,%xmm1,%xmm8
   88ea9:	punpckldq %xmm5,%xmm8
   88eae:	movaps %xmm0,%xmm5
   88eb1:	unpckhpd %xmm4,%xmm5
   88eb5:	shufps $0x24,%xmm5,%xmm8
   88eba:	andps  %xmm6,%xmm8
   88ebe:	movaps %xmm8,%xmm9
   88ec2:	pcmpgtd %xmm7,%xmm9
   88ec7:	movaps %xmm6,%xmm5
   88eca:	andnps %xmm9,%xmm5
   88ece:	orps   %xmm8,%xmm5
   88ed2:	pshufd $0xfa,%xmm2,%xmm2
   88ed7:	pshufd $0xff,%xmm1,%xmm8
   88edd:	punpckhdq %xmm2,%xmm1
   88ee1:	shufps $0xff,%xmm4,%xmm0
   88ee5:	shufps $0x24,%xmm0,%xmm1
   88ee9:	andps  %xmm6,%xmm1
   88eec:	movaps %xmm1,%xmm0
   88eef:	pcmpgtd %xmm7,%xmm0
   88ef3:	andnps %xmm0,%xmm6
   88ef6:	orps   %xmm6,%xmm1
   88ef9:	movd   %xmm8,%ecx
   88efe:	movzwl %cx,%ecx
   88f01:	xor    %edx,%edx
   88f03:	cmp    $0x8000,%ecx
   88f09:	setb   %dl
   88f0c:	shl    $0x10,%edx
   88f0f:	add    %edx,%ecx
   88f11:	add    $0xffff0000,%ecx
   88f17:	jmp    88fb8 <verified_tiff::vp8_transform::transform+0x6d8>
   88f1c:	movd   %xmm2,(%rax)
   88f20:	movdqa %xmm4,%xmm3
   88f24:	punpckldq %xmm0,%xmm3
   88f28:	movq   %xmm3,0x4(%rax)
   88f2d:	pshufd $0xff,%xmm1,%xmm3
   88f32:	movd   %xmm3,%ecx
   88f36:	pshufd $0x55,%xmm2,%xmm5
   88f3b:	movdqa %xmm1,%xmm3
   88f3f:	punpckldq %xmm5,%xmm3
   88f43:	movdqa %xmm0,%xmm5
   88f47:	shufps $0x55,%xmm4,%xmm5
   88f4b:	shufps $0x24,%xmm5,%xmm3
   88f4f:	pshufd $0xee,%xmm2,%xmm6
   88f54:	pshufd $0x55,%xmm1,%xmm5
   88f59:	punpckldq %xmm6,%xmm5
   88f5d:	movdqa %xmm0,%xmm6
   88f61:	punpckhqdq %xmm4,%xmm6
   88f65:	shufps $0x24,%xmm6,%xmm5
   88f69:	pshufd $0xfa,%xmm2,%xmm2
   88f6e:	punpckhdq %xmm2,%xmm1
   88f72:	shufps $0xff,%xmm4,%xmm0
   88f76:	shufps $0x24,%xmm0,%xmm1
   88f7a:	jmp    88fb8 <verified_tiff::vp8_transform::transform+0x6d8>
   88f7c:	lea    0x40000004(%rcx),%rdx
   88f83:	add    $0x4000000b,%rcx
   88f8a:	test   %rdx,%rdx
   88f8d:	cmovns %rdx,%rcx
   88f91:	shr    $0x3,%rcx
   88f95:	add    $0xf8000000,%ecx
   88f9b:	jmp    88f9f <verified_tiff::vp8_transform::transform+0x6bf>
   88f9d:	mov    %edx,%ecx
   88f9f:	mov    %ecx,(%rax)
   88fa1:	mov    %ecx,0x4(%rax)
   88fa4:	mov    %ecx,0x8(%rax)
   88fa7:	movd   %ecx,%xmm0
   88fab:	pshufd $0x0,%xmm0,%xmm3
   88fb0:	movdqa %xmm3,%xmm5
   88fb4:	movdqa %xmm3,%xmm1
   88fb8:	movups %xmm3,0xc(%rax)
   88fbc:	movups %xmm5,0x1c(%rax)
   88fc0:	movups %xmm1,0x2c(%rax)
   88fc4:	mov    %ecx,0x3c(%rax)
   88fc7:	ret
   88fc8:	nopl   0x0(%rax,%rax,1)

