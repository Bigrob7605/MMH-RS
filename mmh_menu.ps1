# MMH-RS V1 Menu System
Clear-Host
Write-Host "MMH-RS V1 Menu System" -ForegroundColor Green
Write-Host "=====================" -ForegroundColor Green

function Show-MainMenu {
    Clear-Host
    Write-Host "============================" -ForegroundColor Green
    Write-Host "|    MMH-RS MAIN MENU      |" -ForegroundColor Green
    Write-Host "|    V1 - Core Release     |" -ForegroundColor Green
    Write-Host "============================" -ForegroundColor Green
    Write-Host "  1. Benchmark (Try MMH File System)"
    Write-Host "  2. Advanced Menu"
    Write-Host "  3. Full CLI Menu"
    Write-Host ""
    Write-Host "--- Available in V2 ---"
    Write-Host "  4. AI Model Benchmarks"
    Write-Host ""
    Write-Host "  Q. Quit"
    Write-Host "----------------------------"
}

function Show-BenchmarksMenu {
    Clear-Host
    Write-Host "--- MMH-RS Benchmark Tiers ---"
    Write-Host "  1. Pack and Unpack a 1GB Seed   (Warm)"
    Write-Host "  2. Pack and Unpack a 2GB Seed   (Toasty)"
    Write-Host "  3. Pack and Unpack a 8GB Seed   (Scorched)"
    Write-Host "  4. Pack and Unpack a 32GB Seed  (Nuked)"
    Write-Host "  5. Pack and Unpack a 128GB Seed (Total Annihilation)"
    Write-Host "  6. Pack and Unpack a 256GB Seed (Memory Madness)"
    Write-Host "  7. Pack and Unpack a 512GB Seed (Swapocalypse)"
    Write-Host "  8. Pack and Unpack a 1TB Seed   (RAMpocalypse)"
    Write-Host "  9. Back to Main Menu"
}

function Show-FileToolsMenu {
    Clear-Host
    Write-Host "--- MMH File System Tools ---"
    Write-Host "  1. Generate test data folder (1 click)"
    Write-Host "  2. Fold a file from test_data folder (picker)"
    Write-Host "  3. Fold entire test_data folder (optimal packing)"
    Write-Host "  4. Unfold a file"
    Write-Host "  B. Back"
}

function Show-AdvancedFeaturesMenu {
    Clear-Host
    Write-Host "--- Advanced Features ---"
    Write-Host "  1. Diagnostics"
    Write-Host "  2. Dev Tools"
    Write-Host "  3. Logs"
    Write-Host "  4. Menu Test"
    Write-Host "  5. Verify Replay Seed"
    Write-Host "  B. Back"
}

function Show-FullCLIMenu {
    Clear-Host
    Write-Host "--- Full MMH-RS CLI ---"
    Write-Host "Enter a CLI command (or 'back', 'exit', 'quit', 'q', ':q' to return):"
    $exitCmds = @('back','exit','quit','q',':q',':exit',':quit')
    while ($true) {
        $cmd = Read-Host "mmh> "
        if ($cmd -eq $null -or $exitCmds -contains $cmd.Trim().ToLower()) { break }
        if ($cmd.Trim() -ne "") {
            & ./target/debug/mmh.exe $cmd
        }
    }
    Write-Host "Exited MMH-RS CLI shell. Returning to main menu..."
    Read-Host "Press Enter to return to the menu"
}

$global:shouldExit = $false
while (-not $global:shouldExit) {
    Show-MainMenu
    $choice = Read-Host "Select option"
    switch ($choice.Trim().ToUpper()) {
        "1" {
            $goBack = $false
            while (-not $goBack -and -not $global:shouldExit) {
                Show-BenchmarksMenu
                $benchChoice = Read-Host "Select option"
                switch ($benchChoice.Trim().ToUpper()) {
                    "1" {
                        Write-Host "Running 1GB (Warm) benchmark..." -ForegroundColor Yellow
                        & ./target/debug/mmh.exe
                        Write-Host "       Press Enter to return to the menu"
                        Read-Host
                    }
                    "2" {
                        Write-Host "Running 2GB (Toasty) benchmark..." -ForegroundColor Yellow
                        & ./target/debug/mmh.exe
                        Write-Host "       Press Enter to return to the menu"
                        Read-Host
                    }
                    "3" {
                        Write-Host "Running 8GB (Scorched) benchmark..." -ForegroundColor Yellow
                        & ./target/debug/mmh.exe
                        Write-Host "       Press Enter to return to the menu"
                        Read-Host
                    }
                    "4" {
                        Write-Host "Running 32GB (Nuked) benchmark..." -ForegroundColor Yellow
                        & ./target/debug/mmh.exe
                        Write-Host "       Press Enter to return to the menu"
                        Read-Host
                    }
                    "5" {
                        Write-Host "Running 128GB (Total Annihilation) benchmark..." -ForegroundColor Yellow
                        & ./target/debug/mmh.exe
                        Write-Host "       Press Enter to return to the menu"
                        Read-Host
                    }
                    "6" {
                        Write-Host "Running 256GB (Memory Madness) benchmark..." -ForegroundColor Yellow
                        & ./target/debug/mmh.exe
                        Write-Host "       Press Enter to return to the menu"
                        Read-Host
                    }
                    "7" {
                        Write-Host "Running 512GB (Swapocalypse) benchmark..." -ForegroundColor Yellow
                        & ./target/debug/mmh.exe
                        Write-Host "       Press Enter to return to the menu"
                        Read-Host
                    }
                    "8" {
                        Write-Host "Running 1TB (RAMpocalypse) benchmark..." -ForegroundColor Yellow
                        & ./target/debug/mmh.exe
                        Write-Host "       Press Enter to return to the menu"
                        Read-Host
                    }
                    "9" { $goBack = $true }
                    default {
                        Write-Host "Invalid option. Please try again." -ForegroundColor Red
                        Read-Host "Press Enter to continue"
                    }
                }
            }
        }
        "2" {
            # Advanced Menu placeholder
            Write-Host "Advanced Menu available in V2!" -ForegroundColor Yellow
            Read-Host "Press Enter to return to the menu"
        }
        "3" {
            # Full CLI Menu placeholder
            Write-Host "Full CLI Menu available in V2!" -ForegroundColor Yellow
            Read-Host "Press Enter to return to the menu"
        }
        "4" {
            Write-Host "AI Model Benchmarks available in V2!" -ForegroundColor Yellow
            Read-Host "Press Enter to return to the menu"
        }
        "Q" { $global:shouldExit = $true }
        default {
            Write-Host "Invalid option. Please try again." -ForegroundColor Red
            Read-Host "Press Enter to continue"
        }
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "|        MMH-RS V1 COMPLETE           |" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Thank you for using MMH-RS V1!" -ForegroundColor Green
Write-Host "The compression engine so honest it will roast your files if they are not worth compressing." -ForegroundColor Yellow
try { Read-Host "Press Enter to close this window" } catch { } 