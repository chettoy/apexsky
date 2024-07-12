const config = {
    key: 68, // KEY_CAPSLOCK
    smooth0: 200.0,
    smooth1: 120.0,
};

let prev_pressed = false;

async function action(now_pressed) {
    if (prev_pressed === now_pressed) {
        return;
    }

    let g_settings = apexsky.config.globalSettings();

    if (!prev_pressed && now_pressed) {
        // console.log("key down");
        g_settings.aimbot_settings.smooth = config.smooth1;
        apexsky.config.updateGlobalSettings(g_settings);
    } else if (prev_pressed && !now_pressed) {
        // console.log("key up");
        g_settings.aimbot_settings.smooth = config.smooth0;
        apexsky.config.updateGlobalSettings(g_settings);
    }

    prev_pressed = now_pressed;
}

apexsky.runtime.onMessage(async ({ name, data }) => {
    if (name === "create") {
        console.log("Hello, world!");
        return {
            "watch_input": [config.key]
        };
    } else if (name === "action_tick") {
        const now_pressed = data.input[config.key];
        await action(now_pressed);
    }
});
