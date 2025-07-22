#!/usr/bin/env python3
"""
MMH-RS Agent Runner & Debug Tool
================================

Usage:
  python agent_runner.py [--debug] [--ai] [--logdir DIR] [--clickme]

Features:
- Runs the MMH-RS agent (`mmh --agent`) or ClickMe launcher (`mmh_clickme.bat`)
- Captures stdout/stderr/exit code
- Collects logs (mmh_cli.log, mmh_error_log.txt)
- Debug mode: shows extra info, stack traces, and saves all output
- Saves a combined report to a timestamped file (always, even if output is empty)
- Human and AI/CI friendly

Future: Add --ai for adaptive/fuzzy agent runs, custom scenario support
"""
import subprocess, os, time, hashlib, json, sys, difflib, traceback
import threading
import psutil

vault_lineage = []
AUDIT_RECORDS = []
ERROR_PATTERNS = ['error', 'fail', 'panic', 'exception', 'corrupt', 'mismatch', 'missing', 'not found']
MASTER_LOG = "master_audit_log.txt"
KAI_ERROR_LOG = "kai_error_log.txt"
EVENT_LOG = "event_log.jsonl"

# System profiling globals
PROFILE_DATA = []
PROFILE_RUNNING = False
PROFILE_INTERVAL = 0.5  # seconds

def profile_system(label):
    global PROFILE_RUNNING
    proc = psutil.Process(os.getpid())
    while PROFILE_RUNNING:
        cpu = psutil.cpu_percent(interval=None)
        ram = psutil.virtual_memory().used
        disk = psutil.disk_io_counters()
        PROFILE_DATA.append({
            'label': label,
            'timestamp': time.time(),
            'cpu_percent': cpu,
            'ram_used': ram,
            'disk_read_bytes': disk.read_bytes,
            'disk_write_bytes': disk.write_bytes,
            'disk_read_count': disk.read_count,
            'disk_write_count': disk.write_count
        })
        time.sleep(PROFILE_INTERVAL)

def start_profiling(label):
    global PROFILE_RUNNING
    PROFILE_RUNNING = True
    t = threading.Thread(target=profile_system, args=(label,), daemon=True)
    t.start()

def stop_profiling():
    global PROFILE_RUNNING
    PROFILE_RUNNING = False
    time.sleep(PROFILE_INTERVAL)  # ensure last sample is written

def save_profile_report():
    if PROFILE_DATA:
        with open('benchmark_profile_report.json', 'w', encoding='utf-8') as f:
            json.dump(PROFILE_DATA, f, indent=2)

# Clean up old audit/log/test files for a fresh session
def clean_session(skip=False):
    if skip:
        print("[SKIP] Skipping file cleanup.")
        return
    for f in os.listdir('.'):
        if f.endswith('_audit.json') or f.endswith('.log') or f.endswith('.seed') or f.endswith('.txt') or f.endswith('.bin') or f.endswith('.mmh') or f.endswith('.out') or f.endswith('.aux') or f.endswith('.bbl') or f.endswith('.blg'):
            try: os.remove(f)
            except: pass
    for f in [MASTER_LOG, KAI_ERROR_LOG, EVENT_LOG]:
        if os.path.exists(f):
            try: os.remove(f)
            except: pass

def log_master(msg):
    print(msg)
    with open(MASTER_LOG, "a", encoding="utf-8") as f:
        f.write(msg + "\n")
        f.flush()

def log_event(event):
    with open(EVENT_LOG, "a", encoding="utf-8") as f:
        f.write(json.dumps(event) + "\n")

def sha256(fp):
    h = hashlib.sha256()
    with open(fp, 'rb') as f:
        while chunk := f.read(8192):
            h.update(chunk)
    return h.hexdigest()

def snapshot_tree(root):
    state = {}
    for dirpath, dirs, files in os.walk(root):
        for f in files:
            fp = os.path.join(dirpath, f)
            try:
                state[fp] = {
                    'size': os.path.getsize(fp),
                    'mtime': os.path.getmtime(fp),
                    'sha256': sha256(fp)
                }
            except Exception: pass
    return state

def log_vault_event(cmd, outputs, files_before, files_after):
    event = {
        "cmd": cmd,
        "outputs": [f for f in files_after if f not in files_before],
        "sha256": {f: files_after[f]['sha256'] for f in files_after if f not in files_before},
        "size": {f: files_after[f]['size'] for f in files_after if f not in files_before},
        "timestamp": time.time(),
    }
    vault_lineage.append(event)

