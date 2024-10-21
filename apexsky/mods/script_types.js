const { core } = Deno;
const { ops } = core;

/**
 * @param  {...any} args 
 * @returns {string}
 */
function argsToMessage(...args) {
  if (args.length === 1 && typeof args[0] === "string") {
    return args[0];
  } else {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
  }
}

const console = {
  log: (...args) => {
    // core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    ops.op_log_print(`[out]: ${argsToMessage(...args)}\n`, false);
  },
  error: (...args) => {
    // core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    ops.op_log_print(`[err]: ${argsToMessage(...args)}\n`, true);
  },
  dir: (obj) => {
    ops.op_log_object(obj, false);
  }
};

class TextDecoder {
  /**
   * @param {Uint8Array} input 
   * @returns {string}
   * @throws {Error}
   */
  decode(input) {
    return ops.op_text_decode_utf8(input);
  }
}

class TextEncoder {
  /**
   * @param {string} input 
   * @returns {Uint8Array} 
   */
  encode(input) {
    return ops.op_text_encode_utf8(input);
  }
}

class SkyMemAccess {
  /**
   * @type {number}
   */
  #handle;

  /**
   * @param {Object} target 
   * @throws {Error}
   */
  constructor(target) {
    this.#handle = ops.op_dmalib_mem_open(target);
  }

  /**
   * @param {string} targetProcessName
   * @param {bigint|null} overrideModuleBase
   * @param {number|null} checkTimeDateStamp
   * @param {boolean} speedTest
   * @param {bool} cachePhysAddr
   * @returns {SkyMemAccess}
   * @throws {Error}
   */
  static openFromArgs(
    targetProcessName,
    overrideModuleBase = null,
    checkTimeDateStamp,
    speedTest = true,
    cachePhysAddr = false,
  ) {
    return new SkyMemAccess({
      target_process_name: targetProcessName,
      override_module_base: overrideModuleBase,
      check_time_date_stamp: checkTimeDateStamp,
      speed_test: speedTest,
      cache_phys_addr: cachePhysAddr,
    });
  }

  static get PRIO_HIGH() {
    return 1;
  }

  static get PRIO_LOW() {
    return 0;
  }

  static get PRIO_PASSIVE() {
    return -1;
  }

  static get PRIO_PREEMPT() {
    return 0x10;
  }

