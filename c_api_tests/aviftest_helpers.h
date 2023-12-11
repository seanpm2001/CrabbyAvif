#include <fstream>
#include <iostream>
#include <iterator>
#include <memory>
#include <vector>

#include "avif/avif.h"
#include "gtest/gtest.h"

namespace avif {

// Struct to call the destroy functions in a unique_ptr.
struct UniquePtrDeleter {
  // void operator()(avifEncoder * encoder) const { avifEncoderDestroy(encoder);
  // }
  void operator()(avifDecoder* decoder) const { avifDecoderDestroy(decoder); }
  // void operator()(avifImage * image) const { avifImageDestroy(image); }
};

// Use these unique_ptr to ensure the structs are automatically destroyed.
// using EncoderPtr = std::unique_ptr<avifEncoder, UniquePtrDeleter>;
using DecoderPtr = std::unique_ptr<avifDecoder, UniquePtrDeleter>;
// using ImagePtr = std::unique_ptr<avifImage, UniquePtrDeleter>;

}  // namespace avif

namespace testutil {
bool Av1DecoderAvailable() { return true; }

std::vector<uint8_t> read_file(const char* file_name) {
  std::ifstream file(file_name, std::ios::binary);
  EXPECT_TRUE(file.is_open());
  // Get file size.
  file.seekg(0, std::ios::end);
  auto size = file.tellg();
  file.seekg(0, std::ios::beg);
  std::vector<uint8_t> data(size);
  file.read(reinterpret_cast<char*>(data.data()), size);
  file.close();
  return data;
}
}  // namespace testutil
