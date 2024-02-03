#include "memory.hpp"

#include "lib/xorstr/xorstr.hpp"
#include <cstdint>
#include <ios>
#include <iostream>
#include <string>

struct FindProcessContext {
  OsInstance<> *os;
  const char *name;
  ProcessInstance<> *target_process;
  bool found;
};

bool find_process(struct FindProcessContext *find_context, Address addr) {

  if (find_context->found) {
    return false;
  }

  if (find_context->os->process_by_address(addr,
                                           find_context->target_process)) {
    return true;
  }

  const struct ProcessInfo *info = find_context->target_process->info();

  if (!strcmp(info->name, find_context->name)) {
    // abort iteration
    find_context->found = true;
    return false;
  }

  // continue iteration
  return true;
}

// Credits: learn_more, stevemk14ebr
size_t findPattern(const PBYTE rangeStart, size_t len, const char *pattern) {
  size_t l = strlen(pattern);
  PBYTE patt_base = static_cast<PBYTE>(malloc(l >> 1));
  PBYTE msk_base = static_cast<PBYTE>(malloc(l >> 1));
  PBYTE pat = patt_base;
  PBYTE msk = msk_base;
  if (pat && msk) {
    l = 0;
    while (*pattern) {
      if (*pattern == ' ')
        pattern++;
      if (!*pattern)
        break;
      if (*(PBYTE)pattern == (BYTE)'\?') {
        *pat++ = 0;
        *msk++ = '?';
        pattern += ((*(PWORD)pattern == (WORD)'\?\?') ? 2 : 1);
      } else {
        *pat++ = getByte(pattern);
        *msk++ = 'x';
        pattern += 2;
      }
      l++;
    }
    *msk = 0;
    pat = patt_base;
    msk = msk_base;
    for (size_t n = 0; n < (len - l); ++n) {
      if (isMatch(rangeStart + n, patt_base, msk_base)) {
        free(patt_base);
        free(msk_base);
        return n;
      }
    }
    free(patt_base);
    free(msk_base);
  }
  return -1;
}

uint64_t Memory::get_proc_baseaddr() { return proc.baseaddr; }

process_status Memory::get_proc_status() { return status; }

void Memory::check_proc() {
  if (status == process_status::FOUND_READY) {
    short c;
    Read<short>(proc.baseaddr, c);

    if (c != 0x5A4D) {
      status = process_status::FOUND_NO_ACCESS;
      close_proc();
    }
  }
}

Memory::Memory() { log_init(LevelFilter::LevelFilter_Info); }

int Memory::open_os() {
  // load all available plugins
  if (inventory) {
    inventory_free(inventory);
    inventory = nullptr;
  }
  inventory = inventory_scan();
  if (!inventory) {
    log_error(xorstr_("unable to create inventory"));
    return 1;
  }
  printf("%s%p\n", xorstr_("inventory initialized: "), inventory);

  const std::string conn_name(xorstr_("pcileech"));
  const std::string conn_arg(xorstr_(":device=FPGA"));

  const std::string os_name(xorstr_("win32"));
  const std::string os_arg;

  ConnectorInstance connector;
  conn = &connector;

  // initialize the connector plugin
  if (conn) {
    printf("Using %s connector.\n", conn_name.c_str());
    if (inventory_create_connector(inventory, conn_name.c_str(),
                                   conn_arg.c_str(), &connector)) {
      printf("Unable to initialize %s connector.\n", conn_name.c_str());
      return 1;
    }

    printf("%s%p\n", xorstr_("Connector initialized: "),
           connector.container.instance.instance);
  }

  // initialize the OS plugin
  if (inventory_create_os(inventory, os_name.c_str(), os_arg.c_str(), conn,
                          &os)) {
    printf("%s", xorstr_("unable to initialize OS\n"));
    return 1;
  }

  printf("%s%p\n", xorstr_("os plugin initialized: "),
         os.container.instance.instance);
  return 0;
}

