import time
import ctypes

dll = ctypes.CDLL('./target/release/libffi.so')
tic = time.time()
dll.process()
print(time.time() - tic)
