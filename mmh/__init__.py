import hashlib
import numpy as np

# --- MMH Core API ---
def unfold(seed: bytes, size: int, mode='fast') -> bytes:
    """
    Deterministic unfold from a 128-bit seed.
    mode='fast': Uses NumPy PCG64 for full 128-bit entropy, blazing speed (default).
    mode='crypto': Uses block-wise SHA-256 expansion for cryptographic security (slower).
    """
    if mode == 'fast':
        bitseed = int.from_bytes(seed, 'big')
        rng = np.random.Generator(np.random.PCG64(bitseed))
        return rng.bytes(size)
    elif mode == 'crypto':
        result = bytearray()
        counter = 0
        while len(result) < size:
            h = hashlib.sha256(seed + counter.to_bytes(8, 'big')).digest()
            result += h
            counter += 1
        return bytes(result[:size])
    else:
        raise ValueError("mode must be 'fast' or 'crypto'")

def unfold_floats(seed: bytes, n: int) -> np.ndarray:
    """
    Deterministic unfold to n float32 values using NumPy PCG64 seeded by 128-bit seed.
    """
    bitseed = int.from_bytes(seed, 'big')
    rng = np.random.Generator(np.random.PCG64(bitseed))
    return rng.random(n, dtype=np.float32)

def fold(data: bytes) -> bytes:
    """Hash data to produce a 128-bit seed (lossy for arbitrary data)."""
    return hashlib.sha256(data).digest()[:16]

def verify(seed: bytes, data: bytes) -> bool:
    """Check if data matches unfold(seed, len(data))."""
    return data == unfold(seed, len(data))

def fingerprint(data: bytes) -> str:
    """Return SHA-256 hex digest of data."""
    return hashlib.sha256(data).hexdigest() 