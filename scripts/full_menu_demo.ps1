# MMH-RS V1 Full Menu Demo Script
# This script demonstrates one run of each main menu action using the CLI directly.
# It works in PowerShell and always uses the latest CLI binary.

# MMH-RS V1 ASCII ART BADGE - Digital DNA
Write-Host "============================"
Write-Host "|   MMH-RS V1 RELEASED!    |"
Write-Host "|   10GB Proven, Fast!     |"
Write-Host "============================"

$mmh = if (Test-Path "target/release/mmh.exe") { "target/release/mmh.exe" } else { "mmh.exe" }

Write-Host "[MMH-RS] Full Menu Demo Starting..."

# 1. Generate a small test file
$testfile = "demo_input.txt"
"Hello, MMH-RS!" | Set-Content $testfile

# 2. Pack (compress) the file
$packed = "demo_input.mmhpack"
& $mmh pack $testfile $packed

# 3. Unpack (decompress) the file
$restored = "demo_restored.txt"
& $mmh unpack $packed $restored

# 4. Verify round-trip integrity
& $mmh verify $testfile $restored

# 5. Generate a deterministic random file
$randfile = "demo_random.bin"
& $mmh gen $randfile --size 128

# 6. Generate a test data directory (gentestdir)
$testdir = "demo_testdir"
& $mmh gentestdir $testdir --size 1

# 7. Run a smoketest on the test directory
& $mmh smoketest $testdir

# 8. Fold a file (same as pack)
$folded = "demo_folded.mmhpack"
& $mmh pack $testfile $folded

# 9. Unfold a file (same as unpack)
$unfolded = "demo_unfolded.txt"
& $mmh unpack $folded $unfolded

Write-Host "[MMH-RS] Demo complete. Cleaning up..."

# Clean up demo files (optional)
Remove-Item $testfile,$packed,$restored,$randfile,$folded,$unfolded -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force $testdir -ErrorAction SilentlyContinue

Write-Host "[MMH-RS] All done!" 