def deep_diff(before, after, label):
    diffs = {}
    for f in after:
        if f in before and after[f]['sha256'] != before[f]['sha256']:
            try:
                if f.endswith('.txt'):
                    with open(f, encoding='utf-8', errors='replace') as fa, open(f, encoding='utf-8', errors='replace') as fb:
                        a = fa.readlines()
                        b = fb.readlines()
                        diff = list(difflib.unified_diff(a, b, fromfile='before', tofile='after'))
                        diffs[f] = diff
                else:
                    diffs[f] = f"SHA256 changed: {before[f]['sha256']} -> {after[f]['sha256']}"
            except Exception as e:
                diffs[f] = f"[WARN] Could not diff: {e}"
    if diffs:
        with open(f"deep_diff_{label}.txt", "w", encoding='utf-8') as f:
            for k, v in diffs.items():
                f.write(f"==== {k} ====" + "\n")
                if isinstance(v, list):
                    f.writelines(v)
                else:
                    f.write(str(v) + "\n")
    return diffs

def run_and_audit(cmd, label, input_lines=None, timeout=30, shell=False):
    files_before = snapshot_tree('.')
    log_master(f"\n[RUN] {label}: {cmd}")
    event = {
        "type": "run_step",
        "timestamp": time.time(),
        "agent": "Kai",
        "label": label,
        "cmd": cmd,
        "input": input_lines,
        "state_before": list(files_before.keys()),
    }
    # Start profiling for this step
    start_profiling(label)
    try:
        if input_lines:
            proc = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=shell)
            outs, errs = proc.communicate(input='\n'.join(input_lines).encode(), timeout=timeout)
        else:
            proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=shell)
            outs, errs = proc.communicate(timeout=timeout)
    finally:
        stop_profiling()
    # Timeout/race catching: wait up to 1s for new files
    for _ in range(5):
        files_after = snapshot_tree('.')
        if any(f for f in files_after if f not in files_before):
            break
        time.sleep(0.2)
    else:
        log_master(f"[WARN: Slow FS] No new files detected after {label} (may be a race condition)")
    with open(f"{label}_stdout.txt", "wb") as f: f.write(outs)
    with open(f"{label}_stderr.txt", "wb") as f: f.write(errs)
    log_master(f"[{label}] STDOUT:\n" + outs.decode(errors='replace'))
    log_master(f"[{label}] STDERR:\n" + errs.decode(errors='replace'))
    # Log diff and vault lineage
    changes = {k: files_after[k] for k in files_after if k not in files_before or files_after[k] != files_before[k]}
    log_vault_event(cmd, list(changes.keys()), files_before, files_after)
    deep_diff(files_before, files_after, label)
    # Log/error pattern mining
    error_found = False
    for log in ['mmh_cli.log', 'mmh_error_log.txt']:
        if os.path.exists(log):
            content = open(log, encoding='utf-8', errors='replace').read()
            if any(e in content.lower() for e in ERROR_PATTERNS):
                log_master(f"[FAIL] {log} contains error after {label}!")
                error_found = True
    # Save audit record
    audit = {
        'cmd': cmd,
        'label': label,
        'changes': changes,
        'stdout': outs.decode(errors='replace'),
        'stderr': errs.decode(errors='replace'),
        'timestamp': time.time(),
        'files_after': list(files_after.keys())
    }
    AUDIT_RECORDS.append(audit)
    with open(f"{label}_audit.json", "w", encoding='utf-8') as f:
        json.dump(audit, f, indent=2)
    log_master(f"[PASS] {label} complete.")
    # Log event
    event.update({
        "output": outs.decode(errors='replace'),
        "error": errs.decode(errors='replace'),
        "state_after": list(files_after.keys()),
        "changes": list(changes.keys()),
        "error_found": error_found
    })
    log_event(event)
    # Placeholder for agent handoff/guardrail logic
    # e.g., if error_found: trigger guardrail, escalate, or handoff

