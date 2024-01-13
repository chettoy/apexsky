#include "apex_sky.h"

global_state_t cached_state;

void load_settings() {
  __load_settings();
  cached_state = __get_global_states();
}

const settings_t global_settings() {
  cached_state = __get_global_states();
  return cached_state.settings;
}

void update_settings(settings_t state) {
  cached_state.settings = state;
  __update_global_states(cached_state);
}

void tui_menu_quit() {
  cached_state.terminal_t = false;
  __update_global_states(cached_state);
}

void tui_menu_forceupdate() {
  cached_state.tui_forceupdate = true;
  __update_global_states(cached_state);
}