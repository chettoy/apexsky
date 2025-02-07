#include "skyapi.h"

#include <chrono>
#include <cstdint>
#include <cstring>
#include <exception>
#include <fstream>
#include <iostream>
#include <thread>

Host host;
Memory mem;

template <typename T>
bool Read(uintptr_t address, T *out,
          AccessPriority prio = AccessPriority::PRIO_LOW) {
  try {
    mem.read_raw_into(address,
                      rust::Slice<uint8_t>((unsigned char *)out, sizeof(T)),
                      (int)prio);
    return true;
  } catch (const std::exception &e) {
    return false;
  }
}

template <typename T>
bool Write(uintptr_t address, T &value,
           AccessPriority prio = AccessPriority::PRIO_LOW) {
  try {
    mem.write_raw(
        address,
        rust::Slice<const uint8_t>((const unsigned char *)value, sizeof(T)),
        (int)prio);
    return true;
  } catch (const std::exception &e) {
    return false;
  }
}

int main() {

  // Test host api

  // Print hello world in different languages based on the host locale
  const char *locale = host.get_locale().c_str();
  if (strcmp(locale, "en_US") == 0) {
    std::cout << "Hello World!" << std::endl;
  } else if (strcmp(locale, "fr_FR") == 0) {
    std::cout << "Bonjour le monde!" << std::endl;
  } else if (strcmp(locale, "de_DE") == 0) {
    std::cout << "Hallo Welt!" << std::endl;
  } else if (strcmp(locale, "ja_JP") == 0) {
    std::cout << "こんにちは世界!" << std::endl;
  } else if (strcmp(locale, "ko_KR") == 0) {
    std::cout << "안녕하세요 세계!" << std::endl;
  } else if (strcmp(locale, "zh_CN") == 0) {
    std::cout << "你好，世界！" << std::endl;
  } else {
    std::cout << "Hello World!" << std::endl;
  }

  // Test memory api

  // Initialize memory api handle
  mem.init("explorer.exe", 0, true, true);

  // Wait for the process to be ready
  uintptr_t baseAddr = 0;
  while ((baseAddr = mem.get_base_addr()) == 0) {
    std::cout << "Waiting..." << std::endl;
    std::this_thread::sleep_for(std::chrono::seconds(2));
  }
  std::cout << "BaseAddr: 0x" << std::hex << baseAddr << std::endl;

  // Read a 16-bit integer from the process memory
  uint16_t a = 0;
  Read<uint16_t>(baseAddr, &a);
  std::cout << "Read: 0x" << std::hex << a << std::endl;

  // Read a 16-bit integer from the process memory with high priority
  uint16_t b = 0;
  Read<uint16_t>(baseAddr, &b, AccessPriority::PRIO_HIGH);
  std::cout << "Read: 0x" << std::hex << b << std::endl;

  // Read multiple values from the process memory
  a = 0;
  b = 0;
  rust::Vec<BatchReadItem> batch;
  batch.push_back(BatchReadItem{
      baseAddr, rust::Slice<uint8_t>((unsigned char *)&a, sizeof(a))});
  batch.push_back(BatchReadItem{
      baseAddr, rust::Slice<uint8_t>((unsigned char *)&b, sizeof(b))});

  if (mem.batch_read(batch, (int)AccessPriority::PRIO_LOW) == 2) {
    std::cout << "Read: a=0x" << std::hex << a << ", b=0x" << std::hex << b
              << std::endl;
  }

  // Dump the PE file from the process memory
  const bool DUMPER = false;
  if (DUMPER) {
    try {
      auto dump = mem.dump_memory();

      auto dumpfile =
          std::fstream("./dump.bin", std::ios::out | std::ios::binary);
      dumpfile.write((char *)dump.data(), dump.size());
      dumpfile.close();

      std::cout << "dump size: " << dump.size() << std::endl;
      std::cout << "written to ./dump.bin" << std::endl;
    } catch (const std::exception &e) {
      std::cout << "error dump: " << e.what() << std::endl;
    }
  }

  return 0;
}

void sky_main() { main(); }
