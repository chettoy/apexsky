const { core } = Deno;
const { ops } = core;

function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
}

const console = {
    log: (...args) => {
        core.print(`[out]: ${argsToMessage(...args)}\n`, false);
    },
    error: (...args) => {
        core.print(`[err]: ${argsToMessage(...args)}\n`, true);
    },
};

const runtime = {
    core: {
        msgHandler: null,
        msgLoop: null,
        pollMessages: async () => {
            while (true) {
                // Retrieve messages using core.opAsync("op_poll_message")
                const message = await ops.op_poll_message();

                // Call handler to handle the message
                try {
                    const result = await apexsky.runtime.core.msgHandler(message);
                    ops.op_message_callback(result);
                } catch (e) {
                    console.error("Error handle messages:", e);
                }

            }
        }
    },
    onMessage: (handler) => {
        apexsky.runtime.core.msgHandler = handler;
        if (!apexsky.runtime.core.msgLoop) {
            // Start polling for messages
            apexsky.runtime.core.msgLoop = apexsky.runtime.core.pollMessages().catch((err) => {
                console.error("Error polling messages:", err);
            });
        }
    },

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

const apexsky = {
    runtime,
    config: {
        globalSettings: () => {
            return ops.op_config_get_global_settings();
        },
        updateGlobalSettings: (new_val) => {
            return ops.op_config_update_global_settings(new_val);
        },
    },
    game: {
        getFrameCount: () => {
            return ops.op_game_frame_count();
        },
        getGameFps: () => {
            return ops.op_game_get_fps();
        },
        getGameOffsets: () => {
            let offsets_obj = ops.op_game_get_offsets();
            for (const i in offsets_obj) {
                offsets_obj[i] = BigInt(offsets_obj[i]);
            }
            return offsets_obj;
        },
        isGameAttached: () => {
            return ops.op_game_is_ready();
        },
        isWorldReady: () => {
            return ops.op_game_is_world_ready();
        },
        getLocalPlayerPtr: () => {
            return ops.op_game_local_player_ptr();
        },
        getViewPlayerPtr: () => {
            return ops.op_game_view_player_ptr();
        },
        getCachedPlayer: (ptr) => {
            return ops.op_game_cached_player(ptr);
        }
    },
    mem: {
        getGameBaseaddr: () => {
            return ops.op_mem_game_baseaddr();
        },
        memReadAll: async (list) => {
            return await ops.op_mem_read_all(list);
        },
        memReadF32: async (addr) => {
            return await ops.op_mem_read_f32(addr);
        },
        memReadI32: async (addr) => {
            return await ops.op_mem_read_i32(addr);
        },
        memWriteF32: async (addr, val) => {
            return await ops.op_mem_write_f32(addr, val);
        },
        memWriteI32: async (addr, val) => {
            return await ops.op_mem_write_i32(addr, val);
        },
    },
};

globalThis.sleep = async (delay) => {
    await ops.op_set_timeout(delay);
}
globalThis.setTimeout = (callback, delay) => {
    ops.op_set_timeout(delay).then(callback);
};
globalThis.console = console;
globalThis.apexsky = apexsky;
