#!/usr/bin/env python3
"""Render archived before/after measurements; never hard-code speedup claims."""
import argparse, csv, html, json, math, pathlib

ROOT=pathlib.Path(__file__).resolve().parents[1]
def gm(values): return math.exp(sum(math.log(x) for x in values)/len(values))
def ms(v): return f'{v:.4f}' if v<1 else f'{v:.3f}' if v<10 else f'{v:.2f}'
def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--evidence',type=pathlib.Path,default=ROOT/'docs/evidence/reader-optimization')
    p.add_argument('--html',type=pathlib.Path,default=ROOT.parent/'output/reader-optimization.html')
    a=p.parse_args();data=json.loads((a.evidence/'results.json').read_text());records=data['records'];meta=data['metadata']
    summary=json.loads((a.evidence/'summary.json').read_text())
    assert len(records)==92 and all(r['correctness']['before']['sha256']==r['correctness']['after']['sha256'] for r in records)
    def find(name,mode,group='main'):
        return next(r for r in records if (r['case']['name'],r['mode'],r['group'])==(name,mode,group))
    selected=[find('rgb-deflate',m) for m in ['native','rgba','crop']]
    selected += [find('lerc-float-'+w,'native') for w in ['plain','deflate','zstd']]
    selected += [find('large-strip-deflate',m,'defaults') for m in ['crop','firstrow']]
    columns=['Input / operation','Before ms','After ms','Speedup','4.7.1 ms','After / 4.7.1']
    def values(r):
        s=r['statistics'];return [r['case']['label']+' / '+r['mode']+(' (default chopping)' if r['group']=='defaults' else ''),ms(s['before']['median_ms']),ms(s['after']['median_ms']),f"{r['speedup']:.2f}×",ms(s['libtiff']['median_ms']),f"{r['vs_libtiff']:.2f}×"]
    def table(headers,rows):
        return '| '+' | '.join(headers)+' |\n| '+' | '.join(['---']*len(headers))+' |\n'+''.join('| '+' | '.join(row)+' |\n' for row in rows)
    aggregate=[]
    for mode in ['native','scan','rgba','crop']:
        rs=[r for r in records if r['group']=='main' and r['mode']==mode]
        aggregate.append([mode,str(len(rs)),f"{gm([r['statistics']['before']['median_ms']/r['statistics']['libtiff']['median_ms'] for r in rs]):.2f}×",f"{gm([r['vs_libtiff'] for r in rs]):.2f}×",f"{gm([r['speedup'] for r in rs]):.2f}×"])
    rgb=find('rgb-deflate','native');lerc=find('lerc-float-plain','native')
    lead=f"RGB Deflate native reads improved {rgb['speedup']:.2f}× and plain float32 LERC improved {lerc['speedup']:.2f}× on this suite. Both remain slower than libtiff 4.7.1: {rgb['vs_libtiff']:.2f}× and {lerc['vs_libtiff']:.2f}× as long, respectively."
    validation=f"The full core proof passed {summary['proof']['verified']:,} obligations with zero errors; all {summary['negative_gates']} negative gates rejected their mutations. {summary['rust_tests']} Rust tests, {summary['core_python_tests']} core Python methods, {summary['c_tests']} C methods, the complete C ASan/UBSan suite, 31 original images and 20,000 deterministic core mutations passed. The separate libtiff oracle passed its own 108 tests. C sanitizers do not instrument Rust; LeakSanitizer is disabled because it cannot run under this runner."
    method=f"Measured {meta['utc']} on {meta['cpu_model']}, CPU {meta['cpu']}, with {meta['rounds']} randomized serial batches of at least {meta['milliseconds']:g} ms per engine/workload. The exact prior sources and current sources were freshly built with the same pinned toolchain. Each operation uses the unchanged C consumer and a fresh TIFF handle. Timing uses common in-memory callbacks, excluding file loading, output allocation, hashing and process startup. Separate file-backed processes measure peak RSS three times; RSS includes caller buffers and the process, not just library allocations. The main suite and layout sweep disable mmap and strip chopping; the default group enables ordinary strip chopping. The reference is freshly built libtiff 4.7.1, zlib rather than libdeflate, with pinned JPEG SIMD disabled, LERC 4.0.0, WebP 1.6.0 and Zstandard 1.5.7. Quotas are raised identically to the original benchmark."
    correctness="All 92 before/after outputs match byte-for-byte, including JPEG. Against libtiff, 90 match exactly; the two textured-JPEG operations retain the previous maximum difference of one 8-bit channel value. Every timed and memory sample matches its own preflight hash. The eight layout and six default-mode checks are excluded from main-suite geometric means."
    changes=[
      "Deflate now binds an input window once per bounded batch, with a checked refinement of the original transitions. Two work units per transition preserve the former adapter-loop-plus-core-step charge.",
      "Complete native Deflate segment requests decode and validate once into private staging. Checksum, strict ending and permitted final-strip padding are checked before application storage changes. Partial reads retain a validation pass and bounded replay; whole-buffer requests avoid an extra result copy.",
      "LERC normalization replaces bit-by-bit top-bit searches with standard leading-zero operations, proved equal to the existing top model. Software IEEE rounding, acceptance, sample semantics and conservative work quotas are unchanged. No native codec runtime dependency was introduced."]
    limitations="Partial Deflate reads still validate the entire compressed segment before exposing any bytes. Their first-row time is therefore not equivalent to libtiff's early-return validation policy. Large LZW reads, RGBA conversion and complete-segment LERC decoding retain earlier costs. Peak RSS is a process measurement and small differences may be noise. Results are specific to these layouts, inputs, codec builds and host; the reader is still experimental. The formal claims cover the core models and local batch refinement; input-window continuity, private-buffer commit and TIFF/C composition remain tested adapter code."
    layout=[r for r in records if r['group']=='layout'];defaults=[r for r in records if r['group']=='defaults']
    memory=[]
    for r in defaults:
        s=r['statistics'];memory.append([r['case']['label']+' / '+r['mode'],*[f"{s[e]['median_rss_mib']:.2f}" for e in ['before','after','libtiff']]])
    regressions=[r for r in records if r['speedup']<1/1.1]
    regtext=('No workload showed a median slowdown greater than 10% in this run.' if not regressions else
        'Median slowdowns greater than 10% in this run: '+', '.join(f"{r['case']['name']} / {r['mode']} ({r['group']}): {1/r['speedup']:.2f}×" for r in regressions)+'. These are measured differences, not statistical significance claims.')
    recheck_rows=[]
    if (a.evidence/'recheck.json').exists():
        recheck=json.loads((a.evidence/'recheck.json').read_text())
        assert len(recheck['records'])==len(regressions)
        remaining=[r for r in recheck['records'] if r['speedup']<1/1.1]
        regtext+=' Every such workload was rechecked in nine randomized 500 ms batches per engine. '
        regtext+=('None retained a slowdown greater than 10% in that longer run.' if not remaining else
            'Slowdowns greater than 10% remaining in the longer run: '+', '.join(f"{r['case']} / {r['mode']}: {1/r['speedup']:.2f}×" for r in remaining)+'.')
        regtext+=' All original results remain in the main table and geometric means; the longer batches are separate evidence.'
        for r in recheck['records']:
            s=r['statistics'];recheck_rows.append([r['case']+' / '+r['mode']+' ('+r['group']+')',f"{r['main_speedup']:.3f}×",ms(s['before']['median_ms']),ms(s['after']['median_ms']),f"{r['speedup']:.3f}×"])
    md='# Deflate and LERC reader optimization\n\n'+lead+'\n\n'+correctness+'\n\n## Selected results\n\nAll times are medians in milliseconds; after / 4.7.1 above one means slower.\n\n'+table(columns,[values(r) for r in selected])
    md+='\n## Main-suite geometric means\n\n'+table(['Operation','Cases','Before / 4.7.1','After / 4.7.1','Before / after'],aggregate)+'\n'+regtext+'\n'
    if recheck_rows:md+='\nLonger-batch rechecks (speedup below one means slower):\n\n'+table(['Workload','Main speedup','Recheck before ms','Recheck after ms','Recheck speedup'],recheck_rows)
    md+='\n## Changes and proof boundary\n\n'+'\n\n'.join(changes)+'\n\n'+validation+'\n\n'+limitations+'\n'
    md+='\n## Identical RGB pixels, different strip heights\n\n'+table(columns,[values(r) for r in layout])
    md+='\n## Ordinary strip chopping\n\n'+table(columns,[values(r) for r in defaults])
    md+='\nPeak process RSS, MiB:\n\n'+table(['Input / operation','Before','After','4.7.1'],memory)
    md+='\n## Method and reproduction\n\n'+method+'\n\nSee [benchmarks/README.md](../benchmarks/README.md) for commands. Raw batches, exact-output hashes, proof/gate receipts, source hashes and regression logs are in `docs/evidence/reader-optimization/`. The shipped reverse patch and baseline restorer check every archived core/C source before building. The original comparison remains in [reader-benchmark.md](reader-benchmark.md).\n'
    (ROOT/'docs/reader-optimization.md').write_text(md)
    with (a.evidence/'results.csv').open('w') as f:
        w=csv.writer(f);w.writerow(['group','case','mode','before_ms','after_ms','libtiff_ms','speedup','after_over_libtiff','before_rss_mib','after_rss_mib','libtiff_rss_mib'])
        for r in records:
            s=r['statistics'];w.writerow([r['group'],r['case']['name'],r['mode'],*[s[e]['median_ms'] for e in ['before','after','libtiff']],r['speedup'],r['vs_libtiff'],*[s[e]['median_rss_mib'] for e in ['before','after','libtiff']]])
    def htmltable(headers,rows):
        return '<div class="scroll"><table><thead><tr>'+''.join('<th>'+html.escape(x)+'</th>' for x in headers)+'</tr></thead><tbody>'+''.join('<tr>'+''.join('<td>'+html.escape(x)+'</td>' for x in row)+'</tr>' for row in rows)+'</tbody></table></div>'
    page='''<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Deflate and LERC optimization</title>
<style>body{margin:0;background:#f5f6f8;color:#172330;font:16px/1.6 system-ui,sans-serif}main{max-width:1120px;margin:auto;padding:48px 24px}h1{font-size:40px;line-height:1.1;margin:14px 0 22px}h2{margin:40px 0 12px;font-size:23px}.eyebrow{font-size:13px;letter-spacing:.12em;text-transform:uppercase;color:#426274}.lead{font-size:21px;max-width:950px}.scroll{overflow:auto;background:white;border:1px solid #d8e0e6;border-radius:8px}table{border-collapse:collapse;width:100%;font-size:14px}th,td{padding:11px 14px;text-align:right;white-space:nowrap;border-bottom:1px solid #e7ebee}th{background:#eaf0f4;color:#284254}th:first-child,td:first-child{text-align:left}tr:last-child td{border-bottom:0}p{max-width:1020px}li{margin:12px 0}select,input{padding:9px 12px;font:inherit;border:1px solid #becbd4;border-radius:5px;margin:0 8px 14px 0;background:#fff}.note{color:#4d6170;font-size:14px}.good{border-left:4px solid #25876e;padding:12px 20px;background:#e8f4ef}details summary{cursor:pointer;font-weight:600;margin:24px 0 12px}a{color:#076a90}@media print{main{padding:0}body{background:#fff}.scroll{overflow:visible}select,input{display:none}}</style><main><div class="eyebrow">verified-tiff · measured optimization</div><h1>Deflate &amp; LERC<br>before, after, and libtiff 4.7.1</h1>'''
    page+='<p class="lead">'+html.escape(lead)+'</p><p class="good">'+html.escape(correctness)+'</p><h2>Selected results</h2><p class="note">Median milliseconds. Speedup compares before with after; after / 4.7.1 above one means slower.</p>'+htmltable(columns,[values(r) for r in selected])
    page+='<h2>Across the main suite</h2>'+htmltable(['Operation','Cases','Before / 4.7.1','After / 4.7.1','Speedup'],aggregate)+'<p class="note">'+html.escape(regtext)+'</p>'
    if recheck_rows:page+='<details><summary>Longer-batch slowdown rechecks</summary>'+htmltable(['Workload','Main speedup','Recheck before ms','Recheck after ms','Recheck speedup'],recheck_rows)+'</details>'
    page+='<h2>What changed</h2><ol>'+''.join('<li>'+html.escape(s)+'</li>' for s in changes)+'</ol><p>'+html.escape(validation)+'</p><p>'+html.escape(limitations)+'</p>'
    page+='<h2>All 92 workloads</h2><label for="group">Group </label><select id="group"><option value="">All</option><option>main</option><option>layout</option><option>defaults</option></select><label for="query">Filter </label><input id="query" placeholder="e.g. lerc, crop, deflate"><div id="all">'+htmltable(['Group']+columns,[[r['group']]+values(r) for r in records])+'</div>'
    page+='<details><summary>Peak RSS for default-mode partial reads</summary>'+htmltable(['Input / operation','Before MiB','After MiB','4.7.1 MiB'],memory)+'</details><details open><summary>Method and scope</summary><p>'+html.escape(method)+'</p><p class="note">Reproduction scripts, raw samples, output and source hashes, proof receipts, tests, and the exact baseline reverse patch are included in the updated source archive.</p></details>'
    page+='''</main><script>const group=document.getElementById('group'),query=document.getElementById('query');function filter(){for(const row of document.querySelectorAll('#all tbody tr'))row.hidden=!!((group.value&&row.cells[0].textContent!==group.value)||!row.textContent.toLowerCase().includes(query.value.toLowerCase()));}group.addEventListener('change',filter);query.addEventListener('input',filter);</script></html>'''
    a.html.parent.mkdir(parents=True,exist_ok=True);a.html.write_text(page)
    print(lead);print(regtext);print(a.html)

if __name__=='__main__':main()
