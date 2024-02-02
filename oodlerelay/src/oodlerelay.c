#include <windows.h>
#include <stdint.h>
#include <stdio.h>

typedef int64_t (__cdecl *compress_decl)(int32_t, uint8_t*, uint64_t, uint8_t*, uint64_t, uint32_t*, uint8_t*, uint64_t*, uint64_t, uint64_t);
compress_decl compress_proc = 0;

typedef int64_t (__cdecl *decompress_decl)(uint8_t*, uint64_t, uint8_t*, uint64_t, uint32_t, uint32_t, uint32_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint32_t);
decompress_decl decompress_proc = 0;

typedef int64_t (__cdecl *get_compressed_buf_size_decl)(uint8_t, uint64_t);
get_compressed_buf_size_decl get_compressed_buf_size_proc = 0;

static HINSTANCE oodle_lib = 0;

int64_t oodlerelay_init(const char* path) {
  oodle_lib = LoadLibrary(path);
  if (!oodle_lib) return -1;
  if (!(compress_proc = (compress_decl)GetProcAddress(oodle_lib, "OodleLZ_Compress"))) return -2;
  if (!(decompress_proc = (decompress_decl)GetProcAddress(oodle_lib, "OodleLZ_Decompress"))) return -3;
  if (!(get_compressed_buf_size_proc = (get_compressed_buf_size_decl)GetProcAddress(oodle_lib, "OodleLZ_GetCompressedBufferSizeNeeded"))) return -4;
  return 0;
}

int64_t oodle_compress(int32_t compressor, uint64_t level, uint8_t *src, uint64_t srclen, uint8_t *dst) {
  return compress_proc(compressor, src, srclen, dst, level, 0, 0, 0, 0, 0);
}

int64_t oodle_decompress(uint8_t *src, uint64_t srclen, uint8_t *dst, uint64_t dstlen) {
  return decompress_proc(src, srclen, dst, dstlen, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3);
}

int64_t oodle_get_compressed_buffer_size_needed(uint8_t compressor, uint64_t srclen) {
  return get_compressed_buf_size_proc(compressor, srclen);
}
