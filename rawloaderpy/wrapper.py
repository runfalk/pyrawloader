from PIL import Image

from ._rawloader import ffi, lib


def _coalesce(*args):
    return next(arg for arg in args if arg is not None)


def decode(path, max_width=None, max_height=None):
    """
    Decode the given raw image file into a Pillow/PIL Image object.

    This function always preserves aspect ratio, and the image is at most
    ``max_width`` wide and ``max_height`` high

    :param path: Path to a raw image
    :param max_width: Maximum width of output image in pixels
    :param max_height: Maximum height of output image in pixels
    :return: A PIL ``Image`` instance
    """

    # TODO: Lots of error handling
    buff = lib.raw_open(
        path,
        _coalesce(max_height, 0),
        _coalesce(max_height, 0))
    image_buffer = ffi.buffer(buff.data, buff.data_len)
    im = Image.frombuffer(
        "RGB", (buff.width, buff.height), image_buffer, "raw", "RGB", 0, 1)
    lib.raw_free(buff)
    return im
