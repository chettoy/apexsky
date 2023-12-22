#include "memory.hpp"

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
    log_error("unable to create inventory");
    return 1;
  }
  printf("inventory initialized: %p\n", inventory);

  const char *conn_name = "kvm";
  const char *conn_arg = "";

  const char *conn2_name = "qemu";
  const char *conn2_arg = "";

  const char *os_name = "win32";
  const char *os_arg = "";

  ConnectorInstance connector;
  conn = &connector;

  // initialize the connector plugin
  if (conn) {
    printf("Using %s connector.\n", conn_name);
    if (inventory_create_connector(inventory, conn_name, conn_arg,
                                   &connector)) {
      printf("Unable to initialize %s connector.\n", conn_name);
      printf("Fallback to %s connector.\n", conn2_name);

      if (inventory_create_connector(inventory, conn2_name, conn2_arg,
                                     &connector)) {
        printf("Unable to initialize %s connector.\n", conn2_name);
        return 1;
      }
    }

    printf("Connector initialized: %p\n",
           connector.container.instance.instance);
  }

  // initialize the OS plugin
  if (inventory_create_os(inventory, os_name, os_arg, conn, &os)) {
    printf("unable to initialize OS\n");
    return 1;
  }

  printf("os plugin initialized: %p\n", os.container.instance.instance);
  return 0;
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

    printf("%s process found: 0x%lx] %d %s %s\n", target_proc, info->address,
           info->pid, info->name, info->path);

    // find the module by its name
    ModuleInfo module_info;
    if (!(ret = proc.hProcess.module_by_name(CSliceRef<uint8_t>(target_module),
                                             &module_info))) {
      printf("%s module found: 0x%lx] 0x%lx %s %s\n", target_proc,
             module_info.address, module_info.base, module_info.name,
             module_info.path);

      proc.baseaddr = module_info.base;
      status = process_status::FOUND_READY;
    } else {
      status = process_status::FOUND_NO_ACCESS;
      close_proc();

      printf("unable to find module: %s\n", target_module);
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
    log_info("inventory freed");
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