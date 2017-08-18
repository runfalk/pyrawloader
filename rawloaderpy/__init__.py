import ctypes
from ._rawloader import ffi, lib

buff = lib.raw_open("test.nef")
print(ffi.string(buff.maker).decode("utf-8"))
print(ffi.string(buff.model).decode("utf-8"))
lib.raw_free(buff)
print("After free")

# Notes for PIL usage
# from PIL import Image
# a = ffi.buffer(buff.ptr, buff.len)
#im = Image.frombuffer("RGB", (1, 1), a, "raw", "RGB", 0, 1)
#im.show()
