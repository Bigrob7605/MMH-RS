#!/bin/sh
# MMH-RS V1 Launcher (POSIX)
# MMH-RS V1 ASCII ART BADGE - Digital DNA
if [ "$1" = "-GUI" ]; then
  echo "[MMH-RS] GUI mode coming in V2!"
  exit 0
fi
if [ $# -eq 0 ]; then
  while true; do
    echo "  ┌─────────────────────┐"
    echo "  │ ┌──┐ ┌──┐ ┌──┐ ┌──┐ │"
    echo "  │ │▒▒│ │▒▒│ │▒▒│ │▒▒│ │"
    echo "  │ └──┘ └──┘ └──┘ └──┘ │"
    echo "  │   M M H - R S       │"
    echo "  └─────────────────────┘"
    echo "     Digital DNA Format"
    echo "============================"
    echo "|   MMH-RS V1 RELEASED!    |"
    echo "|   10GB Proven, Fast!     |"
    echo "============================"
    echo "  MMH-RS V1 Main Menu"
    echo "============================"
    echo "1. Pack a file"
    echo "2. Unpack a file"
    echo "3. Verify round-trip"
    echo "4. Generate random file"
    echo "5. Advanced Features"
    echo "6. View ASCII Art Gallery"
    echo "7. Gandalf Easter Egg"
    echo "Q. Quit"
    printf "Select option: "
    read choice
    case $choice in
      1)
        printf "Input file: "; read in
        printf "Output file: "; read out
        ./mmh.exe pack "$in" "$out"
        ;;
      2)
        printf "Packed file: "; read in
        printf "Output file: "; read out
        ./mmh.exe unpack "$in" "$out"
        ;;
      3)
        printf "Original file: "; read in
        printf "Restored file: "; read out
        ./mmh.exe verify "$in" "$out"
        ;;
      4)
        printf "Output file: "; read out
        printf "Size in bytes: "; read size
        ./mmh.exe gen "$out" --size $size
        ;;
      5)
        show_advanced_menu
        ;;
      6)
        printf "ASCII Art Number (1-8): "; read n
        ./mmh.exe --ascii $n
        ;;
      7)
        ./mmh.exe --wizard
        ;;
      Q|q)
        echo "[MMH-RS] Goodbye!"
        exit 0
        ;;
      *)
        echo "Invalid option."
        ;;
    esac
  done
  exit 0
fi

# Advanced Features Menu Function
show_advanced_menu() {
  while true; do
    echo ""
    echo "  Advanced Features Menu"
    echo "============================"
    echo "1. Run Automated Tests"
    echo "2. Linux/macOS Menu Test"
    echo "3. Clear Test Data"
    echo "4. System Information"
    echo "5. Performance Diagnostics"
    echo "6. Development Tools"
    echo "7. Rebuild"
    echo "8. View Logs"
    echo "9. Export Results"
    echo "B. Back to Main Menu"
    printf "Select option: "
    read adv_choice
    case $adv_choice in
      1)
        echo "Running Automated Test Suite..."
        if [ -f "test_linux_menu.sh" ]; then
          chmod +x test_linux_menu.sh
          ./test_linux_menu.sh
        else
          echo "Test script not found"
        fi
        ;;
      2)
        echo "Running Linux/macOS Menu Test..."
        if [ -f "test_linux_menu.sh" ]; then
          chmod +x test_linux_menu.sh
          ./test_linux_menu.sh
        else
          echo "Test script not found"
        fi
        ;;
      3)
        echo "Clearing test data..."
        rm -rf testdata mmh_stress_test benchmark_testdir demo_testdir mmh_cli.log mmh_error_log.txt main.log test_* 2>/dev/null
        echo "Test data cleared"
        ;;
      4)
        echo "System Information:"
        echo "=================="
        echo "OS: $(uname -s)"
        echo "Architecture: $(uname -m)"
        echo "Shell: $SHELL"
        echo "Current Directory: $(pwd)"
        if [ -f "./mmh" ]; then
          echo "MMH Binary: $(ls -la ./mmh)"
        else
          echo "MMH Binary: Not found"
        fi
        echo ""
        echo "Available Launchers:"
        for launcher in mmh_clickme.sh mmh.sh mmh_universal.sh; do
          if [ -f "$launcher" ]; then
            echo "  ✅ $launcher"
          else
            echo "  ❌ $launcher"
          fi
        done
        ;;
      5)
        echo "Performance Diagnostics:"
        echo "======================="
        if [ -f "./mmh" ]; then
          echo "Testing binary performance..."
          start_time=$(date +%s%N)
          ./mmh --version >/dev/null 2>&1
          end_time=$(date +%s%N)
          duration=$(( (end_time - start_time) / 1000000 ))
          echo "Binary startup time: ${duration}ms"
        fi
        echo "Available Memory: $(free -m | awk 'NR==2{printf "%.0f MB", $7}')"
        echo "Disk Space: $(df -h . | awk 'NR==2{print $4 " free of " $2 " total"}')"
        ;;
      6)
        show_development_tools
        ;;
      7)
        echo "Rebuilding..."
        cargo build --release
        ;;
      8)
        echo "Recent Logs:"
        if [ -f "mmh_cli.log" ]; then
          tail -20 mmh_cli.log
        else
          echo "No log file found"
        fi
        ;;
      9)
        echo "Exporting results..."
        timestamp=$(date +"%Y-%m-%d_%H-%M-%S")
        export_file="mmh_results_${timestamp}.txt"
        if [ -f "mmh_cli.log" ]; then
          cp mmh_cli.log "$export_file"
        fi
        echo "Results exported to $export_file"
        ;;
      B|b)
        echo "Returning to main menu..."
        break
        ;;
      *)
        echo "Invalid option."
        ;;
    esac
  done
}

# Development Tools Function
show_development_tools() {
  while true; do
    echo ""
    echo "  Development Tools"
    echo "=================="
    echo "1. View Cargo.toml"
    echo "2. View Source Files"
    echo "3. Check Rust Version"
    echo "4. List Project Files"
    echo "5. Check Git Status"
    echo "0. Back to Advanced Menu"
    printf "Select option: "
    read dev_choice
    case $dev_choice in
      1)
        if [ -f "Cargo.toml" ]; then
          head -20 Cargo.toml
        else
          echo "Cargo.toml not found"
        fi
        ;;
      2)
        echo "Source Files:"
        if [ -d "src" ]; then
          find src -type f -name "*.rs" | head -10
        else
          echo "src directory not found"
        fi
        ;;
      3)
        if command -v rustc >/dev/null 2>&1; then
          rustc --version
        else
          echo "Rust not found or not in PATH"
        fi
        ;;
      4)
        echo "Project Files:"
        ls -la *.ps1 *.bat *.sh 2>/dev/null || echo "No launcher files found"
        ;;
      5)
        if command -v git >/dev/null 2>&1; then
          git status --porcelain 2>/dev/null || echo "Not a git repository"
        else
          echo "Git not found"
        fi
        ;;
      0)
        break
        ;;
      *)
        echo "Invalid option."
        ;;
    esac
    echo ""
    read -p "Press Enter to continue..."
  done
}

# Forward all arguments to mmh.exe
./mmh.exe "$@" 