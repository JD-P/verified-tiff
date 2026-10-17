#!/usr/bin/env python3
"""Summarize the stepwise run-optimization benchmark as Markdown tables (--readme: README.md's table)."""
import json,math,pathlib,sys
ROOT=pathlib.Path(__file__).resolve().parents[1]
DECODE=['native','scan','rgba','crop']
FAX=['fax-rle','fax-g3','fax-g4','original-testfax4.tiff']
BILEVEL=FAX+['original-lzw-single-strip.tiff']
P3=['float32-deflate-p3','float16-deflate-p3']
name=lambda r:r['case']['name']
lzw=lambda r:'lzw' in name(r)
deflate=lambda r:'deflate' in name(r)
jpeg=lambda r:name(r).startswith('jpeg-')
lossy=lambda r:name(r).startswith('webp-lossy')
lossless=lambda r:name(r).startswith('webp-lossless')
# Workloads each commit targets; every commit is still timed on all 96.
# The final commit is measured as `after`.
STEPS=[('jcc','Jump padding',lambda r:True),
 ('webp','VP8L ordered symbol table',lossless),
 ('deflate','Deflate exit-free runs',deflate),
 ('strips','Fax/JPEG strip copies',lambda r:name(r) in FAX[1:]+['jpeg-smooth','jpeg-texture']),
 ('fax','Fax cursor and row fill',lambda r:name(r) in FAX),
 ('jpeg','JPEG constant divisors',jpeg),
 ('bilevel','Adapter bilevel packing',lambda r:name(r) in BILEVEL),
 ('rgba','RGBA row stores',lambda r:r['mode']=='rgba'),
 ('packbits','PackBits run copies',lambda r:'packbits' in name(r)),
 ('lzw','One-pass LZW',lzw),
 ('lzwstream','Whole LZW segments',lzw),
 ('rgbarows','Check-free RGBA rows',lambda r:r['mode']=='rgba'),
 ('lookahead','JPEG Huffman lookahead',jpeg),
 ('render','JPEG one-pixel render',jpeg),
 ('onepass','One-pass JPEG',jpeg),
 ('packed','Packed RGBA runs',lambda r:name(r) in BILEVEL and r['mode']=='rgba'),
 ('phrases','Streamed LZW phrases',lzw),
 ('spans','CMYK/Lab row spans',lambda r:name(r) in ['cmyk-deflate','lab-deflate']),
 ('predict','Division-free prediction',lambda r:name(r) in P3+['gray16-deflate-p2']),
 ('vp8bool','Inlined VP8 bool decoder',lossy),
 ('unshuffle','Float unshuffle per sample',lambda r:name(r) in P3),
 ('pack8','Word-wide bilevel packing',lambda r:name(r) in BILEVEL),
 ('vp8filter','Inlined VP8 loop filter',lossy),
 ('prefix','WebP 8-bit prefix table',lossless),
 ('storedslice','Stored-block subslices',deflate),
 ('matches','Whole-match slice copies',deflate),
 ('stored','Whole stored-run copies',deflate),
 ('after','Chunked LZW phrase copies',lzw)]
TARGETS={k:f for k,_,f in STEPS};TITLES={k:t for k,t,_ in STEPS}
def gm(xs):xs=list(xs);return math.exp(sum(map(math.log,xs))/len(xs))
def table(h,rows):return '| '+' | '.join(h)+' |\n| '+' | '.join(['---']+['---:']*(len(h)-1))+' |\n'+''.join('| '+' | '.join(r)+' |\n' for r in rows)
def readme(rs):
 """Final-reader table in README.md's format."""
 ms=lambda x:f"{x:#.4g}" if x<1 else f"{x:.3f}"
 ratio=lambda x:f"{x:.2f}×" if x>=0.1 else f"{x:.2g}×"
 print('| Fixture | Task | Ours ms | libtiff ms | Ratio |\n|---|---|---:|---:|---:|')
 for r in rs:
  s=r['statistics'];print(f"| {r['case']['name']} | {r['mode']} | {ms(s['after']['median_ms'])} | {ms(s['libtiff']['median_ms'])} | {ratio(r['vs_libtiff'])} |")
