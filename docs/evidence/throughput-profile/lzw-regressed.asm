<verified_tiff::decode_lz_segment>:
   29a30:	push   %rbp
   29a31:	push   %r15
   29a33:	push   %r14
   29a35:	push   %r13
   29a37:	push   %r12
   29a39:	push   %rbx
   29a3a:	sub    $0x1000,%rsp
   29a41:	movq   $0x0,(%rsp)
   29a49:	sub    $0x1000,%rsp
   29a50:	movq   $0x0,(%rsp)
   29a58:	sub    $0x1000,%rsp
   29a5f:	movq   $0x0,(%rsp)
   29a67:	sub    $0x1000,%rsp
   29a6e:	movq   $0x0,(%rsp)
   29a76:	sub    $0x1000,%rsp
   29a7d:	movq   $0x0,(%rsp)
   29a85:	sub    $0x1000,%rsp
   29a8c:	movq   $0x0,(%rsp)
   29a94:	sub    $0x1000,%rsp
   29a9b:	movq   $0x0,(%rsp)
   29aa3:	sub    $0x148,%rsp
   29aaa:	mov    %r9,0x48(%rsp)
   29aaf:	mov    %r8,0x58(%rsp)
   29ab4:	mov    %rcx,%r13
   29ab7:	mov    %rdx,%r15
   29aba:	mov    %rsi,%r14
   29abd:	mov    %rdi,%r12
   29ac0:	lea    0x38(%rdx),%rcx
   29ac4:	movzbl 0x68(%rdx),%ebx
   29ac8:	lea    0xf0(%rsp),%rdi
   29ad0:	mov    %r12,%rsi
   29ad3:	mov    %r14,%rdx
   29ad6:	mov    %r13,%r8
   29ad9:	mov    %ebx,%r9d
   29adc:	call   2bb00 <verified_tiff::value>
   29ae1:	cmpb   $0x1,0xf0(%rsp)
   29ae9:	je     29b24 <verified_tiff::decode_lz_segment+0xf4>
   29aeb:	mov    %r12,%rsi
   29aee:	mov    0xf8(%rsp),%r12
   29af6:	lea    0x50(%r15),%rcx
   29afa:	lea    0xf0(%rsp),%rdi
   29b02:	mov    %rsi,0x70(%rsp)
   29b07:	mov    %r14,%rdx
   29b0a:	mov    %r13,0x10(%rsp)
   29b0f:	mov    %r13,%r8
   29b12:	mov    %ebx,%r9d
   29b15:	call   2bb00 <verified_tiff::value>
   29b1a:	cmpb   $0x1,0xf0(%rsp)
   29b22:	jne    29b31 <verified_tiff::decode_lz_segment+0x101>
   29b24:	movzbl 0xf1(%rsp),%eax
   29b2c:	jmp    2a9b4 <verified_tiff::decode_lz_segment+0xf84>
   29b31:	mov    0xf8(%rsp),%rcx
   29b39:	movzbl 0x6a(%r15),%eax
   29b3e:	mov    0x20(%r15),%rsi
   29b42:	mov    %al,0xf(%rsp)
   29b46:	test   %al,%al
   29b48:	je     29b57 <verified_tiff::decode_lz_segment+0x127>
   29b4a:	lea    -0x1(%rsi),%rdi
   29b4e:	shr    $0x3,%rdi
   29b52:	inc    %rdi
   29b55:	jmp    29b67 <verified_tiff::decode_lz_segment+0x137>
   29b57:	mov    %rsi,%rax
   29b5a:	mulq   0x10(%r15)
   29b5e:	jo     2a9e4 <verified_tiff::decode_lz_segment+0xfb4>
   29b64:	mov    %rax,%rdi
   29b67:	mov    0x10(%rsp),%rax
   29b6c:	mov    0x28(%r15),%r8
   29b70:	cmpb   $0x0,0x6e(%r15)
   29b75:	mov    %r8,%rdx
   29b78:	jne    29b91 <verified_tiff::decode_lz_segment+0x161>
   29b7a:	mul    %r8
   29b7d:	jo     2a9e4 <verified_tiff::decode_lz_segment+0xfb4>
   29b83:	mov    0x8(%r15),%rdx
   29b87:	sub    %rax,%rdx
   29b8a:	cmp    %r8,%rdx
   29b8d:	cmovae %r8,%rdx
   29b91:	mov    %rdi,%rax
   29b94:	mul    %rdx
   29b97:	mov    %rax,0xa8(%rsp)
   29b9f:	jo     2a9e4 <verified_tiff::decode_lz_segment+0xfb4>
   29ba5:	cmp    $0x2,%rcx
   29ba9:	mov    %rdi,0x60(%rsp)
   29bae:	mov    %rsi,0x20(%rsp)
   29bb3:	mov    %r8,0x40(%rsp)
   29bb8:	jb     29bd3 <verified_tiff::decode_lz_segment+0x1a3>
   29bba:	cmp    %r14,%r12
   29bbd:	jae    2aa6f <verified_tiff::decode_lz_segment+0x103f>
   29bc3:	mov    0x70(%rsp),%rax
   29bc8:	cmpb   $0x0,(%rax,%r12,1)
   29bcd:	je     2a9c6 <verified_tiff::decode_lz_segment+0xf96>
   29bd3:	xor    %eax,%eax
   29bd5:	add    %r12,%rcx
   29bd8:	mov    %r12,0xc0(%rsp)
   29be0:	movq   $0x0,0xc8(%rsp)
   29bec:	mov    %rcx,0xd0(%rsp)
   29bf4:	mov    %al,0xd8(%rsp)
   29bfb:	lea    0xe0(%rsp),%rdi
   29c03:	lea    0xc0(%rsp),%rsi
   29c0b:	mov    $0x9,%r8d
   29c11:	mov    0x70(%rsp),%rdx
   29c16:	mov    %r14,0xb8(%rsp)
   29c1e:	mov    %r14,%rcx
   29c21:	call   2cf70 <<verified_tiff::LzBits>::read>
   29c26:	lea    0xf0(%rsp),%rdi
   29c2e:	mov    $0x7000,%edx
   29c33:	xor    %esi,%esi
   29c35:	call   *0x6682d(%rip)        # 90468 <memset@GLIBC_2.2.5>
   29c3b:	movq   $0x102,0x70f0(%rsp)
   29c47:	movq   $0x100,0x70f8(%rsp)
   29c53:	movups 0xc0(%rsp),%xmm0
   29c5b:	movups 0xd0(%rsp),%xmm1
   29c63:	movups %xmm0,0x7128(%rsp)
   29c6b:	movups %xmm1,0x7138(%rsp)
   29c73:	xorps  %xmm0,%xmm0
   29c76:	movups %xmm0,0x7100(%rsp)
   29c7e:	movq   $0x0,0x7110(%rsp)
   29c8a:	mov    %r12,0x7118(%rsp)
   29c92:	mov    0xa8(%rsp),%rax
   29c9a:	mov    %rax,0x7120(%rsp)
   29ca2:	test   %rax,%rax
   29ca5:	je     2a9b2 <verified_tiff::decode_lz_segment+0xf82>
   29cab:	mov    (%r15),%rcx
   29cae:	mov    0x8(%r15),%rsi
   29cb2:	mov    0x20(%rsp),%rax
   29cb7:	mov    %rcx,0x28(%rsp)
   29cbc:	xor    %rcx,%rax
   29cbf:	mov    0x30(%r15),%rcx
   29cc3:	mov    %rcx,0x18(%rsp)
   29cc8:	xor    $0x1,%rcx
   29ccc:	or     %rax,%rcx
   29ccf:	sete   0x37(%rsp)
   29cd4:	mov    0x10(%rsp),%rax
   29cd9:	mulq   0x40(%rsp)
   29cde:	seto   0x36(%rsp)
   29ce3:	mov    %rsi,0x50(%rsp)
   29ce8:	mov    %rax,0xa0(%rsp)
   29cf0:	sub    %rax,%rsi
   29cf3:	mov    %rsi,0xb0(%rsp)
   29cfb:	movzbl 0x6b(%r15),%eax
   29d00:	cmpq   $0x0,0x18(%r15)
   29d05:	sete   %cl
   29d08:	and    0x69(%r15),%cl
   29d0c:	neg    %cl
   29d0e:	mov    %cl,0xe(%rsp)
   29d12:	mov    %eax,%ecx
   29d14:	xor    $0x1,%cl
   29d17:	test   %eax,%eax
   29d19:	mov    $0x2,%edi
   29d1e:	mov    $0x40,%edx
   29d23:	mov    $0x40,%esi
   29d28:	cmovne %edi,%esi
   29d2b:	mov    %esi,0x98(%rsp)
   29d32:	mov    $0x4,%r8d
   29d38:	mov    $0x20,%esi
   29d3d:	mov    $0x20,%r9d
   29d43:	cmovne %r8d,%r9d
   29d47:	mov    %r9d,0x94(%rsp)
   29d4f:	cmovne %esi,%r8d
   29d53:	mov    %r8d,0x88(%rsp)
   29d5b:	cmovne %edx,%edi
   29d5e:	mov    %edi,0x8c(%rsp)
   29d65:	mov    $0x1,%esi
   29d6a:	mov    $0x80,%edx
   29d6f:	cmovne %esi,%edx
   29d72:	mov    %edx,0x9c(%rsp)
   29d79:	mov    $0x80,%edx
   29d7e:	cmovne %edx,%esi
   29d81:	mov    %esi,0x80(%rsp)
   29d88:	mov    0x10(%r15),%rdx
   29d8c:	mov    %rdx,0x38(%rsp)
   29d91:	movzbl %cl,%ecx
   29d94:	lea    0x8(,%rcx,8),%ecx
   29d9b:	mov    %ecx,0x90(%rsp)
   29da2:	lea    0x8(,%rax,8),%eax
   29da9:	mov    %eax,0x84(%rsp)
   29db0:	lea    0x10f0(%rsp),%r15
   29db8:	xor    %ebx,%ebx
   29dba:	xor    %r14d,%r14d
   29dbd:	movq   $0x0,0x78(%rsp)
   29dc6:	xor    %r12d,%r12d
   29dc9:	xor    %r13d,%r13d
   29dcc:	xor    %eax,%eax
   29dce:	jmp    29e5c <verified_tiff::decode_lz_segment+0x42c>
   29dd3:	mov    0xa0(%rsp),%rax
   29ddb:	cmp    %rax,0x50(%rsp)
   29de0:	setbe  %al
   29de3:	cmp    0xb0(%rsp),%rsi
   29deb:	setae  %dl
   29dee:	or     0x36(%rsp),%al
   29df2:	or     %dl,%al
   29df4:	je     2a157 <verified_tiff::decode_lz_segment+0x727>
   29dfa:	nopw   0x0(%rax,%rax,1)
   29e00:	inc    %rbx
   29e03:	mov    0x60(%rsp),%rax
   29e08:	cmp    %rax,%rbx
   29e0b:	mov    $0x0,%ecx
   29e10:	cmove  %rcx,%rbx
   29e14:	inc    %r14
   29e17:	cmp    %rax,%r14
   29e1a:	cmove  %rcx,%r14
   29e1e:	mov    0x78(%rsp),%rdx
   29e23:	inc    %rdx
   29e26:	cmp    %rax,%rdx
   29e29:	cmove  %rcx,%rdx
   29e2d:	mov    %rdx,0x78(%rsp)
   29e32:	inc    %r12
   29e35:	cmp    %rax,%r12
   29e38:	cmove  %rcx,%r12
   29e3c:	inc    %r13
   29e3f:	cmp    %rax,%r13
   29e42:	cmove  %rcx,%r13
   29e46:	mov    0x68(%rsp),%rax
   29e4b:	inc    %rax
   29e4e:	cmp    0xa8(%rsp),%rax
   29e56:	je     2a9b2 <verified_tiff::decode_lz_segment+0xf82>
   29e5c:	mov    %rax,0x68(%rsp)
   29e61:	mov    0x7100(%rsp),%rdi
   29e69:	cmp    0x7108(%rsp),%rdi
   29e71:	je     29eed <verified_tiff::decode_lz_segment+0x4bd>
   29e73:	cmp    $0x1000,%rdi
   29e7a:	jae    2aa31 <verified_tiff::decode_lz_segment+0x1001>
   29e80:	movzbl 0xf0(%rsp,%rdi,1),%r11d
   29e89:	inc    %rdi
   29e8c:	mov    %rdi,0x7100(%rsp)
   29e94:	incq   0x7110(%rsp)
   29e9c:	mov    0x60(%rsp),%rcx
   29ea1:	mov    0x68(%rsp),%rax
   29ea6:	test   %rcx,%rcx
   29ea9:	jne    2a096 <verified_tiff::decode_lz_segment+0x666>
   29eaf:	jmp    2aa24 <verified_tiff::decode_lz_segment+0xff4>
   29eb4:	data16 data16 cs nopw 0x0(%rax,%rax,1)
   29ec0:	movq   $0x102,0x70f0(%rsp)
   29ecc:	movq   $0x100,0x70f8(%rsp)
   29ed8:	mov    0x7100(%rsp),%rdi
   29ee0:	mov    0x7108(%rsp),%rax
   29ee8:	cmp    %rax,%rdi
   29eeb:	jne    29e73 <verified_tiff::decode_lz_segment+0x443>
   29eed:	mov    0x70f0(%rsp),%rax
   29ef5:	cmpb   $0x0,0x7140(%rsp)
   29efd:	je     29f30 <verified_tiff::decode_lz_segment+0x500>
   29eff:	mov    $0x9,%r8d
   29f05:	cmp    $0x200,%rax
   29f0b:	jb     29f5c <verified_tiff::decode_lz_segment+0x52c>
   29f0d:	mov    $0xa,%r8d
   29f13:	cmp    $0x400,%rax
   29f19:	jb     29f5c <verified_tiff::decode_lz_segment+0x52c>
   29f1b:	cmp    $0x800,%rax
   29f21:	jmp    29f52 <verified_tiff::decode_lz_segment+0x522>
   29f23:	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   29f30:	mov    $0x9,%r8d
   29f36:	cmp    $0x1ff,%rax
   29f3c:	jb     29f5c <verified_tiff::decode_lz_segment+0x52c>
   29f3e:	mov    $0xa,%r8d
   29f44:	cmp    $0x3ff,%rax
   29f4a:	jb     29f5c <verified_tiff::decode_lz_segment+0x52c>
   29f4c:	cmp    $0x7ff,%rax
   29f52:	mov    $0xc,%r8d
   29f58:	sbb    $0x0,%r8
   29f5c:	lea    0xe0(%rsp),%rdi
   29f64:	lea    0x7128(%rsp),%rsi
   29f6c:	mov    0x70(%rsp),%rdx
   29f71:	mov    0xb8(%rsp),%rcx
   29f79:	call   2cf70 <<verified_tiff::LzBits>::read>
   29f7e:	cmpb   $0x0,0xe0(%rsp)
   29f86:	jne    2a080 <verified_tiff::decode_lz_segment+0x650>
   29f8c:	mov    0xe8(%rsp),%rbp
   29f94:	cmp    $0x100,%rbp
   29f9b:	je     29ec0 <verified_tiff::decode_lz_segment+0x490>
   29fa1:	movzbl 0x7140(%rsp),%ecx
   29fa9:	lea    0xc0(%rsp),%rdi
   29fb1:	mov    %r15,%rsi
   29fb4:	mov    %rbp,%rdx
   29fb7:	call   2d1c0 <<verified_tiff::LzTable>::data>
   29fbc:	mov    $0x1,%eax
   29fc1:	cmp    $0x100,%rbp
   29fc8:	jb     2a043 <verified_tiff::decode_lz_segment+0x613>
   29fca:	cmp    $0x1000,%rbp
   29fd1:	jae    2aa0f <verified_tiff::decode_lz_segment+0xfdf>
   29fd7:	lea    0x0(,%rbp,2),%rax
   29fdf:	add    %rbp,%rax
   29fe2:	movzwl 0x2(%r15,%rax,2),%eax
   29fe8:	mov    $0x101,%ecx
   29fed:	cmp    $0x102,%rbp
   29ff4:	jb     2a046 <verified_tiff::decode_lz_segment+0x616>
   29ff6:	lea    -0x1(%rax),%rdi
   29ffa:	nopw   0x0(%rax,%rax,1)
   2a000:	cmp    $0xfff,%rbp
   2a007:	ja     2a9fa <verified_tiff::decode_lz_segment+0xfca>
   2a00d:	cmp    $0x1000,%rdi
   2a014:	jae    2a9e8 <verified_tiff::decode_lz_segment+0xfb8>
   2a01a:	lea    0x0(,%rbp,2),%rcx
   2a022:	add    %rbp,%rcx
   2a025:	movzbl 0x4(%r15,%rcx,2),%edx
   2a02b:	movzwl (%r15,%rcx,2),%ebp
   2a030:	mov    %dl,0xf0(%rsp,%rdi,1)
   2a037:	dec    %rdi
   2a03a:	cmp    $0x101,%rbp
   2a041:	ja     2a000 <verified_tiff::decode_lz_segment+0x5d0>
   2a043:	mov    %rbp,%rcx
   2a046:	mov    %cl,0xf0(%rsp)
   2a04d:	mov    %rax,0x7108(%rsp)
   2a055:	movq   $0x0,0x7100(%rsp)
   2a061:	xor    %edi,%edi
   2a063:	cmp    %rax,%rdi
   2a066:	je     29eed <verified_tiff::decode_lz_segment+0x4bd>
   2a06c:	jmp    29e73 <verified_tiff::decode_lz_segment+0x443>
   2a071:	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   2a080:	xor    %r11d,%r11d
   2a083:	mov    0x60(%rsp),%rcx
   2a088:	mov    0x68(%rsp),%rax
   2a08d:	test   %rcx,%rcx
   2a090:	je     2aa24 <verified_tiff::decode_lz_segment+0xff4>
   2a096:	mov    %rax,%rbp
   2a099:	or     %rcx,%rbp
   2a09c:	shr    $0x20,%rbp
   2a0a0:	je     2a0c0 <verified_tiff::decode_lz_segment+0x690>
   2a0a2:	xor    %edx,%edx
   2a0a4:	div    %rcx
   2a0a7:	mov    %rax,%rsi
   2a0aa:	mov    %rbx,%rcx
   2a0ad:	cmp    0x40(%rsp),%rsi
   2a0b2:	jae    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a0b8:	jmp    2a0d3 <verified_tiff::decode_lz_segment+0x6a3>
   2a0ba:	nopw   0x0(%rax,%rax,1)
   2a0c0:	xor    %edx,%edx
   2a0c2:	div    %ecx
   2a0c4:	mov    %edx,%ecx
   2a0c6:	mov    %eax,%esi
   2a0c8:	cmp    0x40(%rsp),%rsi
   2a0cd:	jae    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a0d3:	movzbl %r11b,%r10d
   2a0d7:	cmpb   $0x0,0xf(%rsp)
   2a0dc:	je     2a100 <verified_tiff::decode_lz_segment+0x6d0>
   2a0de:	test   %rbp,%rbp
   2a0e1:	je     2a126 <verified_tiff::decode_lz_segment+0x6f6>
   2a0e3:	mov    %r12,%r8
   2a0e6:	movabs $0x1fffffffffffffff,%rax
   2a0f0:	cmp    %rax,%r8
   2a0f3:	jbe    2a147 <verified_tiff::decode_lz_segment+0x717>
   2a0f5:	jmp    2a32a <verified_tiff::decode_lz_segment+0x8fa>
   2a0fa:	nopw   0x0(%rax,%rax,1)
   2a100:	cmpb   $0x0,0x37(%rsp)
   2a105:	jne    29dd3 <verified_tiff::decode_lz_segment+0x3a3>
   2a10b:	cmpq   $0x0,0x38(%rsp)
   2a111:	je     2aa62 <verified_tiff::decode_lz_segment+0x1032>
   2a117:	test   %rbp,%rbp
   2a11a:	mov    0x60(%rsp),%rdi
   2a11f:	je     2a182 <verified_tiff::decode_lz_segment+0x752>
   2a121:	mov    %r14,%rax
   2a124:	jmp    2a18d <verified_tiff::decode_lz_segment+0x75d>
   2a126:	mov    0x68(%rsp),%rax
   2a12b:	xor    %edx,%edx
   2a12d:	divl   0x60(%rsp)
   2a131:	mov    %edx,%r8d
   2a134:	movabs $0x1fffffffffffffff,%rax
   2a13e:	cmp    %rax,%r8
   2a141:	ja     2a32a <verified_tiff::decode_lz_segment+0x8fa>
   2a147:	shl    $0x3,%r8
   2a14b:	cmp    0x20(%rsp),%r8
   2a150:	jb     2a1c4 <verified_tiff::decode_lz_segment+0x794>
   2a152:	jmp    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a157:	mov    0xa0(%rsp),%rax
   2a15f:	lea    (%rsi,%rax,1),%r8
   2a163:	mov    0x60(%rsp),%rdi
   2a168:	imul   %rdi,%r8
   2a16c:	test   %rbp,%rbp
   2a16f:	je     2a2a6 <verified_tiff::decode_lz_segment+0x876>
   2a175:	add    0x78(%rsp),%r8
   2a17a:	mov    %r8,%rdi
   2a17d:	jmp    2a306 <verified_tiff::decode_lz_segment+0x8d6>
   2a182:	mov    0x68(%rsp),%rax
   2a187:	xor    %edx,%edx
   2a189:	div    %edi
   2a18b:	mov    %edx,%eax
   2a18d:	mov    %rax,%rdx
   2a190:	mov    0x38(%rsp),%rdi
   2a195:	or     %rdi,%rdx
   2a198:	shr    $0x20,%rdx
   2a19c:	je     2a1b2 <verified_tiff::decode_lz_segment+0x782>
   2a19e:	xor    %edx,%edx
   2a1a0:	div    %rdi
   2a1a3:	mov    %rax,%r8
   2a1a6:	cmp    0x20(%rsp),%r8
   2a1ab:	jb     2a1c4 <verified_tiff::decode_lz_segment+0x794>
   2a1ad:	jmp    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a1b2:	xor    %edx,%edx
   2a1b4:	div    %edi
   2a1b6:	mov    %eax,%r8d
   2a1b9:	cmp    0x20(%rsp),%r8
   2a1be:	jae    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a1c4:	cmpq   $0x0,0x18(%rsp)
   2a1ca:	je     2aa43 <verified_tiff::decode_lz_segment+0x1013>
   2a1d0:	mov    0x10(%rsp),%rax
   2a1d5:	mov    %rax,%rdi
   2a1d8:	mov    0x18(%rsp),%r9
   2a1dd:	or     %r9,%rdi
   2a1e0:	shr    $0x20,%rdi
   2a1e4:	je     2a1fa <verified_tiff::decode_lz_segment+0x7ca>
   2a1e6:	xor    %edx,%edx
   2a1e8:	div    %r9
   2a1eb:	mov    %rdx,%rax
   2a1ee:	mulq   0x20(%rsp)
   2a1f3:	jno    2a20c <verified_tiff::decode_lz_segment+0x7dc>
   2a1f5:	jmp    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a1fa:	xor    %edx,%edx
   2a1fc:	div    %r9d
   2a1ff:	mov    %edx,%eax
   2a201:	mulq   0x20(%rsp)
   2a206:	jo     2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a20c:	mov    %rax,%r9
   2a20f:	test   %rdi,%rdi
   2a212:	je     2a22c <verified_tiff::decode_lz_segment+0x7fc>
   2a214:	mov    0x10(%rsp),%rax
   2a219:	xor    %edx,%edx
   2a21b:	divq   0x18(%rsp)
   2a220:	mulq   0x40(%rsp)
   2a225:	jno    2a242 <verified_tiff::decode_lz_segment+0x812>
   2a227:	jmp    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a22c:	mov    0x10(%rsp),%rax
   2a231:	xor    %edx,%edx
   2a233:	divl   0x18(%rsp)
   2a237:	mulq   0x40(%rsp)
   2a23c:	jo     2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a242:	mov    %rax,%rdi
   2a245:	mov    0x28(%rsp),%rax
   2a24a:	sub    %r9,%rax
   2a24d:	jbe    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a253:	cmp    %rax,%r8
   2a256:	jae    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a25c:	mov    0x50(%rsp),%rax
   2a261:	sub    %rdi,%rax
   2a264:	jbe    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a26a:	cmp    %rax,%rsi
   2a26d:	jae    2a31f <verified_tiff::decode_lz_segment+0x8ef>
   2a273:	add    %r8,%r9
   2a276:	add    %rsi,%rdi
   2a279:	cmpb   $0x0,0xf(%rsp)
   2a27e:	je     2a2b6 <verified_tiff::decode_lz_segment+0x886>
   2a280:	imul   0x28(%rsp),%rdi
   2a286:	add    %r9,%rdi
   2a289:	imul   0x38(%rsp),%rdi
   2a28f:	mov    0x9c(%rsp),%eax
   2a296:	and    %r10d,%eax
   2a299:	neg    %eax
   2a29b:	mov    $0x0,%r11d
   2a2a1:	sbb    %r11d,%r11d
   2a2a4:	jmp    2a306 <verified_tiff::decode_lz_segment+0x8d6>
   2a2a6:	mov    0x68(%rsp),%rax
   2a2ab:	xor    %edx,%edx
   2a2ad:	div    %edi
   2a2af:	mov    %edx,%edi
   2a2b1:	add    %r8,%rdi
   2a2b4:	jmp    2a306 <verified_tiff::decode_lz_segment+0x8d6>
   2a2b6:	cmpq   $0x0,0x38(%rsp)
   2a2bc:	je     2aa82 <verified_tiff::decode_lz_segment+0x1052>
   2a2c2:	test   %rbp,%rbp
   2a2c5:	je     2a2cc <verified_tiff::decode_lz_segment+0x89c>
   2a2c7:	mov    %r13,%rax
   2a2ca:	jmp    2a2d9 <verified_tiff::decode_lz_segment+0x8a9>
   2a2cc:	mov    0x68(%rsp),%rax
   2a2d1:	xor    %edx,%edx
   2a2d3:	divl   0x60(%rsp)
   2a2d7:	mov    %edx,%eax
   2a2d9:	mov    %rax,%rdx
   2a2dc:	mov    0x38(%rsp),%r8
   2a2e1:	or     %r8,%rdx
   2a2e4:	shr    $0x20,%rdx
   2a2e8:	je     2a2f1 <verified_tiff::decode_lz_segment+0x8c1>
   2a2ea:	xor    %edx,%edx
   2a2ec:	div    %r8
   2a2ef:	jmp    2a2f6 <verified_tiff::decode_lz_segment+0x8c6>
   2a2f1:	xor    %edx,%edx
   2a2f3:	div    %r8d
   2a2f6:	imul   0x28(%rsp),%rdi
   2a2fc:	add    %r9,%rdi
   2a2ff:	imul   %r8,%rdi
   2a303:	add    %rdx,%rdi
   2a306:	cmp    0x48(%rsp),%rdi
   2a30b:	jae    2aa50 <verified_tiff::decode_lz_segment+0x1020>
   2a311:	xor    0xe(%rsp),%r11b
   2a316:	mov    0x58(%rsp),%rax
   2a31b:	mov    %r11b,(%rax,%rdi,1)
   2a31f:	cmpb   $0x0,0xf(%rsp)
   2a324:	je     29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a32a:	movabs $0x1fffffffffffffff,%rax
   2a334:	cmp    %rax,%rcx
   2a337:	ja     29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a33d:	shl    $0x3,%rcx
   2a341:	mov    %rcx,%r9
   2a344:	or     $0x1,%r9
   2a348:	cmpq   $0x0,0x18(%rsp)
   2a34e:	je     2a38d <verified_tiff::decode_lz_segment+0x95d>
   2a350:	cmp    0x20(%rsp),%r9
   2a355:	jae    2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a35b:	mov    0x10(%rsp),%rax
   2a360:	mov    %rax,%rdi
   2a363:	mov    0x18(%rsp),%r8
   2a368:	or     %r8,%rdi
   2a36b:	shr    $0x20,%rdi
   2a36f:	je     2a406 <verified_tiff::decode_lz_segment+0x9d6>
   2a375:	xor    %edx,%edx
   2a377:	div    %r8
   2a37a:	mov    %rdx,%rax
   2a37d:	mulq   0x20(%rsp)
   2a382:	jno    2a418 <verified_tiff::decode_lz_segment+0x9e8>
   2a388:	jmp    2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a38d:	cmp    0x20(%rsp),%r9
   2a392:	jb     2aa43 <verified_tiff::decode_lz_segment+0x1013>
   2a398:	mov    %rcx,%rax
   2a39b:	or     $0x2,%rax
   2a39f:	cmp    0x20(%rsp),%rax
   2a3a4:	jb     2aa43 <verified_tiff::decode_lz_segment+0x1013>
   2a3aa:	mov    %rcx,%rax
   2a3ad:	or     $0x3,%rax
   2a3b1:	cmp    0x20(%rsp),%rax
   2a3b6:	jb     2aa43 <verified_tiff::decode_lz_segment+0x1013>
   2a3bc:	mov    %rcx,%rax
   2a3bf:	or     $0x4,%rax
   2a3c3:	cmp    0x20(%rsp),%rax
   2a3c8:	jb     2aa43 <verified_tiff::decode_lz_segment+0x1013>
   2a3ce:	mov    %rcx,%rax
   2a3d1:	or     $0x5,%rax
   2a3d5:	cmp    0x20(%rsp),%rax
   2a3da:	jb     2aa43 <verified_tiff::decode_lz_segment+0x1013>
   2a3e0:	mov    %rcx,%rax
   2a3e3:	or     $0x6,%rax
   2a3e7:	cmp    0x20(%rsp),%rax
   2a3ec:	jb     2aa43 <verified_tiff::decode_lz_segment+0x1013>
   2a3f2:	or     $0x7,%rcx
   2a3f6:	cmp    0x20(%rsp),%rcx
   2a3fb:	jae    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a401:	jmp    2aa43 <verified_tiff::decode_lz_segment+0x1013>
   2a406:	xor    %edx,%edx
   2a408:	div    %r8d
   2a40b:	mov    %edx,%eax
   2a40d:	mulq   0x20(%rsp)
   2a412:	jo     2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a418:	mov    %rax,%r8
   2a41b:	test   %rdi,%rdi
   2a41e:	je     2a435 <verified_tiff::decode_lz_segment+0xa05>
   2a420:	mov    0x10(%rsp),%rax
   2a425:	xor    %edx,%edx
   2a427:	divq   0x18(%rsp)
   2a42c:	mulq   0x40(%rsp)
   2a431:	jno    2a447 <verified_tiff::decode_lz_segment+0xa17>
   2a433:	jmp    2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a435:	mov    0x10(%rsp),%rax
   2a43a:	xor    %edx,%edx
   2a43c:	divl   0x18(%rsp)
   2a440:	mulq   0x40(%rsp)
   2a445:	jo     2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a447:	mov    %rax,%rdi
   2a44a:	mov    0x28(%rsp),%rax
   2a44f:	sub    %r8,%rax
   2a452:	jbe    2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a454:	cmp    %rax,%r9
   2a457:	jae    2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a459:	mov    0x50(%rsp),%rax
   2a45e:	sub    %rdi,%rax
   2a461:	jbe    2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a463:	cmp    %rax,%rsi
   2a466:	jae    2a4a3 <verified_tiff::decode_lz_segment+0xa73>
   2a468:	add    %rsi,%rdi
   2a46b:	add    %r9,%r8
   2a46e:	imul   0x28(%rsp),%rdi
   2a474:	add    %r8,%rdi
   2a477:	imul   0x38(%rsp),%rdi
   2a47d:	cmp    0x48(%rsp),%rdi
   2a482:	jae    2aa50 <verified_tiff::decode_lz_segment+0x1020>
   2a488:	test   %r10d,0x98(%rsp)
   2a490:	setne  %al
   2a493:	xor    0xe(%rsp),%al
   2a497:	and    $0x1,%al
   2a499:	neg    %al
   2a49b:	mov    0x58(%rsp),%rdx
   2a4a0:	mov    %al,(%rdx,%rdi,1)
   2a4a3:	mov    %rcx,%r9
   2a4a6:	or     $0x2,%r9
   2a4aa:	cmp    0x20(%rsp),%r9
   2a4af:	jae    2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a4b5:	mov    0x10(%rsp),%rax
   2a4ba:	mov    %rax,%rdi
   2a4bd:	mov    0x18(%rsp),%r8
   2a4c2:	or     %r8,%rdi
   2a4c5:	shr    $0x20,%rdi
   2a4c9:	je     2a4df <verified_tiff::decode_lz_segment+0xaaf>
   2a4cb:	xor    %edx,%edx
   2a4cd:	div    %r8
   2a4d0:	mov    %rdx,%rax
   2a4d3:	mulq   0x20(%rsp)
   2a4d8:	jno    2a4f1 <verified_tiff::decode_lz_segment+0xac1>
   2a4da:	jmp    2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a4df:	xor    %edx,%edx
   2a4e1:	div    %r8d
   2a4e4:	mov    %edx,%eax
   2a4e6:	mulq   0x20(%rsp)
   2a4eb:	jo     2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a4f1:	mov    %rax,%r8
   2a4f4:	test   %rdi,%rdi
   2a4f7:	je     2a50e <verified_tiff::decode_lz_segment+0xade>
   2a4f9:	mov    0x10(%rsp),%rax
   2a4fe:	xor    %edx,%edx
   2a500:	divq   0x18(%rsp)
   2a505:	mulq   0x40(%rsp)
   2a50a:	jno    2a520 <verified_tiff::decode_lz_segment+0xaf0>
   2a50c:	jmp    2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a50e:	mov    0x10(%rsp),%rax
   2a513:	xor    %edx,%edx
   2a515:	divl   0x18(%rsp)
   2a519:	mulq   0x40(%rsp)
   2a51e:	jo     2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a520:	mov    %rax,%rdi
   2a523:	mov    0x28(%rsp),%rax
   2a528:	sub    %r8,%rax
   2a52b:	jbe    2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a52d:	cmp    %rax,%r9
   2a530:	jae    2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a532:	mov    0x50(%rsp),%rax
   2a537:	sub    %rdi,%rax
   2a53a:	jbe    2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a53c:	cmp    %rax,%rsi
   2a53f:	jae    2a57c <verified_tiff::decode_lz_segment+0xb4c>
   2a541:	add    %rsi,%rdi
   2a544:	add    %r9,%r8
   2a547:	imul   0x28(%rsp),%rdi
   2a54d:	add    %r8,%rdi
   2a550:	imul   0x38(%rsp),%rdi
   2a556:	cmp    0x48(%rsp),%rdi
   2a55b:	jae    2aa50 <verified_tiff::decode_lz_segment+0x1020>
   2a561:	test   %r10d,0x94(%rsp)
   2a569:	setne  %al
   2a56c:	xor    0xe(%rsp),%al
   2a570:	and    $0x1,%al
   2a572:	neg    %al
   2a574:	mov    0x58(%rsp),%rdx
   2a579:	mov    %al,(%rdx,%rdi,1)
   2a57c:	mov    %rcx,%r9
   2a57f:	or     $0x3,%r9
   2a583:	cmp    0x20(%rsp),%r9
   2a588:	jae    2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a58e:	mov    0x10(%rsp),%rax
   2a593:	mov    %rax,%rdi
   2a596:	mov    0x18(%rsp),%r8
   2a59b:	or     %r8,%rdi
   2a59e:	shr    $0x20,%rdi
   2a5a2:	je     2a5b8 <verified_tiff::decode_lz_segment+0xb88>
   2a5a4:	xor    %edx,%edx
   2a5a6:	div    %r8
   2a5a9:	mov    %rdx,%rax
   2a5ac:	mulq   0x20(%rsp)
   2a5b1:	jno    2a5ca <verified_tiff::decode_lz_segment+0xb9a>
   2a5b3:	jmp    2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a5b8:	xor    %edx,%edx
   2a5ba:	div    %r8d
   2a5bd:	mov    %edx,%eax
   2a5bf:	mulq   0x20(%rsp)
   2a5c4:	jo     2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a5ca:	mov    %rax,%r8
   2a5cd:	test   %rdi,%rdi
   2a5d0:	je     2a5e7 <verified_tiff::decode_lz_segment+0xbb7>
   2a5d2:	mov    0x10(%rsp),%rax
   2a5d7:	xor    %edx,%edx
   2a5d9:	divq   0x18(%rsp)
   2a5de:	mulq   0x40(%rsp)
   2a5e3:	jno    2a5f9 <verified_tiff::decode_lz_segment+0xbc9>
   2a5e5:	jmp    2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a5e7:	mov    0x10(%rsp),%rax
   2a5ec:	xor    %edx,%edx
   2a5ee:	divl   0x18(%rsp)
   2a5f2:	mulq   0x40(%rsp)
   2a5f7:	jo     2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a5f9:	mov    %rax,%rdi
   2a5fc:	mov    0x28(%rsp),%rax
   2a601:	sub    %r8,%rax
   2a604:	jbe    2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a606:	cmp    %rax,%r9
   2a609:	jae    2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a60b:	mov    0x50(%rsp),%rax
   2a610:	sub    %rdi,%rax
   2a613:	jbe    2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a615:	cmp    %rax,%rsi
   2a618:	jae    2a655 <verified_tiff::decode_lz_segment+0xc25>
   2a61a:	add    %rsi,%rdi
   2a61d:	add    %r9,%r8
   2a620:	imul   0x28(%rsp),%rdi
   2a626:	add    %r8,%rdi
   2a629:	imul   0x38(%rsp),%rdi
   2a62f:	cmp    0x48(%rsp),%rdi
   2a634:	jae    2aa50 <verified_tiff::decode_lz_segment+0x1020>
   2a63a:	test   %r10d,0x90(%rsp)
   2a642:	setne  %al
   2a645:	xor    0xe(%rsp),%al
   2a649:	and    $0x1,%al
   2a64b:	neg    %al
   2a64d:	mov    0x58(%rsp),%rdx
   2a652:	mov    %al,(%rdx,%rdi,1)
   2a655:	mov    %rcx,%r9
   2a658:	or     $0x4,%r9
   2a65c:	cmp    0x20(%rsp),%r9
   2a661:	jae    2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a667:	mov    0x10(%rsp),%rax
   2a66c:	mov    %rax,%rdi
   2a66f:	mov    0x18(%rsp),%r8
   2a674:	or     %r8,%rdi
   2a677:	shr    $0x20,%rdi
   2a67b:	je     2a691 <verified_tiff::decode_lz_segment+0xc61>
   2a67d:	xor    %edx,%edx
   2a67f:	div    %r8
   2a682:	mov    %rdx,%rax
   2a685:	mulq   0x20(%rsp)
   2a68a:	jno    2a6a3 <verified_tiff::decode_lz_segment+0xc73>
   2a68c:	jmp    2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a691:	xor    %edx,%edx
   2a693:	div    %r8d
   2a696:	mov    %edx,%eax
   2a698:	mulq   0x20(%rsp)
   2a69d:	jo     2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a6a3:	mov    %rax,%r8
   2a6a6:	test   %rdi,%rdi
   2a6a9:	je     2a6c0 <verified_tiff::decode_lz_segment+0xc90>
   2a6ab:	mov    0x10(%rsp),%rax
   2a6b0:	xor    %edx,%edx
   2a6b2:	divq   0x18(%rsp)
   2a6b7:	mulq   0x40(%rsp)
   2a6bc:	jno    2a6d2 <verified_tiff::decode_lz_segment+0xca2>
   2a6be:	jmp    2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a6c0:	mov    0x10(%rsp),%rax
   2a6c5:	xor    %edx,%edx
   2a6c7:	divl   0x18(%rsp)
   2a6cb:	mulq   0x40(%rsp)
   2a6d0:	jo     2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a6d2:	mov    %rax,%rdi
   2a6d5:	mov    0x28(%rsp),%rax
   2a6da:	sub    %r8,%rax
   2a6dd:	jbe    2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a6df:	cmp    %rax,%r9
   2a6e2:	jae    2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a6e4:	mov    0x50(%rsp),%rax
   2a6e9:	sub    %rdi,%rax
   2a6ec:	jbe    2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a6ee:	cmp    %rax,%rsi
   2a6f1:	jae    2a72e <verified_tiff::decode_lz_segment+0xcfe>
   2a6f3:	add    %rsi,%rdi
   2a6f6:	add    %r9,%r8
   2a6f9:	imul   0x28(%rsp),%rdi
   2a6ff:	add    %r8,%rdi
   2a702:	imul   0x38(%rsp),%rdi
   2a708:	cmp    0x48(%rsp),%rdi
   2a70d:	jae    2aa50 <verified_tiff::decode_lz_segment+0x1020>
   2a713:	test   %r10d,0x84(%rsp)
   2a71b:	setne  %al
   2a71e:	xor    0xe(%rsp),%al
   2a722:	and    $0x1,%al
   2a724:	neg    %al
   2a726:	mov    0x58(%rsp),%rdx
   2a72b:	mov    %al,(%rdx,%rdi,1)
   2a72e:	mov    %rcx,%r9
   2a731:	or     $0x5,%r9
   2a735:	cmp    0x20(%rsp),%r9
   2a73a:	jae    2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a740:	mov    0x10(%rsp),%rax
   2a745:	mov    %rax,%rdi
   2a748:	mov    0x18(%rsp),%r8
   2a74d:	or     %r8,%rdi
   2a750:	shr    $0x20,%rdi
   2a754:	je     2a76a <verified_tiff::decode_lz_segment+0xd3a>
   2a756:	xor    %edx,%edx
   2a758:	div    %r8
   2a75b:	mov    %rdx,%rax
   2a75e:	mulq   0x20(%rsp)
   2a763:	jno    2a77c <verified_tiff::decode_lz_segment+0xd4c>
   2a765:	jmp    2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a76a:	xor    %edx,%edx
   2a76c:	div    %r8d
   2a76f:	mov    %edx,%eax
   2a771:	mulq   0x20(%rsp)
   2a776:	jo     2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a77c:	mov    %rax,%r8
   2a77f:	test   %rdi,%rdi
   2a782:	je     2a799 <verified_tiff::decode_lz_segment+0xd69>
   2a784:	mov    0x10(%rsp),%rax
   2a789:	xor    %edx,%edx
   2a78b:	divq   0x18(%rsp)
   2a790:	mulq   0x40(%rsp)
   2a795:	jno    2a7ab <verified_tiff::decode_lz_segment+0xd7b>
   2a797:	jmp    2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a799:	mov    0x10(%rsp),%rax
   2a79e:	xor    %edx,%edx
   2a7a0:	divl   0x18(%rsp)
   2a7a4:	mulq   0x40(%rsp)
   2a7a9:	jo     2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a7ab:	mov    %rax,%rdi
   2a7ae:	mov    0x28(%rsp),%rax
   2a7b3:	sub    %r8,%rax
   2a7b6:	jbe    2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a7b8:	cmp    %rax,%r9
   2a7bb:	jae    2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a7bd:	mov    0x50(%rsp),%rax
   2a7c2:	sub    %rdi,%rax
   2a7c5:	jbe    2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a7c7:	cmp    %rax,%rsi
   2a7ca:	jae    2a807 <verified_tiff::decode_lz_segment+0xdd7>
   2a7cc:	add    %rsi,%rdi
   2a7cf:	add    %r9,%r8
   2a7d2:	imul   0x28(%rsp),%rdi
   2a7d8:	add    %r8,%rdi
   2a7db:	imul   0x38(%rsp),%rdi
   2a7e1:	cmp    0x48(%rsp),%rdi
   2a7e6:	jae    2aa50 <verified_tiff::decode_lz_segment+0x1020>
   2a7ec:	test   %r10d,0x88(%rsp)
   2a7f4:	setne  %al
   2a7f7:	xor    0xe(%rsp),%al
   2a7fb:	and    $0x1,%al
   2a7fd:	neg    %al
   2a7ff:	mov    0x58(%rsp),%rdx
   2a804:	mov    %al,(%rdx,%rdi,1)
   2a807:	mov    %rcx,%r9
   2a80a:	or     $0x6,%r9
   2a80e:	cmp    0x20(%rsp),%r9
   2a813:	jae    2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a819:	mov    0x10(%rsp),%rax
   2a81e:	mov    %rax,%rdi
   2a821:	mov    0x18(%rsp),%r8
   2a826:	or     %r8,%rdi
   2a829:	shr    $0x20,%rdi
   2a82d:	je     2a843 <verified_tiff::decode_lz_segment+0xe13>
   2a82f:	xor    %edx,%edx
   2a831:	div    %r8
   2a834:	mov    %rdx,%rax
   2a837:	mulq   0x20(%rsp)
   2a83c:	jno    2a855 <verified_tiff::decode_lz_segment+0xe25>
   2a83e:	jmp    2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a843:	xor    %edx,%edx
   2a845:	div    %r8d
   2a848:	mov    %edx,%eax
   2a84a:	mulq   0x20(%rsp)
   2a84f:	jo     2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a855:	mov    %rax,%r8
   2a858:	test   %rdi,%rdi
   2a85b:	je     2a872 <verified_tiff::decode_lz_segment+0xe42>
   2a85d:	mov    0x10(%rsp),%rax
   2a862:	xor    %edx,%edx
   2a864:	divq   0x18(%rsp)
   2a869:	mulq   0x40(%rsp)
   2a86e:	jno    2a884 <verified_tiff::decode_lz_segment+0xe54>
   2a870:	jmp    2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a872:	mov    0x10(%rsp),%rax
   2a877:	xor    %edx,%edx
   2a879:	divl   0x18(%rsp)
   2a87d:	mulq   0x40(%rsp)
   2a882:	jo     2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a884:	mov    %rax,%rdi
   2a887:	mov    0x28(%rsp),%rax
   2a88c:	sub    %r8,%rax
   2a88f:	jbe    2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a891:	cmp    %rax,%r9
   2a894:	jae    2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a896:	mov    0x50(%rsp),%rax
   2a89b:	sub    %rdi,%rax
   2a89e:	jbe    2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a8a0:	cmp    %rax,%rsi
   2a8a3:	jae    2a8e0 <verified_tiff::decode_lz_segment+0xeb0>
   2a8a5:	add    %rsi,%rdi
   2a8a8:	add    %r9,%r8
   2a8ab:	imul   0x28(%rsp),%rdi
   2a8b1:	add    %r8,%rdi
   2a8b4:	imul   0x38(%rsp),%rdi
   2a8ba:	cmp    0x48(%rsp),%rdi
   2a8bf:	jae    2aa50 <verified_tiff::decode_lz_segment+0x1020>
   2a8c5:	test   %r10d,0x8c(%rsp)
   2a8cd:	setne  %al
   2a8d0:	xor    0xe(%rsp),%al
   2a8d4:	and    $0x1,%al
   2a8d6:	neg    %al
   2a8d8:	mov    0x58(%rsp),%rdx
   2a8dd:	mov    %al,(%rdx,%rdi,1)
   2a8e0:	or     $0x7,%rcx
   2a8e4:	cmp    0x20(%rsp),%rcx
   2a8e9:	jae    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a8ef:	mov    0x10(%rsp),%rax
   2a8f4:	mov    %rax,%rdx
   2a8f7:	mov    0x18(%rsp),%rdi
   2a8fc:	or     %rdi,%rdx
   2a8ff:	shr    $0x20,%rdx
   2a903:	je     2a91c <verified_tiff::decode_lz_segment+0xeec>
   2a905:	xor    %edx,%edx
   2a907:	div    %rdi
   2a90a:	mov    %rax,%rdi
   2a90d:	mov    %rdx,%rax
   2a910:	mulq   0x20(%rsp)
   2a915:	jno    2a930 <verified_tiff::decode_lz_segment+0xf00>
   2a917:	jmp    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a91c:	xor    %edx,%edx
   2a91e:	div    %edi
   2a920:	mov    %eax,%edi
   2a922:	mov    %rdx,%rax
   2a925:	mulq   0x20(%rsp)
   2a92a:	jo     29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a930:	mov    %rax,%r8
   2a933:	mov    %rdi,%rax
   2a936:	mulq   0x40(%rsp)
   2a93b:	jo     29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a941:	mov    %rax,%rdi
   2a944:	mov    0x28(%rsp),%rax
   2a949:	sub    %r8,%rax
   2a94c:	jbe    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a952:	cmp    %rax,%rcx
   2a955:	jae    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a95b:	mov    0x50(%rsp),%rax
   2a960:	sub    %rdi,%rax
   2a963:	jbe    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a969:	cmp    %rax,%rsi
   2a96c:	jae    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a972:	add    %rsi,%rdi
   2a975:	add    %rcx,%r8
   2a978:	imul   0x28(%rsp),%rdi
   2a97e:	add    %r8,%rdi
   2a981:	imul   0x38(%rsp),%rdi
   2a987:	cmp    0x48(%rsp),%rdi
   2a98c:	jae    2aa50 <verified_tiff::decode_lz_segment+0x1020>
   2a992:	test   %r10d,0x80(%rsp)
   2a99a:	setne  %al
   2a99d:	xor    0xe(%rsp),%al
   2a9a1:	and    $0x1,%al
   2a9a3:	neg    %al
   2a9a5:	mov    0x58(%rsp),%rcx
   2a9aa:	mov    %al,(%rcx,%rdi,1)
   2a9ad:	jmp    29e00 <verified_tiff::decode_lz_segment+0x3d0>
   2a9b2:	mov    $0xff,%al
   2a9b4:	add    $0x7148,%rsp
   2a9bb:	pop    %rbx
   2a9bc:	pop    %r12
   2a9be:	pop    %r13
   2a9c0:	pop    %r14
   2a9c2:	pop    %r15
   2a9c4:	pop    %rbp
   2a9c5:	ret
   2a9c6:	lea    0x1(%r12),%rdi
   2a9cb:	cmp    %r14,%rdi
   2a9ce:	jae    2aa8f <verified_tiff::decode_lz_segment+0x105f>
   2a9d4:	mov    0x70(%rsp),%rax
   2a9d9:	movzbl (%rax,%rdi,1),%eax
   2a9dd:	and    $0x1,%al
   2a9df:	jmp    29bd5 <verified_tiff::decode_lz_segment+0x1a5>
   2a9e4:	mov    $0x3,%al
   2a9e6:	jmp    2a9b4 <verified_tiff::decode_lz_segment+0xf84>
   2a9e8:	lea    0x621c9(%rip),%rdx        # 8cbb8 <anon.35f1884359c0d9048fdad1d4cf8af759.24.llvm.11558219987766828904+0x60>
   2a9ef:	mov    $0x1000,%esi
   2a9f4:	call   *0x65a46(%rip)        # 90440 <_DYNAMIC+0x380>
   2a9fa:	lea    0x6219f(%rip),%rdx        # 8cba0 <anon.35f1884359c0d9048fdad1d4cf8af759.24.llvm.11558219987766828904+0x48>
   2aa01:	mov    $0x1000,%esi
   2aa06:	mov    %rbp,%rdi
   2aa09:	call   *0x65a31(%rip)        # 90440 <_DYNAMIC+0x380>
   2aa0f:	lea    0x62172(%rip),%rdx        # 8cb88 <anon.35f1884359c0d9048fdad1d4cf8af759.24.llvm.11558219987766828904+0x30>
   2aa16:	mov    $0x1000,%esi
   2aa1b:	mov    %rbp,%rdi
   2aa1e:	call   *0x65a1c(%rip)        # 90440 <_DYNAMIC+0x380>
   2aa24:	lea    0x6203d(%rip),%rdi        # 8ca68 <anon.35f1884359c0d9048fdad1d4cf8af759.1.llvm.11558219987766828904+0x120>
   2aa2b:	call   *0x65a1f(%rip)        # 90450 <_DYNAMIC+0x390>
   2aa31:	lea    0x62198(%rip),%rdx        # 8cbd0 <anon.35f1884359c0d9048fdad1d4cf8af759.24.llvm.11558219987766828904+0x78>
   2aa38:	mov    $0x1000,%esi
   2aa3d:	call   *0x659fd(%rip)        # 90440 <_DYNAMIC+0x380>
   2aa43:	lea    0x6204e(%rip),%rdi        # 8ca98 <anon.35f1884359c0d9048fdad1d4cf8af759.1.llvm.11558219987766828904+0x150>
   2aa4a:	call   *0x659f8(%rip)        # 90448 <_DYNAMIC+0x388>
   2aa50:	lea    0x61f39(%rip),%rdx        # 8c990 <anon.35f1884359c0d9048fdad1d4cf8af759.1.llvm.11558219987766828904+0x48>
   2aa57:	mov    0x48(%rsp),%rsi
   2aa5c:	call   *0x659de(%rip)        # 90440 <_DYNAMIC+0x380>
   2aa62:	lea    0x62017(%rip),%rdi        # 8ca80 <anon.35f1884359c0d9048fdad1d4cf8af759.1.llvm.11558219987766828904+0x138>
   2aa69:	call   *0x659e1(%rip)        # 90450 <_DYNAMIC+0x390>
   2aa6f:	lea    0x61eea(%rip),%rdx        # 8c960 <anon.35f1884359c0d9048fdad1d4cf8af759.1.llvm.11558219987766828904+0x18>
   2aa76:	mov    %r12,%rdi
   2aa79:	mov    %r14,%rsi
   2aa7c:	call   *0x659be(%rip)        # 90440 <_DYNAMIC+0x380>
   2aa82:	lea    0x62027(%rip),%rdi        # 8cab0 <anon.35f1884359c0d9048fdad1d4cf8af759.1.llvm.11558219987766828904+0x168>
   2aa89:	call   *0x659b9(%rip)        # 90448 <_DYNAMIC+0x388>
   2aa8f:	lea    0x61ee2(%rip),%rdx        # 8c978 <anon.35f1884359c0d9048fdad1d4cf8af759.1.llvm.11558219987766828904+0x30>
   2aa96:	mov    %r14,%rsi
   2aa99:	call   *0x659a1(%rip)        # 90440 <_DYNAMIC+0x380>
   2aa9f:	int3