def main():
    skip_clean = "--skip-clean" in sys.argv
    smoke = "--smoke" in sys.argv or "--quick" in sys.argv
    abort_after = 10 if "--agent-abort" in sys.argv else None
    clean_session(skip=skip_clean)
    error_summary = []
    start_time = time.time()
    def abort_timer():
        while True:
            if abort_after and (time.time() - start_time) > abort_after:
                print("[ABORT] Test agent run aborted after timeout.")
                save_profile_report()
                os._exit(99)
            time.sleep(0.5)
    if abort_after:
        threading.Thread(target=abort_timer, daemon=True).start()
    if smoke:
        # Minimal quick test
        run_and_audit(["mmh_clickme.bat", "--test"], "clickme", timeout=30, shell=True)
        with open("input.txt", "w", encoding="utf-8") as f: f.write("Hello, MMH-RS!")
        run_and_audit(["target/release/mmh.exe", "pack", "input.txt", "output.seed"], "cli_pack", timeout=10)
        run_and_audit(["target/release/mmh.exe", "unpack", "output.seed", "restored.txt"], "cli_unpack", timeout=10)
        run_and_audit(["target/release/mmh.exe", "verify", "input.txt", "restored.txt"], "cli_verify", timeout=10)
        log_master("[SMOKE] Quick test complete. See master_audit_log.txt for results.")
        return
    # 1. ClickMe batch path (test mode)
    run_and_audit(["mmh_clickme.bat", "--test"], "clickme", timeout=60, shell=True)
    # 2. Menu scripts (simulate input)
    run_and_audit(["powershell", "-File", "mmh_menu.ps1"], "ps_menu", input_lines=["3", "B", "4", "Q"], timeout=30)
    run_and_audit(["mmh_cmdmenu.bat"], "cmd_menu", input_lines=["3", "B", "4", "Q"], timeout=30, shell=True)
    # 3. Direct CLI (pack/unpack/verify, error triggers)
    with open("input.txt", "w") as f: f.write("Hello, MMH-RS!")
    run_and_audit(["target/release/mmh.exe", "pack", "input.txt", "output.seed"], "cli_pack", timeout=10)
    run_and_audit(["target/release/mmh.exe", "unpack", "output.seed", "restored.txt"], "cli_unpack", timeout=10)
    run_and_audit(["target/release/mmh.exe", "verify", "input.txt", "restored.txt"], "cli_verify", timeout=10)
    # Error triggers
    run_and_audit(["target/release/mmh.exe", "unpack", "missing_file.mmhpack", "should_fail.txt"], "cli_unpack_fail", timeout=10)
    run_and_audit(["target/release/mmh.exe", "verify", "input.txt", "missing_file.txt"], "cli_verify_fail", timeout=10)
    # 4. Recursive re-audit and peer review
    log_master("[PEER REVIEW] Checking all audit records...")
    contradiction = False
    all_audits = []
    for fname in os.listdir('.'):
        if fname.endswith('_audit.json'):
            with open(fname, encoding='utf-8') as f:
                rec = json.load(f)
                all_audits.append(rec)
                # Check for fail/contradiction in output
                if any(e in rec.get('stdout', '').lower() for e in ERROR_PATTERNS) or any(e in rec.get('stderr', '').lower() for e in ERROR_PATTERNS):
                    log_master(f"[FAIL] {fname} contains error pattern in output!")
                    error_summary.append(f"[FAIL] {fname} contains error pattern in output!")
    # Cross-link actions/claims for hidden breaks
    pack_success = any("output.seed" in rec['files_after'] for rec in all_audits if rec['label']=="cli_pack")
    unpack_fail = any("missing file" in rec.get('stdout','').lower() for rec in all_audits if rec['label']=="cli_unpack_fail")
    if pack_success and unpack_fail:
        log_master("[FAIL] Contradiction: Pack succeeded but unpack failed on present file!")
        error_summary.append("[FAIL] Contradiction: Pack succeeded but unpack failed on present file!")
        contradiction = True
    # Check for missing/extra files
    for rec in all_audits:
        for f in rec['changes']:
            if not os.path.exists(f):
                log_master(f"[FAIL] {f} reported as created but missing after {rec['label']}!")
                error_summary.append(f"[FAIL] {f} reported as created but missing after {rec['label']}!")
                contradiction = True
    # Save vault lineage manifest
    with open("vault_lineage_manifest.json", "w", encoding='utf-8') as f:
        json.dump(vault_lineage, f, indent=2)
    # Save audit summary
    with open("audit_summary.txt", "w", encoding='utf-8') as f:
        if contradiction or error_summary:
            f.write("[FAIL] Contradiction or error found. See logs above.\n")
            for err in error_summary:
                f.write(err + "\n")
        else:
            f.write("[PASS] All audit checks succeeded.\n")
        f.write(f"Total steps: {len(all_audits)}\n")
        for rec in all_audits:
            f.write(f"{rec['label']}: {list(rec['changes'].keys())}\n")
    # Save full error log if any errors
    if contradiction or error_summary:
        with open(KAI_ERROR_LOG, "w", encoding='utf-8') as f:
            for err in error_summary:
                f.write(err + "\n")
        log_master(f"[KAI] Error log written to {KAI_ERROR_LOG}")
    log_master("")
    log_master("===============================")
    log_master("[KAI] Audit complete. See master_audit_log.txt for full story.")
    log_master("If you see kai_error_log.txt in this folder, open it for detailed errors!")
    log_master("Press Enter to return to the menu...")
    input()
    # Save system profile report at the end
    save_profile_report()

if __name__ == '__main__':
    try:
        main()
    except Exception as ex:
        tb = traceback.format_exc()
        log_master(f"[CRASH] Unhandled exception:\n{tb}")
        print(f"[CRASH] Unhandled exception:\n{tb}")
        raise 