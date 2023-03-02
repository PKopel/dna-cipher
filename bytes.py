from struct import pack

fmt = ''.join(['b' for _ in range(256)])
bytes = bytearray([0b01 for _ in range(256)])

with open("file.bin", "wb") as f:
    f.write(pack(fmt, *bytes))