def main():
 args=[a for a in sys.argv[1:] if a!='--readme']
 path=pathlib.Path(args[0]) if args else ROOT/'docs/evidence/run-optimization/results.json'
 d=json.loads(path.read_text());rs=d['records'];chain=d['metadata']['chain']
 if '--readme' in sys.argv:return readme(rs)
 med=lambda r,e:r['statistics'][e]['median_ms']
 label=lambda r:r['case']['label']+' / '+r['mode']
 decode=[r for r in rs if r['mode'] in DECODE]
 out=[]
 out.append(f"Across {len(decode)} decode workloads (native, scan, RGBA and crop reads), the geometric-mean speedup is "
            f"**{gm(r['speedup'] for r in decode):.2f}×**. The reader now takes {gm(r['vs_libtiff'] for r in decode):.2f}× as long as "
            f"libtiff 4.7.1, against {gm(med(r,'before')/med(r,'libtiff') for r in decode):.2f}× before. All {len(rs)} workloads produce "
            f"byte-identical output at every commit, including JPEG; {sum(r['difference']['exact'] for r in rs)} also match libtiff exactly.")
 # Deterministic callgrind counts (one decode per engine) pick each row's
 # example, so timing noise cannot select it.
 counts=json.loads((path.parent/'instructions.json').read_text())['counts']
 rows=[]
 for a,b in zip(chain,chain[1:]):
  t=[r for r in rs if TARGETS[b](r)]
  if not t:continue
  ir=lambda r:counts[a+'|'+r['case']['file']+'|'+r['mode']]/counts[b+'|'+r['case']['file']+'|'+r['mode']]
  # A commit that adds instructions (constant JPEG divisors) shows its fastest workload instead.
  fewer=b!='jcc' and gm(map(ir,t))>=1
  best=max(t,key=ir if fewer else lambda r:r['step_speedup'][b]);cut='—' if b=='jcc' else f"{gm(map(ir,t)):.2f}×"
  note='' if b=='jcc' else f", {ir(best):.2f}× fewer instructions" if fewer else f", {1/ir(best):.2f}× the instructions"
  rows.append([TITLES[b],str(len(t)),f"{gm(r['step_speedup'][b] for r in t):.2f}×",cut,f"{gm(r['step_speedup'][b] for r in decode):.2f}×",
               f"{label(best)}: {med(best,a):.2f} → {med(best,b):.2f} ms"+note])
 out.append(table(['Commit','Targeted workloads','Targeted time','Targeted instructions','All decode time','Largest instruction cut'],rows))
 picks=[('webp-lossless-texture','native'),('webp-lossless-opaque','native'),('webp-lossless','native'),('webp-lossy-texture','native'),
        ('webp-lossy-opaque','native'),('noise-deflate','native'),('rgb-deflate','native'),('large-strip-deflate','crop'),
        ('lerc-float-deflate','native'),('cmyk-deflate','rgba'),('gray16-deflate-p2','native'),('float32-deflate-p3','native'),
        ('fax-g4','native'),('original-testfax4.tiff','native'),('fax-rle','native'),('jpeg-smooth','native'),('jpeg-texture','native'),
        ('gray-none','rgba'),('rgb-none','rgba'),('gray-packbits','native'),('gray-lzw','native'),('rgb-lzw','native'),
        ('rgb-tiled-lzw','native'),('original-lzw-single-strip.tiff','native')]
 find=lambda n,m:next((r for r in rs if r['case']['name']==n and r['mode']==m),None)
 out.append(table(['Workload','Before ms','After ms','libtiff 4.7.1 ms','Speedup','After / libtiff'],
  [[label(r),f"{med(r,'before'):.2f}",f"{med(r,'after'):.2f}",f"{med(r,'libtiff'):.2f}",f"{r['speedup']:.2f}×",f"{r['vs_libtiff']:.2f}×"] for r in (find(*p) for p in picks) if r]))
 agg=[]
 for m in DECODE:
  g=[r for r in decode if r['mode']==m]
  if not g:continue
  agg.append([m,str(len(g)),f"{gm(med(r,'before')/med(r,'libtiff') for r in g):.2f}×",f"{gm(r['vs_libtiff'] for r in g):.2f}×",f"{gm(r['speedup'] for r in g):.2f}×"])
 out.append(table(['Operation','Workloads','Before / libtiff','After / libtiff','Speedup'],agg))
 slow=sorted((r for r in rs if r['speedup']<1/1.1),key=lambda r:r['speedup'])
 out.append('Median slowdowns above 10%: '+(', '.join(f"{label(r)} ({med(r,'before'):#.3g} → {med(r,'after'):#.3g} ms)" for r in slow) if slow else 'none')+'.')
 rss=[(label(r),r['statistics']['before']['median_rss_mib'],r['statistics']['after']['median_rss_mib']) for r in rs]
 grew=[x for x in rss if x[2]-x[1]>0.5]
 out.append('Peak RSS growth above 0.5 MiB: '+(', '.join(f"{n} ({a:.1f} → {b:.1f} MiB)" for n,a,b in grew) if grew else 'none')+'.')
 print('\n\n'.join(out))
if __name__=='__main__':main()
