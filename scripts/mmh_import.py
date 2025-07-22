#!/usr/bin/env python3
"""
MMH-RS Import/Export CLI
========================

Universal Digital DNA Format import/export tools
"""

import argparse
import os
import sys
import subprocess
from pathlib import Path
from typing import List, Dict, Optional

def import_from_dropbox(dropbox_path: str, output_dir: str = "./digital_dna") -> Dict:
    """
    Import files from Dropbox to Digital DNA format.
    
    Args:
        dropbox_path: Path to Dropbox folder
        output_dir: Output directory for Digital DNA files
    
    Returns:
        Dictionary with import statistics
    """
    print(f"🧬 Importing from Dropbox: {dropbox_path}")
    
    if not os.path.exists(dropbox_path):
        raise FileNotFoundError(f"Dropbox path not found: {dropbox_path}")
    
    os.makedirs(output_dir, exist_ok=True)
    
    stats = {
        "total_files": 0,
        "total_size": 0,
        "compressed_size": 0,
        "failed_files": [],
        "successful_files": []
    }
    
    # Find all files in Dropbox
    for root, dirs, files in os.walk(dropbox_path):
        for file in files:
            file_path = os.path.join(root, file)
            rel_path = os.path.relpath(file_path, dropbox_path)
            
            # Create output path
            output_path = os.path.join(output_dir, f"{rel_path}.dna")
            os.makedirs(os.path.dirname(output_path), exist_ok=True)
            
            try:
                # Pack with MMH-RS
                cmd = ["./target/release/mmh", "pack", file_path, output_path, "--codec", "zstd"]
                result = subprocess.run(cmd, capture_output=True, text=True)
                
                if result.returncode == 0:
                    original_size = os.path.getsize(file_path)
                    compressed_size = os.path.getsize(output_path)
                    
                    stats["total_files"] += 1
                    stats["total_size"] += original_size
                    stats["compressed_size"] += compressed_size
                    stats["successful_files"].append(rel_path)
                    
                    print(f"  ✅ {rel_path} ({original_size:,} → {compressed_size:,} bytes)")
                else:
                    stats["failed_files"].append(rel_path)
                    print(f"  ❌ {rel_path} - {result.stderr}")
                    
            except Exception as e:
                stats["failed_files"].append(rel_path)
                print(f"  ❌ {rel_path} - {e}")
    
    return stats

def import_from_gdrive(gdrive_path: str, output_dir: str = "./digital_dna") -> Dict:
    """
    Import files from Google Drive to Digital DNA format.
    
    Args:
        gdrive_path: Path to Google Drive folder
        output_dir: Output directory for Digital DNA files
    
    Returns:
        Dictionary with import statistics
    """
    print(f"🧬 Importing from Google Drive: {gdrive_path}")
    
    if not os.path.exists(gdrive_path):
        raise FileNotFoundError(f"Google Drive path not found: {gdrive_path}")
    
    os.makedirs(output_dir, exist_ok=True)
    
    stats = {
        "total_files": 0,
        "total_size": 0,
        "compressed_size": 0,
        "failed_files": [],
        "successful_files": []
    }
    
    # Find all files in Google Drive
    for root, dirs, files in os.walk(gdrive_path):
        for file in files:
            file_path = os.path.join(root, file)
            rel_path = os.path.relpath(file_path, gdrive_path)
            
            # Create output path
            output_path = os.path.join(output_dir, f"{rel_path}.dna")
            os.makedirs(os.path.dirname(output_path), exist_ok=True)
            
            try:
                # Pack with MMH-RS
                cmd = ["./target/release/mmh", "pack", file_path, output_path, "--codec", "zstd"]
                result = subprocess.run(cmd, capture_output=True, text=True)
                
                if result.returncode == 0:
                    original_size = os.path.getsize(file_path)
                    compressed_size = os.path.getsize(output_path)
                    
                    stats["total_files"] += 1
                    stats["total_size"] += original_size
                    stats["compressed_size"] += compressed_size
                    stats["successful_files"].append(rel_path)
                    
                    print(f"  ✅ {rel_path} ({original_size:,} → {compressed_size:,} bytes)")
                else:
                    stats["failed_files"].append(rel_path)
                    print(f"  ❌ {rel_path} - {result.stderr}")
                    
            except Exception as e:
                stats["failed_files"].append(rel_path)
                print(f"  ❌ {rel_path} - {e}")
    
    return stats

