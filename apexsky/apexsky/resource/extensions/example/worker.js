const config = {
    "key": 68, // KEY_CAPSLOCK
    "smooth0": 200.0,
    "smooth1": 150.0,
    "disable_vischeck": false,
};

apexsky.runtime.onMessage(async ({ name, data }) => {
    if (name === "create") {
        console.log("Hello, world!");
    }
});

async function test() {
    const t = Date.now();
    const [a, b, c, d] = await Promise.all([
        (async () => {
            return 1;
        })(),
        (async () => {
            await sleep(400);
            return 4;
        })(),
        (async () => {
            await sleep(200);
            return 2;
        })(),
        (async () => {
            await sleep(300);
            return 3;
        })(),
    ]);
    const time = Date.now() - t;
    console.log(`t=${time}, a${a}, b${b}, c${c}, d${d}`);
}

test();

// apexsky.action.onGameKeyEvent.addListener(async (event) => {
//     if (event.keycode === config.key) {
//         let g_settings = apexsky.config.global_settings();
//         if (event.action === apexsky.action.KEY_DOWN) {
//             if (config.disable_vischeck) {
//                 g_settings.aimbot_settings.aim = 1;
//             }
//             g_settings.aimbot_settings.smooth = smooth1;
//             apexsky.config.update_settings(g_settings);
//         } else if (event.action === apexsky.action.KEY_UP) {
//             g_settings.aimbot_settings.aim = 2;
//             g_settings.aimbot_settings.smooth = smooth0;
//             apexsky.config.update_settings(g_settings);
//         }
//     }
// });
