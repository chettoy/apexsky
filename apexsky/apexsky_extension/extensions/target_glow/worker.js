// Import necessary modules from apexsky configuration and memory management
const { globalSettings } = apexsky.config;
const { getGameBaseaddr, memWriteI32, memWriteF32, memReadU8, memWriteU8 } = apexsky.mem;
const { getGameOffsets, getCachedAimEntity } = apexsky.game;

// Get game offsets for further use in memory operations
const offsets = getGameOffsets();
// Define specific offsets used in the glow functionality
const offset_glow_visible_type = BigInt(0x26c);
const offset_glow_distance = BigInt(0x264);
const offset_glow_fix = BigInt(0x278);

// Define a constant for the highlight ID used in setting glow effect
const highlightId = 78; // HIGHLIGHT_PLAYER_RED: u8 = 78; see `data.rs`

// Register an option to watch aimbot functionality in apexsky runtime
apexsky.runtime.registerOptions({ "watch_aimbot": true });

// Initialize a boolean flag and map for storing original highlight IDs
let glow = false;
const originalHighlightId = new Map();

// Define the actionHook function to update the glow setting based on global settings
async function actionHook() {
    const gSettings = globalSettings();
    glow = gSettings['player_glow'];
}

// Define the aimbotHook function to handle the glow effect for the aimed entity
async function aimbotHook(aimbot) {
    // Check if glow is enabled and ensure there's an aim entity
    if (!glow) {
        return;
    }

    const aimEntityPtr = BigInt(aimbot['aim_entity']);
    if (!getCachedAimEntity(aimEntityPtr)) {
        return;
    }

    // Retrieve and store the original highlight ID if not already done
    if (!originalHighlightId.has(aimEntityPtr)) {
        const value = await memReadU8(aimEntityPtr + offsets.entity_highlight_generic_context - 1n);
        originalHighlightId.set(aimEntityPtr, value);
    }

    // Apply the glow effect by writing to memory locations related to glow settings
    memWriteU8(aimEntityPtr + offsets.entity_highlight_generic_context - 1n, highlightId);
    memWriteI32(aimEntityPtr + offset_glow_visible_type, 2); // Set visible type to 2
    memWriteF32(aimEntityPtr + offset_glow_distance, 200.0 * 40.0); // Set distance value
    memWriteI32(aimEntityPtr + offset_glow_fix, 0);  // Set ptr+0x278 value to 0

    // Restore original highlight IDs for other entities not being aimed at
    originalHighlightId.forEach((value, key, map) => {
        if (key !== aimEntityPtr) {
            memWriteU8(key + offsets.entity_highlight_generic_context - 1n, value);
            map.delete(key);
        }
    });
}

// Listen for messages in the apexsky runtime and handle them accordingly
apexsky.runtime.onMessage(async ({ name, data }) => {
    switch (name) {
        case "create":
            // Log a message when the extension is created
            console.log("target glow mod loaded!");
            break;

        case "action_tick":
            // Handle action tick event by calling the action hook function
            await actionHook();
            break;

        case "aimbot_tick":
            // Handle aimbot tick event by calling the aimbot hook function with data
            await aimbotHook(data.aimbot);
            break;
    }
});