def export_to_zip(dna_dir: str, output_zip: str) -> bool:
    """
    Export Digital DNA files to ZIP format.
    
    Args:
        dna_dir: Directory containing Digital DNA files
        output_zip: Output ZIP file path
    
    Returns:
        True if export successful
    """
    print(f"📦 Exporting Digital DNA to ZIP: {output_zip}")
    
    if not os.path.exists(dna_dir):
        raise FileNotFoundError(f"Digital DNA directory not found: {dna_dir}")
    
    try:
        import zipfile
        
        with zipfile.ZipFile(output_zip, 'w', zipfile.ZIP_DEFLATED) as zipf:
            for root, dirs, files in os.walk(dna_dir):
                for file in files:
                    if file.endswith('.dna'):
                        file_path = os.path.join(root, file)
                        arcname = os.path.relpath(file_path, dna_dir)
                        zipf.write(file_path, arcname)
                        print(f"  📦 Added {arcname}")
        
        print(f"✅ Exported {output_zip}")
        return True
        
    except Exception as e:
        print(f"❌ Export failed: {e}")
        return False

def print_stats(stats: Dict, source: str):
    """Print import statistics."""
    print(f"\n📊 {source} Import Statistics:")
    print(f"  Total files: {stats['total_files']:,}")
    print(f"  Original size: {stats['total_size']:,} bytes ({stats['total_size'] / 1024**3:.2f} GB)")
    print(f"  Compressed size: {stats['compressed_size']:,} bytes ({stats['compressed_size'] / 1024**3:.2f} GB)")
    
    if stats['total_size'] > 0:
        compression_ratio = stats['total_size'] / stats['compressed_size']
        savings_percent = (1 - stats['compressed_size'] / stats['total_size']) * 100
        print(f"  Compression ratio: {compression_ratio:.2f}x")
        print(f"  Storage savings: {savings_percent:.1f}%")
    
    print(f"  Successful: {len(stats['successful_files'])}")
    print(f"  Failed: {len(stats['failed_files'])}")
    
    if stats['failed_files']:
        print(f"  Failed files: {', '.join(stats['failed_files'][:5])}")
        if len(stats['failed_files']) > 5:
            print(f"  ... and {len(stats['failed_files']) - 5} more")

def main():
    """Main CLI function."""
    parser = argparse.ArgumentParser(
        description="MMH-RS Import/Export CLI - Universal Digital DNA Format",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  mmh import --from dropbox /path/to/dropbox
  mmh import --from gdrive /path/to/gdrive
  mmh export --to zip ./digital_dna backup.zip
        """
    )
    
    subparsers = parser.add_subparsers(dest='command', help='Available commands')
    
    # Import command
    import_parser = subparsers.add_parser('import', help='Import from cloud services')
    import_parser.add_argument('--from', dest='source', required=True, 
                              choices=['dropbox', 'gdrive'], 
                              help='Source service')
    import_parser.add_argument('path', help='Path to source directory')
    import_parser.add_argument('--output', '-o', default='./digital_dna',
                              help='Output directory (default: ./digital_dna)')
    
    # Export command
    export_parser = subparsers.add_parser('export', help='Export to other formats')
    export_parser.add_argument('--to', dest='format', required=True,
                              choices=['zip'], 
                              help='Export format')
    export_parser.add_argument('input', help='Input directory')
    export_parser.add_argument('output', help='Output file')
    
    args = parser.parse_args()
    
    if args.command == 'import':
        print("🧬 MMH-RS Import Tool")
        print("=====================")
        print()
        
        if args.source == 'dropbox':
            stats = import_from_dropbox(args.path, args.output)
            print_stats(stats, "Dropbox")
        elif args.source == 'gdrive':
            stats = import_from_gdrive(args.path, args.output)
            print_stats(stats, "Google Drive")
        
        print(f"\n✅ Import completed! Digital DNA files saved to: {args.output}")
        
    elif args.command == 'export':
        print("📦 MMH-RS Export Tool")
        print("====================")
        print()
        
        if args.format == 'zip':
            success = export_to_zip(args.input, args.output)
            if success:
                print(f"\n✅ Export completed! ZIP file saved to: {args.output}")
            else:
                print(f"\n❌ Export failed!")
                sys.exit(1)
    
    else:
        parser.print_help()

if __name__ == "__main__":
    main() 