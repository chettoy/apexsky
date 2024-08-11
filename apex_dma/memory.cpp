#include "memory.hpp"

#include "lib/xorstr/xorstr.hpp"
#include <cstdint>
#include <ios>
#include <iostream>
#include <string>
#include <unistd.h>

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

Memory::Memory() { mf_log_init(LevelFilter::LevelFilter_Info); }

int Memory::open_os(bool nokvm) {
  // load all available plugins
  if (inventory) {
    mf_inventory_free(inventory);
    inventory = nullptr;
  }
  inventory = mf_inventory_scan();
  if (!inventory) {
    mf_log_error(xorstr_("unable to create inventory"));
    return 1;
  }
  printf("%s%p\n", xorstr_("inventory initialized: "), inventory);

  std::string conn_name;
  const std::string conn_arg;
  std::string os_name;
  const std::string os_arg;

  if (nokvm) {
    conn_name = std::string();
    os_name = std::string(xorstr_("native"));
  } else {
    if (access(xorstr_("/dev/memflow"), F_OK) != -1) {
      conn_name = std::string(xorstr_("kvm"));
    } else {
      conn_name = std::string(xorstr_("qemu"));
    }
    os_name = std::string(xorstr_("win32"));
  }
  // const std::string conn_name(xorstr_("pcileech"));
  // const std::string conn_arg(xorstr_(":device=FPGA"));

  // const std::string os_name(xorstr_("win32"));
  // const std::string os_arg;

  ConnectorInstance connector;
  conn = &connector;

  // initialize the connector plugin
  if (conn) {
    printf("Using %s connector.\n", conn_name.c_str());
    if (mf_inventory_create_connector(inventory, conn_name.c_str(),
                                      conn_arg.c_str(), &connector)) {
      printf("Unable to initialize %s connector.\n", conn_name.c_str());
      return 1;
    }

    printf("%s%p\n", xorstr_("Connector initialized: "),
           connector.container.instance.instance);
  }

  // initialize the OS plugin
  if (mf_inventory_create_os(inventory, os_name.c_str(), os_arg.c_str(), conn,
                             &os)) {
    printf("%s", xorstr_("unable to initialize OS\n"));
    return 1;
  }

  printf("%s%p\n", xorstr_("os plugin initialized: "),
         os.container.instance.instance);
  return 0;
}

void Memory::speed_test() {
  if (!proc.baseaddr)
    return;

  std::chrono::milliseconds start_time =
      duration_cast<std::chrono::milliseconds>(
          std::chrono::system_clock::now().time_since_epoch());
  int32_t counter = 0;

  puts(xorstr_("Received metadata:"));
  auto metadata = proc.hProcess.metadata();
  std::cout << xorstr_("real_size=0x") << std::hex << metadata.real_size
            << std::endl;
  std::cout << xorstr_("max_address=0x") << std::hex << metadata.max_address
            << std::endl;
  std::cout << xorstr_("readonly=") << metadata.readonly << std::endl;
  std::cout << std::endl;

  uintptr_t addr = proc.baseaddr;

  puts(xorstr_("== speed test start =="));

  while (counter <= 500) {
    uint8_t buf[0x1000];
    if (proc.hProcess.read_raw_into(
            addr, CSliceMut<uint8_t>((char *)buf, sizeof(uint8_t) * 0x1000)) !=
        0) {
      puts(xorstr_("speed_test: unable to read process memory"));
      return;
    }

    counter += 1;
    if ((counter % 100) == 0) {
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

  if (!(ret = os.process_by_name(CSliceRef<uint8_t>(target_proc),
                                 &proc.hProcess))) {
    const struct ProcessInfo *info = proc.hProcess.info();

    std::cout << target_proc << xorstr_(" process found: 0x") << std::hex
              << info->address << xorstr_("] ") << info->pid << " "
              << info->name << " " << info->path << std::endl;

    // 修复cr3
    const short MZ_HEADER = 0x5a4d;
    char *base_section = new char[8];
    long *base_section_value = (long *)base_section;
    memset(base_section, 0, 8);
    CSliceMut<uint8_t> slice(base_section, 8);
    os.read_raw_into(proc.hProcess.info()->address + 0x520, slice); // win10
    proc.baseaddr = *base_section_value;
    // 遍历dtb
    for (size_t dtb = 0; dtb < SIZE_MAX; dtb += 0x1000) {
      proc.hProcess.set_dtb(dtb, Address_INVALID);
      short c5;
      Read<short>(*base_section_value, c5);
      if (c5 == MZ_HEADER) {
        break;
      }
    }
    status = process_status::FOUND_READY;
  } else {
    status = process_status::NOT_FOUND;
  }
  return ret;
}

Memory::~Memory() {
  if (inventory) {
    mf_inventory_free(inventory);
    inventory = nullptr;
    mf_log_info(xorstr_("inventory freed"));
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
