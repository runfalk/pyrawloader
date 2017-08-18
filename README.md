PyRawLoader
===========
RawLoader (https://github.com/pedrocr/rawloader) integration for Python and
Pillow (https://github.com/python-pillow/Pillow).


Development setup
-----------------
```
virtualenv --prompt="(rawloaderpy)" venv/
source venv/bin/activate
pip install --upgrade pip setuptools
pip install --no-binary :all: snaek
python setup.py build develop
python -c 'import pyrawloader; pyrawloader.decode("test.nef").show()'
```