void Memory::speed_test() {
  std::chrono::milliseconds start_time =
      duration_cast<std::chrono::milliseconds>(
          std::chrono::system_clock::now().time_since_epoch());
  int32_t counter = 0;

  puts(xorstr_("Received metadata:"));
  auto metadata = conn->metadata();
  std::cout << xorstr_("ideal_batch_size=") << std::hex
            << metadata.ideal_batch_size << std::endl;
  std::cout << xorstr_("real_size=") << std::hex << metadata.real_size
            << std::endl;
  std::cout << xorstr_("max_address=") << std::hex << metadata.max_address
            << std::endl;
  std::cout << xorstr_("readonly=") << metadata.readonly << std::endl;
  std::cout << std::endl;

  uintptr_t addr = 0x1000;
  uint8_t mem[8];
  if (conn->phys_view().read_raw_into(
          addr, CSliceMut<uint8_t>((char *)mem, sizeof(uint8_t) * 0x1000)) !=
      0) {
    puts(xorstr_("conn->phys_view().read_raw_into failed"));
  }
  std::cout << xorstr_("Received memory: ") << std::hex << mem << std::endl;
  std::cout << std::endl;

  if (os.read_raw_into(addr, CSliceMut<uint8_t>(
                                 (char *)mem, sizeof(uint8_t) * 0x1000)) != 0) {
    puts(xorstr_("os.read_raw_into failed"));
  }
  std::cout << xorstr_("Received memory: ") << std::hex << mem << std::endl;
  std::cout << std::endl;

  puts(xorstr_("== speed test start =="));

  while (counter <= 50000000) {
    uint8_t buf[0x1000];
    if (os.read_raw_into(
            addr, CSliceMut<uint8_t>((char *)buf, sizeof(uint8_t) * 0x1000)) !=
        0) {
      puts(xorstr_("speed_test: unable to read physical memory"));
      return;
    }

    counter += 1;
    if ((counter % 10000000) == 0) {
      std::chrono::milliseconds now_ms =
          duration_cast<std::chrono::milliseconds>(
              std::chrono::system_clock::now().time_since_epoch());
      long elapsed = (now_ms - start_time).count();
      if (elapsed > 0) {
        std::cout << ((float)counter) / elapsed * 1000.0
                  << xorstr_(" reads/sec") << std::endl;
        std::cout << elapsed / ((float)counter) << xorstr_(" ms/read")
                  << std::endl;
      }
    }
  }

  puts(xorstr_("== speed test end =="));
}

int Memory::open_proc(const char *name) {
  int ret;
  const char *target_proc = name;
  const char *target_module = name;

  // find a specific process based on its name
  // via process_by_name

  if (!(ret = os.process_by_name(CSliceRef<uint8_t>(target_proc),
                                 &proc.hProcess))) {
    const struct ProcessInfo *info = proc.hProcess.info();

    std::cout << target_proc << xorstr_(" process found: 0x") << std::hex
              << info->address << xorstr_("] ") << info->pid << " "
              << info->name << " " << info->path << std::endl;

    // find the module by its name
    ModuleInfo module_info;
    if (!(ret = proc.hProcess.module_by_name(CSliceRef<uint8_t>(target_module),
                                             &module_info))) {
      std::cout << target_proc << xorstr_(" module found: 0x") << std::hex
                << module_info.address << xorstr_("] 0x") << std::hex
                << module_info.base << " " << module_info.name << " "
                << module_info.path << std::endl;

      proc.baseaddr = module_info.base;
      status = process_status::FOUND_READY;
    } else {
      status = process_status::FOUND_NO_ACCESS;
      close_proc();

      printf("%s%s\n", xorstr_("unable to find module: "), target_module);
      log_debug_errorcode(ret);
    }
  } else {
    status = process_status::NOT_FOUND;
  }

  return ret;
}

Memory::~Memory() {
  if (inventory) {
    inventory_free(inventory);
    inventory = nullptr;
    log_info(xorstr_("inventory freed"));
  }
}

void Memory::close_proc() {
  proc.baseaddr = 0;
  status = process_status::NOT_FOUND;
}

uint64_t Memory::ScanPointer(uint64_t ptr_address, const uint32_t offsets[],
                             int level) {
  if (!ptr_address)
    return 0;

  uint64_t lvl = ptr_address;

  for (int i = 0; i < level; i++) {
    if (!Read<uint64_t>(lvl, lvl) || !lvl)
      return 0;
    lvl += offsets[i];
  }

  return lvl;
}
