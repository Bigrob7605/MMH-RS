#!/usr/bin/env python3
"""
MMH-RS Python Integration: Digital DNA for Data Science
=======================================================

This example demonstrates how to use MMH-RS as the Universal Digital DNA Format
for data science, machine learning, and research workflows.

Features:
- Immortalize datasets with Digital DNA
- Reproducible research with deterministic seeds
- Time-travel for your entire data pipeline
- Proof of originality for research data
"""

import subprocess
import hashlib
import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from datetime import datetime

@dataclass
class DigitalDNA:
    """Represents a file's Digital DNA - its unique genetic code."""
    seed: str
    original_path: str
    compressed_path: str
    original_size: int
    compressed_size: int
    created_at: datetime
    metadata: Dict

class MMHRS:
    """Python interface for MMH-RS Digital DNA operations."""
    
    def __init__(self, mmh_path: str = "./target/release/mmh"):
        self.mmh_path = mmh_path
        self._verify_mmh_installed()
    
    def _verify_mmh_installed(self):
        """Verify MMH-RS is installed and working."""
        try:
            result = subprocess.run([self.mmh_path, "--help"], 
                                  capture_output=True, text=True)
            if result.returncode != 0:
                raise RuntimeError("MMH-RS not found or not working")
        except FileNotFoundError:
            raise RuntimeError(f"MMH-RS not found at {self.mmh_path}")
    
    def immortalize(self, input_path: str, output_path: str, 
                   codec: str = "zstd") -> DigitalDNA:
        """
        Immortalize a file with Digital DNA.
        
        Args:
            input_path: Path to the file to immortalize
            output_path: Path for the Digital DNA file
            codec: Compression codec (zstd, lz4, brotli, none)
        
        Returns:
            DigitalDNA object with the file's genetic information
        """
        # Pack the file
        cmd = [self.mmh_path, "pack", input_path, output_path, "--codec", codec]
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise RuntimeError(f"Failed to immortalize {input_path}: {result.stderr}")
        
        # Extract seed from output
        seed = None
        for line in result.stdout.split('\n'):
            if line.startswith("Seed: "):
                seed = line.split("Seed: ")[1]
                break
        
        if not seed:
            raise RuntimeError("Could not extract seed from MMH-RS output")
        
        # Get file sizes
        original_size = os.path.getsize(input_path)
        compressed_size = os.path.getsize(output_path)
        
        # Create metadata
        metadata = {
            "codec": codec,
            "compression_ratio": original_size / compressed_size,
            "sha256": self._calculate_sha256(input_path),
            "file_type": Path(input_path).suffix,
            "immortalized_at": datetime.now().isoformat()
        }
        
        return DigitalDNA(
            seed=seed,
            original_path=input_path,
            compressed_path=output_path,
            original_size=original_size,
            compressed_size=compressed_size,
            created_at=datetime.now(),
            metadata=metadata
        )
    
    def restore(self, dna_path: str, output_path: str) -> bool:
        """
        Restore a file from its Digital DNA.
        
        Args:
            dna_path: Path to the Digital DNA file
            output_path: Path for the restored file
        
        Returns:
            True if restoration successful
        """
        cmd = [self.mmh_path, "unpack", dna_path, output_path]
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise RuntimeError(f"Failed to restore {dna_path}: {result.stderr}")
        
        return True
    
    def verify_integrity(self, original_path: str, restored_path: str) -> bool:
        """
        Verify that a restored file matches the original.
        
        Args:
            original_path: Path to the original file
            restored_path: Path to the restored file
        
        Returns:
            True if files match exactly
        """
        cmd = [self.mmh_path, "verify", original_path, restored_path]
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        return result.returncode == 0
    
    def generate_random_data(self, output_path: str, size: int, 
                           seed: Optional[str] = None, 
                           format: str = "raw") -> str:
        """
        Generate deterministic random data for testing.
        
        Args:
            output_path: Path for the generated data
            size: Size in bytes
            seed: Optional seed for deterministic generation
            format: Output format (raw, hex, base64)
        
        Returns:
            The seed used for generation
        """
        cmd = [self.mmh_path, "gen", output_path, "--size", str(size), "--format", format]
        
        if seed:
            cmd.extend(["--seed", seed])
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        
        if result.returncode != 0:
            raise RuntimeError(f"Failed to generate random data: {result.stderr}")
        
        # Extract seed from output
        generated_seed = None
        for line in result.stdout.split('\n'):
            if line.startswith("Seed: "):
                generated_seed = line.split("Seed: ")[1]
                break
        
        return generated_seed or seed or "unknown"
    
    def _calculate_sha256(self, file_path: str) -> str:
        """Calculate SHA256 hash of a file."""
        sha256_hash = hashlib.sha256()
        with open(file_path, "rb") as f:
            for chunk in iter(lambda: f.read(4096), b""):
                sha256_hash.update(chunk)
        return sha256_hash.hexdigest()

