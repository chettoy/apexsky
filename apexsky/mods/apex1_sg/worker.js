console.log("Auto SG load");

let sgState = {
  startSg: false,
  lastSgFinishMs: 0,
};

/**
 * @param {GameApi} game
 * @param {SkyMemAccess} mem
 * @param {Object} offsets
 * @param {bigint} tickNum
 * @returns {Promise<void>}
 * @throws {Error}
 */
async function action(game, mem, offsets, tickNum) {
  // console.log("tick");

  if (!(await game.isWorldReady())) {
    // console.log("not world ready");
    return;
  }

  // console.log("get info");
  const [baseaddr, g_settings, lplayer] = await Promise.all([
    mem.getBaseAddr(),
    game.getGlobalSettings(),
    game.getLocalPlayerPtr(),
  ]);

  // console.log("get local player")
  const localPlayer = await game.getPlayerData(lplayer);
  if (!localPlayer || baseaddr === 0n) {
    return;
  }

  // console.log("read");
  const [jumpState, worldTime, traversalStartTime, isGrappleActived] =
    await Promise.all([
      mem.readI32(baseaddr + offsets.in_jump),
      mem.readF32(lplayer + offsets.cplayer_timebase),
      mem.readF32(lplayer + offsets.cplayer_traversal_starttime),
      mem.readI32(lplayer + offsets.player_grapple_active),
    ]);

  if (g_settings.feature_settings.auto_super_glide) {
    /**
     * SuperGlide
     * https://www.unknowncheats.me/forum/apex-legends/578160-external-auto-superglide-3.html
     */

    const propsFor75Fps = {
      propHangStart: 0.1,
      propHangCancel: 0.12,
      propHangMax: 1.5,
      propTravStart: 0.87,
      propActionInterval: 11,
      propReleaseWait: 50,
    };
    const propsFor144Fps = {
      propHangStart: 0.05,
      propHangCancel: 0.07,
      propHangMax: 0.75,
      propTravStart: 0.9,
      propActionInterval: 7,
      propReleaseWait: 25,
    };
    const propsFor240Fps = {
      propHangStart: 0.033,
      propHangCancel: 0.04,
      propHangMax: 0.2,
      propTravStart: 0.95,
      propActionInterval: 4,
      propReleaseWait: 20,
    };

    let props = {
      propHangStart: 0.1,
      propHangCancel: 0.12,
      propHangMax: 1.5,
      propTravStart: 0.87,
      propActionInterval: 11,
      propReleaseWait: 50,
    };
    // const gameFps = g_settings.game_fps;
    // if (Math.abs(gameFps - 75.0) < Math.abs(gameFps - 144.0)) {
    //   props = propsFor75Fps;
    // } else if (Math.abs(gameFps - 144.0) < Math.abs(gameFps - 240.0)) {
    //   props = propsFor144Fps;
    // } else {
    //   props = propsFor240Fps;
    // }
    const {
      propHangStart,
      propHangCancel,
      propHangMax,
      propTravStart,
      propActionInterval,
      propReleaseWait,
    } = props;

    let hangOnWallTime = worldTime - traversalStartTime;
    let pressJump = 0;

    // When hanging on the wall, loop and wait for the time to start SG
    const loopStartTimeMs = Date.now();
    while (!sgState.startSg && hangOnWallTime > 0.0) {
      // Re-read the required data
      const [currJump, currFrame, currWorldTime, currTravStartTime, currTravProgress] =
        await Promise.all([
          mem.readI32(baseaddr + offsets.in_jump, SkyMemAccess.PRIO_HIGH),
          mem.readI32(baseaddr + offsets.global_vars + 8n, SkyMemAccess.PRIO_HIGH),
          mem.readF32(lplayer + offsets.cplayer_timebase, SkyMemAccess.PRIO_HIGH),
          mem.readF32(lplayer + offsets.cplayer_traversal_starttime, SkyMemAccess.PRIO_HIGH),
          mem.readF32(lplayer + offsets.cplayer_traversal_progress, SkyMemAccess.PRIO_HIGH),
        ]);

      hangOnWallTime = currWorldTime - currTravStartTime;

      const nowInMs = Date.now();

      if (
        currTravProgress < propTravStart &&
        hangOnWallTime > propHangStart &&
        hangOnWallTime < propHangCancel
      ) {
        await mem.writeI32(
          baseaddr + offsets.in_jump + 0x8n,
          4,
          SkyMemAccess.PRIO_PREEMPT,
        );
      } else if (
        currTravProgress > propTravStart &&
        hangOnWallTime > propHangStart &&
        hangOnWallTime < propHangMax
      ) {
        if (nowInMs - sgState.lastSgFinishMs > 100 && jumpState > 0) {
          // start SG
          sgState.startSg = true;
          pressJump = currJump;
        }
        break;
      }

      // Preventing dead loops caused by erroneous data
      if (nowInMs > loopStartTimeMs + 2000) {
        break;
      }
    }
    if (sgState.startSg) {
      // press jump button
      await mem.writeI32(
        baseaddr + offsets.in_jump + 0x8n,
        5,
        SkyMemAccess.PRIO_PREEMPT,
      );
      await sleep(propActionInterval);
      // press duck button
      await mem.writeI32(
        baseaddr + offsets.in_duck + 0x8n,
        6,
        SkyMemAccess.PRIO_PREEMPT,
      );
      await sleep(propReleaseWait);
      // recover jump button
      if (pressJump == 0) {
        await mem.writeI32(baseaddr + offsets.in_jump + 0x8n, 4);
      }
      // release duck button
      await mem.writeI32(baseaddr + offsets.in_duck + 0x8n, 4);
      sgState.lastSgFinishMs = Date.now();
      sgState.startSg = false;
    }
  }

  if (g_settings.feature_settings.auto_super_grapple) {
    /**
     * Super Grapple
     */
    if (isGrappleActived) {
      const isGrappleAttached = await mem.readI32(
        lplayer + offsets.player_grapple + offsets.grapple_attached,
      );
      if (isGrappleAttached === 1) {
        await mem.writeI32(baseaddr + offsets.in_jump + 0x8n, 5);
        await sleep(getRandomInt(16, 25));
        await mem.writeI32(baseaddr + offsets.in_jump + 0x8n, 4);
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

// GameApi.debugRpc();

new GameApi().waitMainMod().then(async (game) => {
  console.log("Auto SG ready");

  const offsets = await game.getGameOffsets();
  const settings = await game.getGlobalSettings();

  const mem = new SkyMemAccess({
    target_process_name: (settings.game_ver_dx11 ? "r5apex.exe" : "r5apex_dx12.exe"),
    override_module_base: null,
    check_time_date_stamp: offsets.time_date_stamp,
    speed_test: true,
    cache_phys_addr: true,
  });

  const actionTick = new SkyValueWatcher("apex1.action_tick", "AUTOSG01H8RG1ZNGRPS4H54AEW");
  try {
    while (true) {
      const data = await actionTick.nextRaw();
      const tickNum = new DataView(data.buffer).getBigInt64(0, true);
      // console.log("tickNum=" + tickNum);
      // await sleep(200);

      // try {
      await action(game, mem, offsets, tickNum);
      // } catch (err) {
      //   // Do not print memory read/write failure error messages
      //   if (err.message !== "fail") {
      //     console.error(err.name + ": " + err.message);
      //   }
      // }
    }
  } finally {
    actionTick.close();
  }
});