  /**
   * @param {number} priority
   * @returns {Promise<bigint>}
   * @throws {Error}
   */
  async getBaseAddr(priority = 1) {
    return await ops.op_dmalib_mem_baseaddr_async(this.#handle, priority);
  }

  /**
   * @param {bigint} addr
   * @param {number} size
   * @param {number} priority
   * @param {number} req_id
   * @returns {Promise<Uint8Array>}
   * @throws {Error}
   */
  async readRaw(addr, size, priority = 0, req_id = 0) {
    return await ops.op_dmalib_mem_read_async(
      this.#handle,
      addr,
      size,
      priority,
      req_id,
    );
  }

  /**
   * @param {bigint} addr
   * @param {Uint8Array} data
   * @param {number} priority
   * @param {number} req_id
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async writeRaw(addr, data, priority = 0, req_id = 0) {
    return await ops.op_dmalib_mem_write_async(
      this.#handle,
      addr,
      data,
      priority,
      req_id,
    );
  }

  /**
   * @param {string} sig
   * @param {bigint} start
   * @param {bigint} end
   * @returns {Promise<bigint|null>}
   * @throws {Error}
   */
  async findPattern(sig, start, end) {
    return await ops.op_dmalib_mem_find_sig_async(this.#handle, sig, start, end);
  }

  /**
   * @param {bigint} addr
   * @param {number} priority
   * @returns {Promise<number>}
   * @throws {Error}
   */
  async readF32(addr, priority = 0) {
    const data = await this.readRaw(addr, 4, priority);
    return new DataView(data.buffer).getFloat32(0, true);
  }

  /**
   * @param {bigint} addr
   * @param {number} priority
   * @returns {Promise<number>}
   * @throws {Error}
   */
  async readI32(addr, priority = 0) {
    const data = await this.readRaw(addr, 4, priority);
    return new DataView(data.buffer).getInt32(0, true);
  }

  /**
   * @param {bigint} addr
   * @param {number} priority
   * @returns {Promise<number>}
   * @throws {Error}
   */
  async readU8(addr, priority = 0) {
    const data = await this.readRaw(addr, 1, priority);
    return new DataView(data.buffer).getUint8(0);
  }

  /**
   * @param {bigint} addr
   * @param {number} priority
   * @returns {Promise<number>}
   * @throws {Error}
   */
  async readU32(addr, priority = 0) {
    const data = await this.readRaw(addr, 4, priority);
    return new DataView(data.buffer).getUint32(0, true);
  }

  /**
   * @param {bigint} addr
   * @param {number} priority
   * @returns {Promise<bigint>}
   * @throws {Error}
   */
  async readU64(addr, priority = 0) {
    const data = await this.readRaw(addr, 8, priority);
    return new DataView(data.buffer).getBigUint64(0, true);
  }

  /**
   * @param {bigint} addr
   * @param {number} val
   * @param {number} priority
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async writeF32(addr, val, priority = 0) {
    const data = new DataView(new ArrayBuffer(4));
    data.setFloat32(0, val, true);
    return await this.writeRaw(addr, new Uint8Array(data.buffer), priority);
  }

  /**
   * @param {bigint} addr
   * @param {number} val
   * @param {number} priority
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async writeI32(addr, val, priority = 0) {
    const data = new DataView(new ArrayBuffer(4));
    data.setInt32(0, val, true);
    return await this.writeRaw(addr, new Uint8Array(data.buffer), priority);
  }

  /**
   * @param {bigint} addr
   * @param {number} val
   * @param {number} priority
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async writeU8(addr, val, priority = 0) {
    const data = new DataView(new ArrayBuffer(1));
    data.setUint8(0, val);
    return await this.writeRaw(addr, new Uint8Array(data.buffer), priority);
  }

  /**
   * @param {bigint} addr
   * @param {number} val
   * @param {number} priority
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async writeU32(addr, val, priority = 0) {
    const data = new DataView(new ArrayBuffer(4));
    data.setUint32(0, val, true);
    return await this.writeRaw(addr, new Uint8Array(data.buffer), priority);
  }

  /**
   * @param {bigint} addr
   * @param {bigint} val
   * @param {number} priority
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async writeU64(addr, val, priority = 0) {
    const data = new DataView(new ArrayBuffer(8));
    data.setBigUint64(0, val, true);
    return await this.writeRaw(addr, new Uint8Array(data.buffer), priority);
  }

  /**
   * @param {bigint} addr
   * @param {number} bufferSize
   * @param {number} priority
   * @returns {Promise<string>}
   * @throws {Error}
   */
  async readCString(addr, bufferSize, priority = 0) {
    const data = await this.readRaw(addr, bufferSize, priority);
    let end = data.indexOf(0);
    if (end === -1) {
      return new TextDecoder().decode(data);
    } else {
      return new TextDecoder().decode(data.subarray(0, end));
    }
  }
}

class SkyHostApi {
  /**
   * @param {number} lhs
   * @param {number} rhs
   * @returns {number}
   */
  static add(lhs, rhs) {
    return ops.op_host_add(lhs, rhs);
  }

  /**
   * @param {string} path
   * @returns {boolean}
   */
  static checkFilePermission(path) {
    return ops.op_host_check_file_permission(path);
  }

  /**
   * @returns {string}
   */
  static getBaseDir() {
    return ops.op_host_get_base_dir();
  }

  /**
   * @returns {string}
   */
  static getConfigDir() {
    return ops.op_host_get_config_dir();
  }

  /**
   * @returns {string|null}
   */
  static getLocale() {
    return ops.op_host_get_locale();
  }

  /**
   * @returns {Array<string>}
   */
  static get skyModuleArgs() {
    return ops.op_host_sky_module_args();
  }

  /**
   * @returns {string}
   */
  static get skyModuleToken() {
    return ops.op_host_sky_module_token();
  }

  static get runtime() {
    return {
      readFile: (path) => {
        return ops.op_read_file(path);
      },
      writeFile: (path, contents) => {
        return ops.op_write_file(path, contents);
      },
      removeFile: (path) => {
        return ops.op_remove_file(path);
      },
      fetch: async (url) => {
        return await ops.op_fetch(url);
      },
    };
  }
}

class SkySharedStore {
  /**
   * @param {bigint} id
   * @param {Uint8Array} data
   * @returns {void}
   */
  static setRaw(id, data) {
    return ops.op_store_set(id, data);
  }

  /**
   * @param {bigint} id
   * @param {Object} data
   * @returns {void}
   */
  static setJson(id, data) {
    return ops.op_store_set(id, new TextEncoder().encode(JSON.stringify(data)));
  }

  /**
   * @param {bigint} id
   * @returns {Uint8Array|null}
   */
  static getRaw(id) {
    return ops.op_store_get(id);
  }

  /**
   * @param {bigint} id
   * @returns {Object|null}
   */
  static getJson(id) {
    const data = ops.op_store_get(id);
    if (data === null) {
      return null;
    }
    return JSON.parse(new TextDecoder().decode(data));
  }

  /**
   * @param {bigint} id
   * @returns {boolean}
   */
  static has(id) {
    return ops.op_store_has(id);
  }

  /**
   * @param {bigint} id
   * @returns {boolean}
   */
  static remove(id) {
    return ops.op_store_del(id);
  }

  /**
   * @param {bigint} id
   * @param {bigint} childId
   * @param {Uint8Array} data
   * @returns {void}
   */
  static childSetRaw(id, childId, data) {
    return ops.op_store_child_set(id, childId, data);
  }

  /**
   * @param {bigint} id
   * @param {bigint} childId
   * @param {Object} data
   * @returns {void}
   */
  static childSetJson(id, childId, data) {
    return ops.op_store_child_set(
      id,
      childId,
      new TextEncoder().encode(JSON.stringify(data)),
    );
  }

  /**
   * @param {bigint} id
   * @param {bigint} childId
   * @returns {Uint8Array|null}
   */
  static childGetRaw(id, childId) {
    return ops.op_store_child_get(id, childId);
  }

  /**
   * @param {bigint} id
   * @param {bigint} childId
   * @returns {Object|null}
   */
  static childGetJson(id, childId) {
    const data = ops.op_store_child_get(id, childId);
    if (data === null) {
      return null;
    }
    return JSON.parse(new TextDecoder().decode(data));
  }

  /**
   * @param {bigint} id
   * @param {bigint} childId
   * @returns {boolean}
   */
  static childHas(id, childId) {
    return ops.op_store_child_has(id, childId);
  }

  /**
   * @param {bigint} id
   * @param {bigint} childId
   * @returns {boolean}
   */
  static childRemove(id, childId) {
    return ops.op_store_child_del(id, childId);
  }

  /**
   * @param {bigint} id
   * @returns {Array<bigint>}
   */
  static listChildren(id) {
    return ops.op_store_list_children(id);
  }

  /**
   * @param {bigint} id
   * @returns {bigint}
   */
  static clearChildren(id) {
    return ops.op_store_clear_children(id);
  }
}

class SkyMsgChannel {
  /**
   * @type {bigint}
   */
  #handle;

  /**
   * @param {string} name
   */
  constructor(name) {
    this.#handle = ops.op_msg_reg_msg(name);
  }

  /**
   * @param {Uint8Array} data
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async sendRaw(data) {
    return await ops.op_msg_send_async(this.#handle, data);
  }

  /**
   * @param {Object} data
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async sendJson(data) {
    return await this.sendRaw(new TextEncoder().encode(JSON.stringify(data)));
  }

  /**
   * @returns {Promise<Uint8Array>}
   * @throws {Error}
   */
  async recvRaw() {
    return await ops.op_msg_recv_async(this.#handle);
  }

  /**
   * @returns {Promise<Object>}
   * @throws {Error}
   */
  async recvJson() {
    return JSON.parse(new TextDecoder().decode(await this.recvRaw()));
  }

  /**
   * @returns {Promise<Uint8Array|null>}
   * @throws {Error}
   */
  tryRecvRaw() {
    return ops.op_msg_try_recv(this.#handle);
  }

  /**
   * @returns {Promise<Object|null>}
   * @throws {Error}
   */
  tryRecvJson() {
    const data = ops.op_msg_try_recv(this.#handle);
    if (data === null) {
      return null;
    }
    return JSON.parse(new TextDecoder().decode(data));
  }
}

class SkyValueWatcher {
  /**
   * @type {bigint}
   */
  #handle;

  /**
   * @param {string} name
   * @param {string} watcherUnique
   */
  constructor(name, watcherUnique) {
    this.#handle = ops.op_msg_watch_subscribe(name, watcherUnique);
  }

  close() {
    ops.op_msg_watch_unsubscribe(this.#handle);
  }

  /**
   * @returns {Uint8Array}
   */
  fetchRaw() {
    return ops.op_msg_watch_fetch_value(this.#handle);
  }

  /**
   * @returns {Object}
   * @throws {Error}
   */
  fetchJson() {
    return JSON.parse(new TextDecoder().decode(this.fetchRaw()));
  }

  /**
   * @returns {Promise<Uint8Array>}
   */
  async nextRaw() {
    return await ops.op_msg_watch_next_value_async(this.#handle);
  }

  /**
   * @returns {Promise<Object>}
   * @throws {Error}
   */
  async nextJson() {
    return JSON.parse(new TextDecoder().decode(await this.nextRaw()));
  }
}

class SkyRpcServer {
  /**
   * @type {bigint}
   */
  #handle;

  /**
   * @param {string} name
   */
  constructor(name) {
    this.#handle = ops.op_rpc_reg_rpc(name);
  }

  /**
   * @returns {Promise<Uint8Array>}
   * @throws {Error}
   */
  async recvRaw() {
    return await ops.op_rpc_recv_async(this.#handle);
  }

  /**
   * @returns {Promise<Object>}
   * @throws {Error}
   */
  async recvJson() {
    return JSON.parse(new TextDecoder().decode(await this.recvRaw()));
  }

  /**
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async recvVoid() {
    await this.recvRaw();
  }

  /**
   * @returns {Promise<Uint8Array|null>}
   * @throws {Error}
   */
  tryRecvRaw() {
    return ops.op_rpc_try_recv(this.#handle);
  }

  /**
   * @param {Uint8Array} data
   * @returns {void}
   * @throws {Error}
   */
  replyRaw(data) {
    return ops.op_rpc_reply(this.#handle, data);
  }

  /**
   * @param {Object} data
   * @returns {void}
   * @throws {Error}
   */
  replyJson(data) {
    return ops.op_rpc_reply(
      this.#handle,
      new TextEncoder().encode(JSON.stringify(data)),
    );
  }

  /**
   * @returns {void}
   * @throws {Error}
   */
  replyVoid() {
    return ops.op_rpc_reply(this.#handle, Uint8Array.of([]));
  }
}

class SkyRpcClient {
  /**
   * @type {bigint}
   */
  #handle;

  /**
   * @param {string} name
   */
  constructor(name) {
    this.#handle = ops.op_rpc_use_rpc(name);
  }

  /**
   * @returns {boolean}
   * @throws {Error}
   */
  isOnline() {
    return ops.op_rpc_is_online(this.#handle);
  }

  /**
   * @returns {Promise<SkyRpcClient>}
   * @throws {Error}
   */
  async waitOnline() {
    while (!(await ops.op_rpc_wait_online_async(this.#handle)));
    return this;
  }

  /**
   * @param {Uint8Array} data
   * @returns {Promise<Uint8Array>}
   * @throws {Error}
   */
  async callRaw(data) {
    return await ops.op_rpc_call_async(this.#handle, data);
  }

  /**
   * @param {Object} data
   * @returns {Promise<Object>}
   * @throws {Error}
   */
  async callJson(data) {
    const ret = await this.callRaw(
      new TextEncoder().encode(JSON.stringify(data)),
    );
    if (ret.length === 0) {
      return null;
    } else {
      return JSON.parse(new TextDecoder().decode(ret));
    }
  }

  /**
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async callVoid() {
    await this.callRaw(Uint8Array.of([]));
  }

  /**
   * @param {Uint8Array} data
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async callPutRaw(data) {
    await this.callRaw(data);
  }

  /**
   * @param {Object} data
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async callPutJson(data) {
    await this.callRaw(new TextEncoder().encode(JSON.stringify(data)));
  }

  /**
   * @returns {Promise<Uint8Array>}
   * @throws {Error}
   */
  async callGetRaw() {
    return await this.callRaw(Uint8Array.of([]));
  }

  /**
   * @returns {Promise<Object>}
   * @throws {Error}
   */
  async callGetJson() {
    const ret = await this.callRaw(Uint8Array.of([]));
    if (ret.length === 0) {
      return null;
    } else {
      return JSON.parse(new TextDecoder().decode(ret));
    }
  }
}

const skyapi = {
  dmalib: {
    open: (target) => {
      return new SkyMemAccess(target);
    },
  },
  host: new SkyHostApi(),
  store: new SkySharedStore(),
  msg: {
    open: (name) => {
      return new SkyMsgChannel(name);
    },
  },
  rpc: {
    reg: (name) => {
      return new SkyRpcServer(name);
    },
    use: (name) => {
      return new SkyRpcClient(name);
    },
  },
};

class GameApi {
  /**
   * @throws {Error}
   */
  constructor() {
    const useRpc = skyapi.rpc.use;
    this.rpc = {
      config_get_global_settings: useRpc("op_config_get_global_settings"),
      config_update_global_settings: useRpc("op_config_update_global_settings"),
      game_frame_count: useRpc("op_game_frame_count"),
      game_get_fps: useRpc("op_game_get_fps"),
      game_get_offsets: useRpc("op_game_get_offsets"),
      game_is_ready: useRpc("op_game_is_ready"),
      game_is_world_ready: useRpc("op_game_is_world_ready"),
      game_local_player_ptr: useRpc("op_game_local_player_ptr"),
      game_view_player_ptr: useRpc("op_game_view_player_ptr"),
      game_cached_player: useRpc("op_game_cached_player"),
      game_cached_npc: useRpc("op_game_cached_npc"),
      game_cached_loot: useRpc("op_game_cached_loot"),
      game_cached_aim_entity: useRpc("op_game_cached_aim_entity"),
    };
  }

  /**
   * Check if all GameApi RPCs of the main mod are online
   * @returns {Promise<boolean>}
   * @throws {Error}
   */
  async isMainModOnline() {
    const rpc = this.rpc;
    return await Promise.all([
      rpc.config_get_global_settings.isOnline(),
      rpc.config_update_global_settings.isOnline(),
      rpc.game_frame_count.isOnline(),
      rpc.game_get_fps.isOnline(),
      rpc.game_get_offsets.isOnline(),
      rpc.game_is_ready.isOnline(),
      rpc.game_is_world_ready.isOnline(),
      rpc.game_local_player_ptr.isOnline(),
      rpc.game_view_player_ptr.isOnline(),
      rpc.game_cached_player.isOnline(),
      rpc.game_cached_npc.isOnline(),
      rpc.game_cached_loot.isOnline(),
      rpc.game_cached_aim_entity.isOnline(),
    ]).then((results) => results.every((r) => r));
  }

  /**
   * Wait for all GameApi RPCs of the main mod to come online
   * @returns {Promise<GameApi>}
   * @throws {Error}
   */
  async waitMainMod() {
    const rpc = this.rpc;
    await Promise.all([
      rpc.config_get_global_settings.waitOnline(),
      rpc.config_update_global_settings.waitOnline(),
      rpc.game_frame_count.waitOnline(),
      rpc.game_get_fps.waitOnline(),
      rpc.game_get_offsets.waitOnline(),
      rpc.game_is_ready.waitOnline(),
      rpc.game_is_world_ready.waitOnline(),
      rpc.game_local_player_ptr.waitOnline(),
      rpc.game_view_player_ptr.waitOnline(),
      rpc.game_cached_player.waitOnline(),
      rpc.game_cached_npc.waitOnline(),
      rpc.game_cached_loot.waitOnline(),
      rpc.game_cached_aim_entity.waitOnline(),
    ]);
    return this;
  }

  static async debugRpc() {
    console.log("Debug game api");

    let game = new GameApi();

    while (true) {
      await sleep(1500);
      let status = "\n";
      for (const [key, value] of Object.entries(game.rpc)) {
        status += `[${key}] isOnline: ${value.isOnline()}\n`;
      }
      console.log("game api status" + status);
      if (await game.isMainModOnline()) {
        break;
      }
    }

    console.log("test wait for game api");

    try {
      let status = "\n";
      for (const [key, value] of Object.entries(game.rpc)) {
        await value.waitOnline();
        status += `[${key}] wait online ok\n`;
      }
      console.log("game api status" + status);

      await game.waitMainMod();
    } catch (e) {
      console.dir(e);
    }

    console.log("Debug game api end");
  }

  /**
   * Get the global settings
   * @returns {Promise<any>}
   * @throws {Error}
   */
  async getGlobalSettings() {
    return await this.rpc.config_get_global_settings.callGetJson();
  }

  /**
   * Update the global settings
   * @param {any} new_val
   * @returns {Promise<void>}
   * @throws {Error}
   */
  async updateGlobalSettings(new_val) {
    await this.rpc.config_update_global_settings.callPutJson(new_val);
  }

  /**
   * Get the frame count
   * @returns {Promise<number>}
   * @throws {Error}
   */
  async getFrameCount() {
    return await this.rpc.game_frame_count.callGetJson();
  }

  /**
   * Get the game FPS
   * @returns {Promise<number>}
   * @throws {Error}
   */
  async getGameFps() {
    return await this.rpc.game_get_fps.callGetJson();
  }

  /**
   * Get the game offsets
   * @returns {Promise<any>}
   * @throws {Error}
   */
  async getGameOffsets() {
    let offsets = await this.rpc.game_get_offsets.callGetJson();
    for (const i in offsets) {
      offsets[i] = BigInt(offsets[i]);
    }
    return offsets;
  }

  /**
   * Check if the game is attached
   * @returns {Promise<boolean>}
   * @throws {Error}
   */
  async isGameAttached() {
    return await this.rpc.game_is_ready.callGetJson();
  }

  /**
   * Check if the world is ready
   * @returns {Promise<boolean>}
   * @throws {Error}
   */
  async isWorldReady() {
    return await this.rpc.game_is_world_ready.callGetJson();
  }

  /**
   * Get the local player pointer
   * @returns {Promise<bigint>}
   * @throws {Error}
   */
  async getLocalPlayerPtr() {
    return BigInt(await this.rpc.game_local_player_ptr.callGetJson());
  }

  /**
   * Get the view player pointer
   * @returns {Promise<bigint>}
   * @throws {Error}
   */
  async getViewPlayerPtr() {
    return BigInt(await this.rpc.game_view_player_ptr.callGetJson());
  }

  /**
   * Get the cached player
   * @param {bigint} ptr
   * @returns {Promise<any>}
   * @throws {Error}
   */
  async getPlayerData(ptr) {
    return await this.rpc.game_cached_player.callJson(ptr.toString());
  }

  /**
   * Get the cached NPC
   * @param {bigint} ptr
   * @returns {Promise<any>}
   * @throws {Error}
   */
  async getNpcData(ptr) {
    return await this.rpc.game_cached_npc.callJson(ptr.toString());
  }

  /**
   * Get the cached loot
   * @param {bigint} ptr
   * @returns {Promise<any>}
   * @throws {Error}
   */
  async getLootData(ptr) {
    return await this.rpc.game_cached_loot.callJson(ptr.toString());
  }

  /**
   * Get the cached aim entity
   * @param {bigint} ptr
   * @returns {Promise<any>}
   * @throws {Error}
   */
  async getAimEntityData(ptr) {
    return await this.rpc.game_cached_aim_entity.callJson(ptr.toString());
  }
}

globalThis.sleep = async (delay) => {
  await ops.op_set_timeout(delay);
};
globalThis.setTimeout = (callback, delay) => {
  ops.op_set_timeout(delay).then(callback);
};
globalThis.console = console;
globalThis.SkyHostApi = SkyHostApi;
globalThis.SkyMemAccess = SkyMemAccess;
globalThis.SkyMsgChannel = SkyMsgChannel;
globalThis.SkyValueWatcher = SkyValueWatcher;
globalThis.SkyRpcClient = SkyRpcClient;
globalThis.SkyRpcServer = SkyRpcServer;
globalThis.SkySharedStore = SkySharedStore;
globalThis.skyapi = skyapi;
globalThis.GameApi = GameApi;
