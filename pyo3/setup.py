from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="dfs_module",
    version="0.1",
    rust_extensions=[RustExtension("dfs_module", "Cargo.toml", binding="pyo3")],
    packages=["dfs_module"],
    zip_safe=False,
)
