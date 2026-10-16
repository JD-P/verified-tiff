
target/compat/libtiff.so.6:     file format elf64-x86-64


Disassembly of section .text:

0000000000052f90 <verified_tiff_capi::lerc::pack::<4>>:
   52f90:	55                   	push   %rbp
   52f91:	41 57                	push   %r15
   52f93:	41 56                	push   %r14
   52f95:	41 55                	push   %r13
   52f97:	41 54                	push   %r12
   52f99:	53                   	push   %rbx
   52f9a:	48 83 ec 48          	sub    $0x48,%rsp
   52f9e:	48 89 f5             	mov    %rsi,%rbp
   52fa1:	48 89 7c 24 08       	mov    %rdi,0x8(%rsp)
   52fa6:	0f b6 84 24 a0 00 00 	movzbl 0xa0(%rsp),%eax
   52fad:	00 
   52fae:	0a 84 24 a8 00 00 00 	or     0xa8(%rsp),%al
   52fb5:	3c 01                	cmp    $0x1,%al
   52fb7:	74 1a                	je     52fd3 <verified_tiff_capi::lerc::pack::<4>+0x43>
   52fb9:	31 c0                	xor    %eax,%eax
   52fbb:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   52fc0:	48 39 c1             	cmp    %rax,%rcx
   52fc3:	0f 84 ff 02 00 00    	je     532c8 <verified_tiff_capi::lerc::pack::<4>+0x338>
   52fc9:	80 3c 02 ff          	cmpb   $0xff,(%rdx,%rax,1)
   52fcd:	48 8d 40 01          	lea    0x1(%rax),%rax
   52fd1:	74 ed                	je     52fc0 <verified_tiff_capi::lerc::pack::<4>+0x30>
   52fd3:	48 8b 9c 24 80 00 00 	mov    0x80(%rsp),%rbx
   52fda:	00 
   52fdb:	48 85 db             	test   %rbx,%rbx
   52fde:	0f 84 66 04 00 00    	je     5344a <verified_tiff_capi::lerc::pack::<4>+0x4ba>
   52fe4:	4c 8b b4 24 90 00 00 	mov    0x90(%rsp),%r14
   52feb:	00 
   52fec:	4c 8b ac 24 88 00 00 	mov    0x88(%rsp),%r13
   52ff3:	00 
   52ff4:	4d 85 ed             	test   %r13,%r13
   52ff7:	0f 84 f0 02 00 00    	je     532ed <verified_tiff_capi::lerc::pack::<4>+0x35d>
   52ffd:	4a 8d 04 b5 00 00 00 	lea    0x0(,%r14,4),%rax
   53004:	00 
   53005:	48 89 44 24 20       	mov    %rax,0x20(%rsp)
   5300a:	4c 8b 64 24 08       	mov    0x8(%rsp),%r12
   5300f:	49 8d 44 24 18       	lea    0x18(%r12),%rax
   53014:	48 89 44 24 18       	mov    %rax,0x18(%rsp)
   53019:	45 31 f6             	xor    %r14d,%r14d
   5301c:	31 db                	xor    %ebx,%ebx
   5301e:	48 c7 44 24 10 00 00 	movq   $0x0,0x10(%rsp)
   53025:	00 00 
   53027:	45 31 ff             	xor    %r15d,%r15d
   5302a:	48 89 4c 24 30       	mov    %rcx,0x30(%rsp)
   5302f:	48 89 54 24 28       	mov    %rdx,0x28(%rsp)
   53034:	eb 47                	jmp    5307d <verified_tiff_capi::lerc::pack::<4>+0xed>
   53036:	66 2e 0f 1f 84 00 00 	cs nopw 0x0(%rax,%rax,1)
   5303d:	00 00 00 
   53040:	f6 5c 24 07          	negb   0x7(%rsp)
   53044:	b8 00 00 00 00       	mov    $0x0,%eax
   53049:	19 c0                	sbb    %eax,%eax
   5304b:	41 88 04 38          	mov    %al,(%r8,%rdi,1)
   5304f:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
   53054:	48 03 44 24 20       	add    0x20(%rsp),%rax
   53059:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
   5305e:	4d 01 ee             	add    %r13,%r14
   53061:	4a 8d 04 ed 00 00 00 	lea    0x0(,%r13,8),%rax
   53068:	00 
   53069:	48 01 44 24 18       	add    %rax,0x18(%rsp)
   5306e:	4c 29 eb             	sub    %r13,%rbx
   53071:	49 01 c4             	add    %rax,%r12
   53074:	4d 39 d7             	cmp    %r10,%r15
   53077:	0f 84 cd 03 00 00    	je     5344a <verified_tiff_capi::lerc::pack::<4>+0x4ba>
   5307d:	4d 89 ea             	mov    %r13,%r10
   53080:	4c 39 f5             	cmp    %r14,%rbp
   53083:	4c 89 f0             	mov    %r14,%rax
   53086:	48 0f 47 c5          	cmova  %rbp,%rax
   5308a:	48 01 d8             	add    %rbx,%rax
   5308d:	4d 8d 5d ff          	lea    -0x1(%r13),%r11
   53091:	4c 39 d8             	cmp    %r11,%rax
   53094:	49 0f 43 c3          	cmovae %r11,%rax
   53098:	4d 0f af d7          	imul   %r15,%r10
   5309c:	48 89 ee             	mov    %rbp,%rsi
   5309f:	4c 29 d6             	sub    %r10,%rsi
   530a2:	4c 89 ff             	mov    %r15,%rdi
   530a5:	41 bf 00 00 00 00    	mov    $0x0,%r15d
   530ab:	4c 0f 43 fe          	cmovae %rsi,%r15
   530af:	4d 39 df             	cmp    %r11,%r15
   530b2:	4d 0f 43 fb          	cmovae %r11,%r15
   530b6:	49 89 fb             	mov    %rdi,%r11
   530b9:	49 c1 eb 03          	shr    $0x3,%r11
   530bd:	49 39 cb             	cmp    %rcx,%r11
   530c0:	0f 83 47 03 00 00    	jae    5340d <verified_tiff_capi::lerc::pack::<4>+0x47d>
   530c6:	48 89 5c 24 40       	mov    %rbx,0x40(%rsp)
   530cb:	89 f9                	mov    %edi,%ecx
   530cd:	f6 d1                	not    %cl
   530cf:	80 e1 07             	and    $0x7,%cl
   530d2:	40 b6 01             	mov    $0x1,%sil
   530d5:	40 d2 e6             	shl    %cl,%sil
   530d8:	48 89 7c 24 38       	mov    %rdi,0x38(%rsp)
   530dd:	48 0f af bc 24 90 00 	imul   0x90(%rsp),%rdi
   530e4:	00 00 
   530e6:	42 22 34 1a          	and    (%rdx,%r11,1),%sil
   530ea:	40 88 74 24 07       	mov    %sil,0x7(%rsp)
   530ef:	74 5f                	je     53150 <verified_tiff_capi::lerc::pack::<4>+0x1c0>
   530f1:	31 c0                	xor    %eax,%eax
   530f3:	0f b6 94 24 a0 00 00 	movzbl 0xa0(%rsp),%edx
   530fa:	00 
   530fb:	0f 1f 44 00 00       	nopl   0x0(%rax,%rax,1)
   53100:	4d 8d 14 06          	lea    (%r14,%rax,1),%r10
   53104:	49 39 ea             	cmp    %rbp,%r10
   53107:	0f 83 ed 02 00 00    	jae    533fa <verified_tiff_capi::lerc::pack::<4>+0x46a>
   5310d:	84 d2                	test   %dl,%dl
   5310f:	48 89 c1             	mov    %rax,%rcx
   53112:	48 0f 45 8c 24 98 00 	cmovne 0x98(%rsp),%rcx
   53119:	00 00 
   5311b:	48 01 f9             	add    %rdi,%rcx
   5311e:	4c 8d 14 8d 03 00 00 	lea    0x3(,%rcx,4),%r10
   53125:	00 
   53126:	48 c1 e1 02          	shl    $0x2,%rcx
   5312a:	4d 39 ca             	cmp    %r9,%r10
   5312d:	0f 83 11 02 00 00    	jae    53344 <verified_tiff_capi::lerc::pack::<4>+0x3b4>
   53133:	45 8b 14 c4          	mov    (%r12,%rax,8),%r10d
   53137:	48 ff c0             	inc    %rax
   5313a:	45 89 14 08          	mov    %r10d,(%r8,%rcx,1)
   5313e:	49 39 c5             	cmp    %rax,%r13
   53141:	75 bd                	jne    53100 <verified_tiff_capi::lerc::pack::<4>+0x170>
   53143:	e9 28 01 00 00       	jmp    53270 <verified_tiff_capi::lerc::pack::<4>+0x2e0>
   53148:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   5314f:	00 
   53150:	4d 89 eb             	mov    %r13,%r11
   53153:	4c 89 f1             	mov    %r14,%rcx
   53156:	48 8b 5c 24 10       	mov    0x10(%rsp),%rbx
   5315b:	80 bc 24 a0 00 00 00 	cmpb   $0x0,0xa0(%rsp)
   53162:	00 
   53163:	48 8b b4 24 b0 00 00 	mov    0xb0(%rsp),%rsi
   5316a:	00 
   5316b:	74 33                	je     531a0 <verified_tiff_capi::lerc::pack::<4>+0x210>
   5316d:	48 03 bc 24 98 00 00 	add    0x98(%rsp),%rdi
   53174:	00 
   53175:	48 8d 0c bd 03 00 00 	lea    0x3(,%rdi,4),%rcx
   5317c:	00 
   5317d:	48 c1 e7 02          	shl    $0x2,%rdi
   53181:	4c 39 c9             	cmp    %r9,%rcx
   53184:	0f 83 57 02 00 00    	jae    533e1 <verified_tiff_capi::lerc::pack::<4>+0x451>
   5318a:	48 83 fe 06          	cmp    $0x6,%rsi
   5318e:	74 63                	je     531f3 <verified_tiff_capi::lerc::pack::<4>+0x263>
   53190:	48 83 fe 07          	cmp    $0x7,%rsi
   53194:	75 68                	jne    531fe <verified_tiff_capi::lerc::pack::<4>+0x26e>
   53196:	31 c0                	xor    %eax,%eax
   53198:	eb 5e                	jmp    531f8 <verified_tiff_capi::lerc::pack::<4>+0x268>
   5319a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   531a0:	48 83 fe 06          	cmp    $0x6,%rsi
   531a4:	74 0a                	je     531b0 <verified_tiff_capi::lerc::pack::<4>+0x220>
   531a6:	48 83 fe 07          	cmp    $0x7,%rsi
   531aa:	75 14                	jne    531c0 <verified_tiff_capi::lerc::pack::<4>+0x230>
   531ac:	31 c0                	xor    %eax,%eax
   531ae:	eb 21                	jmp    531d1 <verified_tiff_capi::lerc::pack::<4>+0x241>
   531b0:	b8 00 00 c0 7f       	mov    $0x7fc00000,%eax
   531b5:	eb 1a                	jmp    531d1 <verified_tiff_capi::lerc::pack::<4>+0x241>
   531b7:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   531be:	00 00 
   531c0:	48 39 e9             	cmp    %rbp,%rcx
   531c3:	0f 83 57 02 00 00    	jae    53420 <verified_tiff_capi::lerc::pack::<4>+0x490>
   531c9:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
   531ce:	8b 04 c8             	mov    (%rax,%rcx,8),%eax
   531d1:	48 8d 7b 03          	lea    0x3(%rbx),%rdi
   531d5:	4c 39 cf             	cmp    %r9,%rdi
   531d8:	0f 83 ec 01 00 00    	jae    533ca <verified_tiff_capi::lerc::pack::<4>+0x43a>
   531de:	41 89 04 18          	mov    %eax,(%r8,%rbx,1)
   531e2:	48 83 c3 04          	add    $0x4,%rbx
   531e6:	48 ff c1             	inc    %rcx
   531e9:	49 ff cb             	dec    %r11
   531ec:	75 b2                	jne    531a0 <verified_tiff_capi::lerc::pack::<4>+0x210>
   531ee:	e9 7d 00 00 00       	jmp    53270 <verified_tiff_capi::lerc::pack::<4>+0x2e0>
   531f3:	b8 00 00 c0 7f       	mov    $0x7fc00000,%eax
   531f8:	41 89 04 38          	mov    %eax,(%r8,%rdi,1)
   531fc:	eb 72                	jmp    53270 <verified_tiff_capi::lerc::pack::<4>+0x2e0>
   531fe:	49 ff c7             	inc    %r15
   53201:	49 83 ff 05          	cmp    $0x5,%r15
   53205:	73 05                	jae    5320c <verified_tiff_capi::lerc::pack::<4>+0x27c>
   53207:	45 31 ff             	xor    %r15d,%r15d
   5320a:	eb 44                	jmp    53250 <verified_tiff_capi::lerc::pack::<4>+0x2c0>
   5320c:	44 89 f9             	mov    %r15d,%ecx
   5320f:	83 e1 03             	and    $0x3,%ecx
   53212:	ba 04 00 00 00       	mov    $0x4,%edx
   53217:	48 0f 44 ca          	cmove  %rdx,%rcx
   5321b:	49 29 cf             	sub    %rcx,%r15
   5321e:	48 29 c8             	sub    %rcx,%rax
   53221:	48 ff c0             	inc    %rax
   53224:	45 31 d2             	xor    %r10d,%r10d
   53227:	66 0f 1f 84 00 00 00 	nopw   0x0(%rax,%rax,1)
   5322e:	00 00 
   53230:	4c 89 d1             	mov    %r10,%rcx
   53233:	49 83 c2 04          	add    $0x4,%r10
   53237:	4c 39 d0             	cmp    %r10,%rax
   5323a:	75 f4                	jne    53230 <verified_tiff_capi::lerc::pack::<4>+0x2a0>
   5323c:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   53241:	8b 04 c8             	mov    (%rax,%rcx,8),%eax
   53244:	41 89 04 38          	mov    %eax,(%r8,%rdi,1)
   53248:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
   5324f:	00 
   53250:	4f 8d 14 3e          	lea    (%r14,%r15,1),%r10
   53254:	49 39 ea             	cmp    %rbp,%r10
   53257:	0f 83 9d 01 00 00    	jae    533fa <verified_tiff_capi::lerc::pack::<4>+0x46a>
   5325d:	43 8b 04 fc          	mov    (%r12,%r15,8),%eax
   53261:	49 ff c7             	inc    %r15
   53264:	41 89 04 38          	mov    %eax,(%r8,%rdi,1)
   53268:	4d 39 fd             	cmp    %r15,%r13
   5326b:	75 e3                	jne    53250 <verified_tiff_capi::lerc::pack::<4>+0x2c0>
   5326d:	0f 1f 00             	nopl   (%rax)
   53270:	4c 8b 7c 24 38       	mov    0x38(%rsp),%r15
   53275:	49 ff c7             	inc    %r15
   53278:	80 bc 24 a8 00 00 00 	cmpb   $0x0,0xa8(%rsp)
   5327f:	00 
   53280:	48 8b 4c 24 30       	mov    0x30(%rsp),%rcx
   53285:	48 8b 54 24 28       	mov    0x28(%rsp),%rdx
   5328a:	4c 8b 94 24 80 00 00 	mov    0x80(%rsp),%r10
   53291:	00 
   53292:	48 8b 84 24 90 00 00 	mov    0x90(%rsp),%rax
   53299:	00 
   5329a:	48 8b 5c 24 40       	mov    0x40(%rsp),%rbx
   5329f:	0f 84 aa fd ff ff    	je     5304f <verified_tiff_capi::lerc::pack::<4>+0xbf>
   532a5:	4c 89 ff             	mov    %r15,%rdi
   532a8:	48 0f af f8          	imul   %rax,%rdi
   532ac:	48 ff cf             	dec    %rdi
   532af:	4c 39 cf             	cmp    %r9,%rdi
   532b2:	0f 82 88 fd ff ff    	jb     53040 <verified_tiff_capi::lerc::pack::<4>+0xb0>
   532b8:	48 8d 15 01 c9 13 00 	lea    0x13c901(%rip),%rdx        # 18fbc0 <anon.dc6fc3f1ea759549b0bfce82fee17fc2.90.llvm.7637344445459145111+0x180>
   532bf:	4c 89 ce             	mov    %r9,%rsi
   532c2:	ff 15 f8 63 14 00    	call   *0x1463f8(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   532c8:	49 c1 e9 02          	shr    $0x2,%r9
   532cc:	4c 39 cd             	cmp    %r9,%rbp
   532cf:	4c 0f 42 cd          	cmovb  %rbp,%r9
   532d3:	4d 85 c9             	test   %r9,%r9
   532d6:	0f 84 6e 01 00 00    	je     5344a <verified_tiff_capi::lerc::pack::<4>+0x4ba>
   532dc:	49 83 f9 04          	cmp    $0x4,%r9
   532e0:	0f 83 97 00 00 00    	jae    5337d <verified_tiff_capi::lerc::pack::<4>+0x3ed>
   532e6:	31 c0                	xor    %eax,%eax
   532e8:	e9 49 01 00 00       	jmp    53436 <verified_tiff_capi::lerc::pack::<4>+0x4a6>
   532ed:	80 bc 24 a8 00 00 00 	cmpb   $0x0,0xa8(%rsp)
   532f4:	00 
   532f5:	74 64                	je     5335b <verified_tiff_capi::lerc::pack::<4>+0x3cb>
   532f7:	49 8d 7e ff          	lea    -0x1(%r14),%rdi
   532fb:	31 c0                	xor    %eax,%eax
   532fd:	0f 1f 00             	nopl   (%rax)
   53300:	49 89 c3             	mov    %rax,%r11
   53303:	49 c1 eb 03          	shr    $0x3,%r11
   53307:	49 39 cb             	cmp    %rcx,%r11
   5330a:	0f 83 fd 00 00 00    	jae    5340d <verified_tiff_capi::lerc::pack::<4>+0x47d>
   53310:	4c 39 cf             	cmp    %r9,%rdi
   53313:	73 a3                	jae    532b8 <verified_tiff_capi::lerc::pack::<4>+0x328>
   53315:	4c 8d 50 01          	lea    0x1(%rax),%r10
   53319:	f6 d0                	not    %al
   5331b:	42 0f b6 34 1a       	movzbl (%rdx,%r11,1),%esi
   53320:	0f b6 c0             	movzbl %al,%eax
   53323:	83 e0 07             	and    $0x7,%eax
   53326:	0f a3 c6             	bt     %eax,%esi
   53329:	b8 00 00 00 00       	mov    $0x0,%eax
   5332e:	19 c0                	sbb    %eax,%eax
   53330:	41 88 04 38          	mov    %al,(%r8,%rdi,1)
   53334:	4c 01 f7             	add    %r14,%rdi
   53337:	4c 89 d0             	mov    %r10,%rax
   5333a:	4c 39 d3             	cmp    %r10,%rbx
   5333d:	75 c1                	jne    53300 <verified_tiff_capi::lerc::pack::<4>+0x370>
   5333f:	e9 06 01 00 00       	jmp    5344a <verified_tiff_capi::lerc::pack::<4>+0x4ba>
   53344:	48 89 cf             	mov    %rcx,%rdi
   53347:	48 8d 77 04          	lea    0x4(%rdi),%rsi
   5334b:	48 8d 0d 9e c8 13 00 	lea    0x13c89e(%rip),%rcx        # 18fbf0 <anon.dc6fc3f1ea759549b0bfce82fee17fc2.90.llvm.7637344445459145111+0x1b0>
   53352:	4c 89 ca             	mov    %r9,%rdx
   53355:	ff 15 1d 61 14 00    	call   *0x14611d(%rip)        # 199478 <_GLOBAL_OFFSET_TABLE_+0x918>
   5335b:	31 c0                	xor    %eax,%eax
   5335d:	0f 1f 00             	nopl   (%rax)
   53360:	49 89 c3             	mov    %rax,%r11
   53363:	49 c1 eb 03          	shr    $0x3,%r11
   53367:	49 39 cb             	cmp    %rcx,%r11
   5336a:	0f 83 9d 00 00 00    	jae    5340d <verified_tiff_capi::lerc::pack::<4>+0x47d>
   53370:	48 ff c0             	inc    %rax
   53373:	48 39 c3             	cmp    %rax,%rbx
   53376:	75 e8                	jne    53360 <verified_tiff_capi::lerc::pack::<4>+0x3d0>
   53378:	e9 cd 00 00 00       	jmp    5344a <verified_tiff_capi::lerc::pack::<4>+0x4ba>
   5337d:	48 b8 fc ff ff ff ff 	movabs $0xffffffffffffffc,%rax
   53384:	ff ff 0f 
   53387:	4c 21 c8             	and    %r9,%rax
   5338a:	4b 8d 0c 09          	lea    (%r9,%r9,1),%rcx
   5338e:	48 83 e1 f8          	and    $0xfffffffffffffff8,%rcx
   53392:	31 d2                	xor    %edx,%edx
   53394:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
   53399:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
   533a0:	f3 0f 6f 04 96       	movdqu (%rsi,%rdx,4),%xmm0
   533a5:	f3 0f 6f 4c 96 10    	movdqu 0x10(%rsi,%rdx,4),%xmm1
   533ab:	66 0f 70 c0 e8       	pshufd $0xe8,%xmm0,%xmm0
   533b0:	66 0f 70 c9 e8       	pshufd $0xe8,%xmm1,%xmm1
   533b5:	66 0f 6c c1          	punpcklqdq %xmm1,%xmm0
   533b9:	f3 41 0f 7f 04 50    	movdqu %xmm0,(%r8,%rdx,2)
   533bf:	48 83 c2 08          	add    $0x8,%rdx
   533c3:	48 39 d1             	cmp    %rdx,%rcx
   533c6:	75 d8                	jne    533a0 <verified_tiff_capi::lerc::pack::<4>+0x410>
   533c8:	eb 7b                	jmp    53445 <verified_tiff_capi::lerc::pack::<4>+0x4b5>
   533ca:	48 89 df             	mov    %rbx,%rdi
   533cd:	48 8d 77 04          	lea    0x4(%rdi),%rsi
   533d1:	48 8d 0d 18 c8 13 00 	lea    0x13c818(%rip),%rcx        # 18fbf0 <anon.dc6fc3f1ea759549b0bfce82fee17fc2.90.llvm.7637344445459145111+0x1b0>
   533d8:	4c 89 ca             	mov    %r9,%rdx
   533db:	ff 15 97 60 14 00    	call   *0x146097(%rip)        # 199478 <_GLOBAL_OFFSET_TABLE_+0x918>
   533e1:	48 83 e6 fe          	and    $0xfffffffffffffffe,%rsi
   533e5:	48 83 fe 06          	cmp    $0x6,%rsi
   533e9:	0f 94 c0             	sete   %al
   533ec:	49 39 ea             	cmp    %rbp,%r10
   533ef:	0f 92 c1             	setb   %cl
   533f2:	08 c1                	or     %al,%cl
   533f4:	0f 85 4d ff ff ff    	jne    53347 <verified_tiff_capi::lerc::pack::<4>+0x3b7>
   533fa:	48 8d 15 d7 c7 13 00 	lea    0x13c7d7(%rip),%rdx        # 18fbd8 <anon.dc6fc3f1ea759549b0bfce82fee17fc2.90.llvm.7637344445459145111+0x198>
   53401:	4c 89 d7             	mov    %r10,%rdi
   53404:	48 89 ee             	mov    %rbp,%rsi
   53407:	ff 15 b3 62 14 00    	call   *0x1462b3(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   5340d:	48 8d 15 94 c7 13 00 	lea    0x13c794(%rip),%rdx        # 18fba8 <anon.dc6fc3f1ea759549b0bfce82fee17fc2.90.llvm.7637344445459145111+0x168>
   53414:	4c 89 df             	mov    %r11,%rdi
   53417:	48 89 ce             	mov    %rcx,%rsi
   5341a:	ff 15 a0 62 14 00    	call   *0x1462a0(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   53420:	49 89 ca             	mov    %rcx,%r10
   53423:	48 8d 15 ae c7 13 00 	lea    0x13c7ae(%rip),%rdx        # 18fbd8 <anon.dc6fc3f1ea759549b0bfce82fee17fc2.90.llvm.7637344445459145111+0x198>
   5342a:	4c 89 d7             	mov    %r10,%rdi
   5342d:	48 89 ee             	mov    %rbp,%rsi
   53430:	ff 15 8a 62 14 00    	call   *0x14628a(%rip)        # 1996c0 <_GLOBAL_OFFSET_TABLE_+0xb60>
   53436:	48 8b 4c 24 08       	mov    0x8(%rsp),%rcx
   5343b:	8b 0c c1             	mov    (%rcx,%rax,8),%ecx
   5343e:	41 89 0c 80          	mov    %ecx,(%r8,%rax,4)
   53442:	48 ff c0             	inc    %rax
   53445:	49 39 c1             	cmp    %rax,%r9
   53448:	75 ec                	jne    53436 <verified_tiff_capi::lerc::pack::<4>+0x4a6>
   5344a:	48 83 c4 48          	add    $0x48,%rsp
   5344e:	5b                   	pop    %rbx
   5344f:	41 5c                	pop    %r12
   53451:	41 5d                	pop    %r13
   53453:	41 5e                	pop    %r14
   53455:	41 5f                	pop    %r15
   53457:	5d                   	pop    %rbp
   53458:	c3                   	ret