class DatasetImmortalizer:
    """High-level interface for immortalizing datasets and research data."""
    
    def __init__(self, mmh: MMHRS):
        self.mmh = mmh
        self.dna_registry = {}
    
    def immortalize_dataset(self, dataset_path: str, 
                          output_dir: str = "./immortal_datasets") -> DigitalDNA:
        """
        Immortalize an entire dataset with Digital DNA.
        
        Args:
            dataset_path: Path to the dataset file or directory
            output_dir: Directory to store immortalized data
        
        Returns:
            DigitalDNA object for the dataset
        """
        os.makedirs(output_dir, exist_ok=True)
        
        # Generate output path
        dataset_name = Path(dataset_path).stem
        output_path = os.path.join(output_dir, f"{dataset_name}.mmh")
        
        # Immortalize the dataset
        dna = self.mmh.immortalize(dataset_path, output_path)
        
        # Store in registry
        self.dna_registry[dataset_name] = dna
        
        return dna
    
    def create_research_snapshot(self, data_files: List[str], 
                               snapshot_name: str) -> Dict:
        """
        Create a research snapshot with multiple datasets.
        
        Args:
            data_files: List of data files to immortalize
            snapshot_name: Name for the research snapshot
        
        Returns:
            Dictionary with snapshot information
        """
        snapshot_dir = f"./research_snapshots/{snapshot_name}"
        os.makedirs(snapshot_dir, exist_ok=True)
        
        snapshot_data = {
            "snapshot_name": snapshot_name,
            "created_at": datetime.now().isoformat(),
            "datasets": {},
            "total_original_size": 0,
            "total_compressed_size": 0
        }
        
        for data_file in data_files:
            if not os.path.exists(data_file):
                print(f"Warning: {data_file} not found, skipping")
                continue
            
            # Immortalize the dataset
            output_path = os.path.join(snapshot_dir, f"{Path(data_file).stem}.mmh")
            dna = self.mmh.immortalize(data_file, output_path)
            
            # Add to snapshot
            dataset_name = Path(data_file).stem
            snapshot_data["datasets"][dataset_name] = {
                "seed": dna.seed,
                "original_path": data_file,
                "compressed_path": output_path,
                "original_size": dna.original_size,
                "compressed_size": dna.compressed_size,
                "compression_ratio": dna.metadata["compression_ratio"]
            }
            
            snapshot_data["total_original_size"] += dna.original_size
            snapshot_data["total_compressed_size"] += dna.compressed_size
        
        # Save snapshot metadata
        metadata_path = os.path.join(snapshot_dir, "snapshot_metadata.json")
        with open(metadata_path, 'w') as f:
            json.dump(snapshot_data, f, indent=2)
        
        return snapshot_data
    
    def restore_research_snapshot(self, snapshot_name: str, 
                                output_dir: str = "./restored_research") -> List[str]:
        """
        Restore an entire research snapshot.
        
        Args:
            snapshot_name: Name of the snapshot to restore
            output_dir: Directory to restore files to
        
        Returns:
            List of restored file paths
        """
        snapshot_dir = f"./research_snapshots/{snapshot_name}"
        metadata_path = os.path.join(snapshot_dir, "snapshot_metadata.json")
        
        if not os.path.exists(metadata_path):
            raise FileNotFoundError(f"Snapshot metadata not found: {metadata_path}")
        
        with open(metadata_path, 'r') as f:
            snapshot_data = json.load(f)
        
        os.makedirs(output_dir, exist_ok=True)
        restored_files = []
        
        for dataset_name, dataset_info in snapshot_data["datasets"].items():
            compressed_path = dataset_info["compressed_path"]
            restored_path = os.path.join(output_dir, f"{dataset_name}")
            
            # Restore the dataset
            self.mmh.restore(compressed_path, restored_path)
            restored_files.append(restored_path)
            
            print(f"✅ Restored {dataset_name}")
        
        return restored_files

