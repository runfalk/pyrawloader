from setuptools import setup

setup(
    name="PyRawLoader",
    version="0.1.0",
    packages=["pyrawloader"],
    zip_safe=False,
    platforms="any",
    setup_requires=["snaek"],
    install_requires=[
        "pillow",
        "snaek",
    ],
    snaek_rust_modules=[
        ("pyrawloader._rawloader", "rust/"),
    ]
)
