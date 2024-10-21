#include "skyapi.h"

#include <chrono>
#include <cstdint>
#include <cstring>
#include <exception>
#include <fstream>
#include <iostream>
#include <thread>

Memory mem;

template <typename T> bool Read(uintptr_t address, T *out) {
  try {
    auto buffer =
        mem.read_raw(address, sizeof(T), (int)AccessPriority::PRIO_LOW);
    memcpy(out, buffer.data(), buffer.size());
    return true;
  } catch (const std::exception &e) {
    return false;
  }
}

int main() {

  mem.init("explorer.exe", 0, true, true);

  uintptr_t baseAddr = 0;

  while ((baseAddr = mem.get_base_addr()) == 0) {
    std::cout << "Waiting..." << std::endl;
    std::this_thread::sleep_for(std::chrono::seconds(2));
  }

  std::cout << "BaseAddr: 0x" << std::hex << baseAddr << std::endl;

  uint16_t a = 0;
  Read<uint16_t>(baseAddr, &a);

  std::cout << "Read: 0x" << std::hex << a << std::endl;

  uint16_t b = 0;

  auto batch = rust::Vec<BatchReadItem>();
  batch.push_back(BatchReadItem{
      baseAddr, rust::Slice<uint8_t>((unsigned char *)&a, sizeof(a))});
  batch.push_back(BatchReadItem{
      baseAddr, rust::Slice<uint8_t>((unsigned char *)&b, sizeof(b))});
  if (mem.batch_read(batch, (int)AccessPriority::PRIO_LOW) == 2) {
    std::cout << "Read: a=0x" << std::hex << a << ", b=0x" << std::hex << b
              << std::endl;
  }

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