def main():
    """Example usage of MMH-RS Digital DNA for data science."""
    
    print("🧬 MMH-RS Digital DNA for Data Science")
    print("=======================================")
    print()
    
    # Initialize MMH-RS
    try:
        mmh = MMHRS()
        print("✅ MMH-RS initialized successfully")
    except Exception as e:
        print(f"❌ Failed to initialize MMH-RS: {e}")
        print("Make sure MMH-RS is built: cargo build --release")
        return
    
    # Create test data
    test_data = "This is test data for Digital DNA demonstration.\n" * 1000
    test_file = "test_dataset.txt"
    
    with open(test_file, 'w') as f:
        f.write(test_data)
    
    print(f"📝 Created test dataset: {test_file}")
    print()
    
    # Immortalize the dataset
    print("🧬 Immortalizing dataset with Digital DNA...")
    dna = mmh.immortalize(test_file, "test_dataset.mmh")
    
    print(f"✅ Dataset immortalized!")
    print(f"   Digital DNA Seed: {dna.seed}")
    print(f"   Original size: {dna.original_size:,} bytes")
    print(f"   Compressed size: {dna.compressed_size:,} bytes")
    print(f"   Compression ratio: {dna.metadata['compression_ratio']:.2f}x")
    print()
    
    # Restore the dataset
    print("🔄 Restoring dataset from Digital DNA...")
    mmh.restore("test_dataset.mmh", "restored_dataset.txt")
    print("✅ Dataset restored successfully!")
    print()
    
    # Verify integrity
    print("🔍 Verifying integrity...")
    if mmh.verify_integrity(test_file, "restored_dataset.txt"):
        print("✅ Integrity verified! Files match exactly.")
    else:
        print("❌ Integrity check failed!")
    print()
    
    # Demonstrate research snapshot
    print("📊 Creating research snapshot...")
    immortalizer = DatasetImmortalizer(mmh)
    
    # Create multiple test files
    test_files = []
    for i in range(3):
        filename = f"research_data_{i}.txt"
        with open(filename, 'w') as f:
            f.write(f"Research data {i}: " + "Sample data for research.\n" * 500)
        test_files.append(filename)
    
    # Create snapshot
    snapshot_data = immortalizer.create_research_snapshot(test_files, "demo_research")
    
    print(f"✅ Research snapshot created!")
    print(f"   Total original size: {snapshot_data['total_original_size']:,} bytes")
    print(f"   Total compressed size: {snapshot_data['total_compressed_size']:,} bytes")
    print(f"   Datasets immortalized: {len(snapshot_data['datasets'])}")
    print()
    
    # Restore snapshot
    print("🔄 Restoring research snapshot...")
    restored_files = immortalizer.restore_research_snapshot("demo_research")
    print(f"✅ Restored {len(restored_files)} datasets!")
    print()
    
    # Cleanup
    cleanup_files = [
        test_file, "test_dataset.mmh", "restored_dataset.txt",
        *test_files, *[f"{f}.mmh" for f in test_files]
    ]
    
    for file in cleanup_files:
        if os.path.exists(file):
            os.remove(file)
    
    print("🧹 Cleanup completed")
    print()
    print("🎉 Digital DNA demonstration completed successfully!")
    print()
    print("Next steps:")
    print("1. Immortalize your own datasets")
    print("2. Create research snapshots for reproducibility")
    print("3. Share Digital DNA seeds with collaborators")
    print("4. Set up automated backups with MMH-RS")
    print()
    print("🚀 Welcome to the future of data storage!")

if __name__ == "__main__":
    main() 