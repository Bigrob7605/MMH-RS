#!/usr/bin/env python3
"""
MMH-RS Python Package Setup
===========================

Universal Digital DNA Format for Python
"""

from setuptools import setup, find_packages
import os

# Read the README file
def read_readme():
    with open("README.md", "r", encoding="utf-8") as fh:
        return fh.read()

# Read requirements
def read_requirements():
    with open("requirements.txt", "r", encoding="utf-8") as fh:
        return [line.strip() for line in fh if line.strip() and not line.startswith("#")]

setup(
    name="mmh-rs",
    version="1.0.0",
    author="Bigrob7605",
    author_email="Screwball7605@aol.com",
    description="Universal Digital DNA Format - One format to store them all",
    long_description=read_readme(),
    long_description_content_type="text/markdown",
    url="https://github.com/Bigrob7605/MMH-RS",
    packages=find_packages(),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "Intended Audience :: Science/Research",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Topic :: System :: Archiving :: Backup",
        "Topic :: System :: Filesystems",
        "Topic :: Scientific/Engineering :: Information Analysis",
        "Topic :: Software Development :: Libraries :: Python Modules",
    ],
    python_requires=">=3.8",
    install_requires=read_requirements(),
    extras_require={
        "dev": [
            "pytest>=6.0",
            "pytest-cov>=2.0",
            "black>=21.0",
            "flake8>=3.8",
            "mypy>=0.800",
        ],
        "docs": [
            "sphinx>=4.0",
            "sphinx-rtd-theme>=1.0",
            "myst-parser>=0.15",
        ],
    },
    entry_points={
        "console_scripts": [
            "mmh=mmh_rs.cli:main",
        ],
    },
    include_package_data=True,
    package_data={
        "mmh_rs": ["*.dll", "*.so", "*.dylib"],
    },
    keywords=[
        "compression",
        "storage",
        "backup",
        "digital-dna",
        "universal-format",
        "self-healing",
        "data-integrity",
        "cryptographic",
        "deterministic",
        "cross-platform",
    ],
    project_urls={
        "Bug Reports": "https://github.com/Bigrob7605/MMH-RS/issues",
        "Source": "https://github.com/Bigrob7605/MMH-RS",
        "Documentation": "https://github.com/Bigrob7605/MMH-RS#readme",
        "Migration Challenge": "https://github.com/Bigrob7605/MMH-RS#migration-challenge",
    },
) 