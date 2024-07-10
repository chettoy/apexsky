const { globalSettings } = apexsky.config;
const { getGameBaseaddr, memReadAll, memReadI32, memReadF32, memWriteI32 } = apexsky.mem;
const { getFrameCount, getGameFps, getGameOffsets, getLocalPlayerPtr, isWorldReady } = apexsky.game;

const offsets = getGameOffsets();

let sg_ctx = {
    start_jump_time: 0,
    start_sg: false,
    last_sg_finish_ms: 0,
};

async function action() {
    if (!isWorldReady()) {
        return;
    }

    let g_settings = globalSettings();

    const baseaddr = getGameBaseaddr();
    const lplayer = getLocalPlayerPtr();

    const [jump_state, world_time, traversal_start_time, traversal_progress, is_grpple_actived] = await memReadAll([
        { "type": "i32", "addr": (baseaddr + offsets.in_jump) },
        { "type": "f32", "addr": (lplayer + offsets.cplayer_timebase) },
        { "type": "f32", "addr": (lplayer + offsets.cplayer_traversal_starttime) },
        { "type": "f32", "addr": (lplayer + offsets.cplayer_traversal_progress) },
        { "type": "i32", "addr": (lplayer + offsets.player_grapple_active) },
    ]);

    if (g_settings.super_key_toggle) {
        /** SuperGlide
         * https://www.unknowncheats.me/forum/apex-legends/578160-external-auto-superglide-3.html
         */
        const hang_on_wall = world_time - traversal_start_time;
        const game_fps = getGameFps();

        let hang_start, hang_cancel, trav_start, hang_max, action_interval;
        let release_wait;

        // for 75 fps
        hang_start = 0.1;
        hang_cancel = 0.12;
        trav_start = 0.87;
        hang_max = 1.5;
        action_interval = 0.011;
        release_wait = 50;
        if (Math.abs(game_fps - 144.0) <
            Math.abs(game_fps - 75.0)) {
            // for 144 fps
            hang_start = 0.05;
            hang_cancel = 0.07;
            trav_start = 0.90;
            hang_max = 0.75;
            action_interval = 0.007;
            release_wait = 25;
            if (Math.abs(game_fps - 240.0) <
                Math.abs(game_fps - 144.0)) {
                // for 240 fps
                hang_start = 0.033;
                hang_cancel = 0.04;
                trav_start = 0.95;
                hang_max = 0.2;
                action_interval = 0.004;
                release_wait = 20;
            }
        }

        if (hang_on_wall > hang_start) {
            if (hang_on_wall < hang_cancel) {
                await memWriteI32(baseaddr + offsets.in_jump + BigInt(0x8), 4);
            }
            if (traversal_progress > trav_start && hang_on_wall < hang_max && !sg_ctx.start_sg) {
                const now_ms = Date.now();
                if (now_ms - sg_ctx.last_sg_finish_ms > 320 && jump_state > 0) {
                    // start SG
                    sg_ctx.start_jump_time = world_time;
                    sg_ctx.start_sg = true;
                }
            }
        }
        if (sg_ctx.start_sg) {
            // press jump button
            await memWriteI32(baseaddr + offsets.in_jump + BigInt(0x8), 5);

            while (true) {
                const current_time = await memReadF32(lplayer + offsets.cplayer_timebase);
                if (current_time - sg_ctx.start_jump_time < action_interval) {
                    //keep looping    
                } else {
                    break;
                }
            }
            await memWriteI32(baseaddr + offsets.in_duck + BigInt(0x8), 6);
            await sleep(release_wait);
            await memWriteI32(baseaddr + offsets.in_jump + BigInt(0x8), 4);
            sg_ctx.last_sg_finish_ms = Date.now();
            sg_ctx.start_sg = false;
        }
    }

    if (g_settings.super_grpple) {
        if (is_grpple_actived) {
            const is_grpple_attached = await memReadI32(lplayer + offsets.player_grapple + offsets.grapple_attached);
            if (is_grpple_attached === 1) {
                await memWriteI32(baseaddr + offsets.in_jump + BigInt(0x8), 5);
                await sleep(getRandomInt(16, 25));
                await memWriteI32(baseaddr + offsets.in_jump + BigInt(0x8), 4);
            }
        }
    }
}

function getRandomInt(min, max) {
    const minCeiled = Math.ceil(min);
    const maxFloored = Math.floor(max);
    // The maximum is exclusive and the minimum is inclusive
    return Math.floor(Math.random() * (maxFloored - minCeiled) + minCeiled);
}


apexsky.runtime.onMessage(async ({ name, data }) => {
    if (name === "create") {
        console.log("Auto SG load");
    } else if (name === "action_tick") {
        try {
            await action();
        } catch (e) {
            console.error("action err: ", e);
        }
    }
});