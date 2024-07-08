#include <chrono>
#include <cstddef>
#include <iostream>
#include <thread>

#include "Client/main.h"
#include "Client/overlay.h"
#include "esp_client.hpp"
#include "vector.h"
#include "xorstr.hpp"

bool overlay_t;
EspSettings g_esp_settings;
EspData g_esp_data;
Loots g_esp_loots;
Matrix g_view_matrix;
Vector g_esp_local_pos, g_esp_local_viewangle;
int map = 0;
std::vector<std::string> esp_spec_names, esp_teammates_damage;
size_t g_spectators = 0, g_allied_spectators = 0;
std::mutex esp_mtx;

extern void start_overlay();

int main(int argc, char **argv) {
  // Instantiate the client. It requires a channel, out of which the actual RPCs
  // are created. This channel models a connection to an endpoint (in this case,
  // localhost at port 50051). We indicate that the channel isn't authenticated
  // (use of InsecureChannelCredentials()).
  ChannelArguments args;
  // Set the default compression algorithm for the channel.
  args.SetCompressionAlgorithm(GRPC_COMPRESS_GZIP);
  EspServiceClient esp(grpc::CreateCustomChannel(
      xorstr_("localhost:50051"), grpc::InsecureChannelCredentials(), args));

  // Read ESP Settings

  g_esp_settings = esp.GetEspSettings();

  std::cout << "Res: " << g_esp_settings.screen_width() << "x"
            << g_esp_settings.screen_height() << std::endl;

  std::cout << std::endl;

  // Start Overlay
  if (!overlay_t) {
    std::thread overlay_thr = std::thread(start_overlay);
    overlay_thr.detach();
  }

  // Read ESP Data

  while (true) {
    EspData espData = esp.GetEspData(false, false);

    if (!espData.ready()) {
      std::cout << xorstr_("ESP service not ready!") << std::endl;
      overlay_t = false;
      return 0;
    }

    if (!espData.in_game() || !espData.has_local_player()) {
      // std::cout << xorstr_("Waiting for the game to be ready..") <<
      // std::endl;
      std::this_thread::sleep_for(std::chrono::seconds(2));
      std::lock_guard<std::mutex> esp_lock(esp_mtx);
      g_esp_data = espData;
      g_allied_spectators = 0;
      g_spectators = 0;
      esp_spec_names.clear();
    } else {
      std::lock_guard<std::mutex> esp_lock(esp_mtx);
      g_esp_data = espData;

      // Read ViewMatrix
      auto matrixData = espData.view_matrix();
      for (int i = 0; i < 16; ++i) {
        g_view_matrix.matrix[i] = matrixData.elements(i);
      }

      // Read local player
      PlayerState local_player = espData.local_player();
      Vec3 local_pos = local_player.origin();
      g_esp_local_pos = Vector(local_pos.x(), local_pos.y(), local_pos.z());
      Vec3 local_viewangle = local_player.view_angles();
      g_esp_local_viewangle =
          Vector(local_viewangle.x(), local_viewangle.y(), local_viewangle.z());

      // Update spectators and teammates
      SpectatorList spectators = g_esp_data.spectators();
      esp_spec_names.clear();
      g_allied_spectators = 0;
      g_spectators = 0;
      for (int i = 0; i < spectators.elements_size(); i++) {
        SpectatorInfo spectator = g_esp_data.spectators().elements(i);
        esp_spec_names.push_back(std::string(spectator.name()));
        if (spectator.is_teammate()) {
          g_allied_spectators++;
        } else {
          g_spectators++;
        }
      }
      Players teammates = g_esp_data.teammates();
      esp_teammates_damage.clear();
      for (int i = 0; i < teammates.players_size(); i++) {
        PlayerState player = teammates.players(i);
        esp_teammates_damage.push_back(std::string(player.player_name()) + " " +
                                       std::to_string(player.damagedealt()));
      }
    }

    // // Read Players

    // Players players = esp.GetPlayers();

    // std::cout << "Players received: " << players.players_size() << std::endl;

    // for (const auto &player : players.players()) {
    //   std::cout << "player: " << player.player_name()
    //             << ", HP: " << player.health()
    //             << ", damage: " << player.damagedealt()
    //             << ", kills: " << player.kills() << ", pos: "
    //             << "(" << player.origin().x() << "," << player.origin().y()
    //             << "," << player.origin().z() << ")" << std::endl;
    // }

    // std::this_thread::sleep_for(std::chrono::seconds(2));
    std::this_thread::sleep_for(std::chrono::milliseconds(2));
  }

  return 0;
}