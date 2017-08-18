from PIL import Image

from ._rawloader import ffi, lib


buff = lib.raw_open("test.nef")
print(ffi.string(buff.maker).decode("utf-8"))
print(ffi.string(buff.model).decode("utf-8"))

image_buffer = ffi.buffer(buff.data, buff.data_len)
im = Image.frombuffer("RGB", (buff.width, buff.height), image_buffer, "raw", "RGB", 0, 1)
im.show()

raw_input("Continue")

lib.raw_free(buff)
print("After free")
