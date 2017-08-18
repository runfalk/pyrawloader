from setuptools import setup

setup(
    name="rawloaderpy",
    version="0.1.0",
    packages=["rawloaderpy"],
    zip_safe=False,
    platforms="any",
    setup_requires=["snaek"],
    install_requires=[
        "pillow",
        "snaek",
    ],
    snaek_rust_modules=[
        ("rawloaderpy._rawloader", "rust/"),
    ]
)
