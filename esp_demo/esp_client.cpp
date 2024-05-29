#include <iostream>
#include <unistd.h>

#include "esp_client.hpp"

int main(int argc, char **argv) {
  // Instantiate the client. It requires a channel, out of which the actual RPCs
  // are created. This channel models a connection to an endpoint (in this case,
  // localhost at port 50051). We indicate that the channel isn't authenticated
  // (use of InsecureChannelCredentials()).
  ChannelArguments args;
  // Set the default compression algorithm for the channel.
  args.SetCompressionAlgorithm(GRPC_COMPRESS_GZIP);
  EspServiceClient esp(grpc::CreateCustomChannel(
      "localhost:50051", grpc::InsecureChannelCredentials(), args));

  // Read ESP Settings

  EspSettings espSettings = esp.GetEspSettings();

  std::cout << "Res: " << espSettings.screen_width() << "x"
            << espSettings.screen_height() << std::endl;

  std::cout << std::endl;

  // Read ESP Data

  while (true) {
    EspData espData = esp.GetEspData(false, false);

    if (!espData.ready()) {
      std::cout << "ESP service not ready!" << std::endl;
      return 0;
    }

    if (!espData.in_game() || !espData.has_local_player()) {
      std::cout << "Waiting for the game to be ready.." << std::endl;
      sleep(2);
      continue;
    }

    // Read ViewMatrix

    std::array<float, 16> viewMatrix;
    auto matrixData = espData.view_matrix();
    for (int i = 0; i < 16; ++i) {
      viewMatrix[i] = matrixData.elements(i);
    }

    std::cout << "Received ViewMatrix:" << std::endl;
    for (int i = 0; i < 4; ++i) {
      for (int j = 0; j < 4; ++j) {
        std::cout << viewMatrix[i * 4 + j] << "\t";
      }
      std::cout << std::endl;
    }
    std::cout << std::endl;

    // Read Players

    Players players = esp.GetPlayers();

    std::cout << "Players received: " << players.players_size() << std::endl;

    for (const auto &player : players.players()) {
      std::cout << "player: " << player.player_name()
                << ", HP: " << player.health()
                << ", damage: " << player.damagedealt()
                << ", kills: " << player.kills() << ", pos: "
                << "(" << player.origin().x() << "," << player.origin().y()
                << "," << player.origin().z() << ")" << std::endl;
    }

    sleep(2);
  }

  return 0;
}