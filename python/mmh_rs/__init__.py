"""
MMH-RS: Universal Digital DNA Format
====================================

One format to store them all. Every file. Every platform. Forever.

Example:
    >>> import mmh_rs
    >>> mmh_rs.pack("photos/", "my_photos.dna")
    >>> mmh_rs.unpack("my_photos.dna", "restored_photos/")
"""

from .core import pack, unpack, verify, generate_random_data
from .digital_dna import DigitalDNA, MMHRS, DatasetImmortalizer

__version__ = "1.0.0"
__author__ = "Bigrob7605"
__email__ = "Screwball7605@aol.com"

__all__ = [
    "pack",
    "unpack", 
    "verify",
    "generate_random_data",
    "DigitalDNA",
    "MMHRS",
    "DatasetImmortalizer",
]

# Convenience functions for one-liner usage
def pack(input_path: str, output_path: str, codec: str = "zstd") -> str:
    """
    Immortalize a file with Digital DNA.
    
    Args:
        input_path: Path to the file to immortalize
        output_path: Path for the Digital DNA file
        codec: Compression codec (zstd, lz4, brotli, none)
    
    Returns:
        The Digital DNA seed
    
    Example:
        >>> import mmh_rs
        >>> seed = mmh_rs.pack("photo.jpg", "photo.dna")
        >>> print(f"Digital DNA: {seed}")
    """
    mmh = MMHRS()
    dna = mmh.immortalize(input_path, output_path, codec)
    return dna.seed

def unpack(dna_path: str, output_path: str) -> bool:
    """
    Restore a file from its Digital DNA.
    
    Args:
        dna_path: Path to the Digital DNA file
        output_path: Path for the restored file
    
    Returns:
        True if restoration successful
    
    Example:
        >>> import mmh_rs
        >>> mmh_rs.unpack("photo.dna", "restored_photo.jpg")
        True
    """
    mmh = MMHRS()
    return mmh.restore(dna_path, output_path)

def verify(original_path: str, restored_path: str) -> bool:
    """
    Verify that a restored file matches the original.
    
    Args:
        original_path: Path to the original file
        restored_path: Path to the restored file
    
    Returns:
        True if files match exactly
    
    Example:
        >>> import mmh_rs
        >>> mmh_rs.verify("original.jpg", "restored.jpg")
        True
    """
    mmh = MMHRS()
    return mmh.verify_integrity(original_path, restored_path)

def generate_random_data(output_path: str, size: int, seed: str = None, format: str = "raw") -> str:
    """
    Generate deterministic random data for testing.
    
    Args:
        output_path: Path for the generated data
        size: Size in bytes
        seed: Optional seed for deterministic generation
        format: Output format (raw, hex, base64)
    
    Returns:
        The seed used for generation
    
    Example:
        >>> import mmh_rs
        >>> seed = mmh_rs.generate_random_data("test.bin", 1024)
        >>> print(f"Generated with seed: {seed}")
    """
    mmh = MMHRS()
    return mmh.generate_random_data(output_path, size, seed, format) 