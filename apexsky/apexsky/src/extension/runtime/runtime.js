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
        return core.opAsync("op_fetch", url);
    },
    onMessage: {
        listeners: [],
        addListener: (callback) => {
            apexsky.runtime.onMessage.listeners.push(callback);
        },
    },
};

const apexsky = {
    runtime,
}

// Create an async function to poll messages
async function pollMessages() {
    while (true) {
        // Retrieve messages using core.opAsync("op_poll_message")
        const message = await core.opAsync("op_poll_message");

        // Iterate over all listeners and call them to handle the message
        apexsky.runtime.onMessage.listeners.forEach((listener) => {
            listener(message);
        });
    }
}

// Start polling for messages
pollMessages().catch((err) => {
    console.error("Error polling messages:", err);
});



globalThis.setTimeout = (callback, delay) => {
    core.opAsync("op_set_timeout", delay).then(callback);
};
globalThis.console = console;
globalThis.apexsky = apexsky;
