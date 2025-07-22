# MMH-RS Full Local Test Suite (10GB Simulated Data)
# Usage: Run in PowerShell from project root

$ErrorActionPreference = 'Continue'
$PSScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Definition
$projectRoot = Split-Path $PSScriptRoot -Parent
$log = Join-Path $projectRoot "mmh_test_log.txt"
$testdir = Join-Path $projectRoot "sim_10gb_test"
$packfile = Join-Path $projectRoot "sim_10gb_test.mmhpack"
$restoredir = Join-Path $projectRoot "sim_10gb_restored"
$mmhExe = Join-Path $projectRoot "mmh.exe"

function Log {
    param([string]$msg)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "$timestamp $msg" | Tee-Object -FilePath $log -Append
}

# Cleanup section
$testDirs = @('sim_10gb_test','demo_testdir','test_full_2gb','speedtest_dir','speedtest_dir_10gb','v1_speedtest_10gb','testdir','test_full','test_full_10gb','test_full_2gb','testdir_10gb','smoketest_dir','smoketest_dir_10gb','test_2gb','test_10gb','test_full','test_full_10gb','test_full_2gb','testdir_10gb','testdir')
$testFiles = @('demo_input.txt','demo_input.mmhpack','demo_restored.txt','demo_random.bin','demo_folded.mmhpack','demo_unfolded.txt','mmh_test_log.txt','mmh_error_log.txt')
foreach ($d in $testDirs) { Remove-Item -Recurse -Force $d -ErrorAction SilentlyContinue }
foreach ($f in $testFiles) { Remove-Item -Force $f -ErrorAction SilentlyContinue }
# Wait until all are deleted
$maxTries = 10
$tries = 0
while ($tries -lt $maxTries) {
    $remaining = @()
    foreach ($d in $testDirs) { if (Test-Path $d) { $remaining += $d } }
    foreach ($f in $testFiles) { if (Test-Path $f) { $remaining += $f } }
    if ($remaining.Count -eq 0) { break }
    Write-Host "Waiting for cleanup: $($remaining -join ', ')"
    Start-Sleep -Seconds 1
    $tries++
}
if ($remaining.Count -ne 0) {
    Write-Host "WARNING: Some test files/folders could not be deleted: $($remaining -join ', ')"
}

# 2. Generate 10GB+ of mixed test data
Log "[START] Generating 10GB+ of mixed test data in $testdir..."

# Create root and nested directories
if (!(Test-Path $testdir)) { New-Item -ItemType Directory -Path $testdir | Out-Null }
if (!(Test-Path "$testdir/nested1")) { New-Item -ItemType Directory -Path "$testdir/nested1" | Out-Null }
if (!(Test-Path "$testdir/nested2")) { New-Item -ItemType Directory -Path "$testdir/nested2" | Out-Null }
if (!(Test-Path "$testdir/nested1/deep")) { New-Item -ItemType Directory -Path "$testdir/nested1/deep" | Out-Null }

# Large text file (3GB) - generate by repeating a line
Log "Creating large text file..."
$largeTextFile = "$testdir/large_text.txt"
$largeText = "This is a line of text for the large file."
$lines = 60000000  # ~3GB
$chunkSize = 10000
$chunk = ($largeText + "`n") * $chunkSize
for ($i = 0; $i -lt $lines; $i += $chunkSize) {
    $chunk | Add-Content -Path $largeTextFile
}

# Large text file 2 (2GB)
fsutil file createnew "$testdir/large_text2.txt" 2147483648

# Large binary file (2GB)
Log "Creating large binary file..."
fsutil file createnew "$testdir/large_bin.bin" 2147483648

# Random data (1GB)
Log "Creating random data file..."
if (!(Test-Path $testdir)) { New-Item -ItemType Directory -Path $testdir | Out-Null }
$rand = [System.IO.File]::OpenWrite("$testdir/random1.bin")
$bytes = New-Object byte[] (1073741824)
(new-object Random).NextBytes($bytes)
$rand.Write($bytes,0,$bytes.Length)
$rand.Close()

# Image-like (fake) (500MB)
Log "Creating fake image file..."
fsutil file createnew "$testdir/fake_image.jpg" 524288000

# Archive-like (fake) (500MB)
Log "Creating fake archive file..."
fsutil file createnew "$testdir/fake_archive.zip" 524288000

# Weird format (500MB)
Log "Creating weird format file..."
fsutil file createnew "$testdir/weird_format.weird" 524288000

# Nested files (various sizes)
Log "Creating nested files..."
fsutil file createnew "$testdir/nested1/deep/deepfile1.bin" 524288000
fsutil file createnew "$testdir/nested2/nestfile2.txt" 524288000

# More large files to ensure 10GB+
fsutil file createnew "$testdir/extra1.bin" 1073741824
fsutil file createnew "$testdir/extra2.bin" 1073741824

# Small files
1..100 | ForEach-Object {
    Set-Content "$testdir/smallfile$_.txt" "This is small file $_"
}

# Confirm size
$sizeGB = ((Get-ChildItem -Recurse $testdir | Measure-Object -Property Length -Sum).Sum / 1GB)
Log "Total test data size: $sizeGB GB"
if ($sizeGB -lt 10) {
    Log "[WARNING] Test data is less than 10GB. Consider increasing file sizes."
}

# 3. Run MMH-RS pack (single file)
$singleFile = Join-Path $testdir "large_text.txt"
$packedFile = Join-Path $projectRoot "large_text.mmhpack"
$restoredFile = Join-Path $projectRoot "large_text_restored.txt"
Log "[RUN] mmh.exe pack $singleFile $packedFile"
$packSuccess = $false
& $mmhExe pack $singleFile $packedFile 2>&1 | Tee-Object -FilePath $log -Append
if ($LASTEXITCODE -eq 0) {
    Log "[SUCCESS] mmh.exe pack completed."
    $packSuccess = $true
} else {
    Log "[FAIL] mmh.exe pack failed with exit code $LASTEXITCODE."
}

# 4. Run MMH-RS unpack (single file)
Log "[RUN] mmh.exe unpack $packedFile $restoredFile"
$unpackSuccess = $false
& $mmhExe unpack $packedFile $restoredFile 2>&1 | Tee-Object -FilePath $log -Append
if ($LASTEXITCODE -eq 0) {
    Log "[SUCCESS] mmh.exe unpack completed."
    $unpackSuccess = $true
} else {
    Log "[FAIL] mmh.exe unpack failed with exit code $LASTEXITCODE."
}

# 5. Run MMH-RS verify (single file)
Log "[RUN] mmh.exe verify $singleFile $restoredFile"
$verifySuccess = $false
& $mmhExe verify $singleFile $restoredFile 2>&1 | Tee-Object -FilePath $log -Append
if ($LASTEXITCODE -eq 0) {
    Log "[SUCCESS] mmh.exe verify completed."
    $verifySuccess = $true
} else {
    Log "[FAIL] mmh.exe verify failed with exit code $LASTEXITCODE."
}

# 6. Summary
Log "[SUMMARY]"
if ($packSuccess) { Log "PACK: PASS" } else { Log "PACK: FAIL" }
if ($unpackSuccess) { Log "UNPACK: PASS" } else { Log "UNPACK: FAIL" }
if ($verifySuccess) { Log "VERIFY: PASS" } else { Log "VERIFY: FAIL" }

Log "[END] Test suite complete. Review $log for details." 