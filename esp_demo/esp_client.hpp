
#include <google/protobuf/empty.pb.h>
#include <grpc/compression.h>
#include <grpcpp/client_context.h>
#include <iostream>
#include <memory>
#include <ostream>

#include <grpcpp/grpcpp.h>

#include "build/com/chettoy/apexsky/apexlegends/aimbot.pb.h"
#include "build/com/chettoy/apexsky/apexlegends/esp_data.pb.h"
#include "build/com/chettoy/apexsky/apexlegends/player.pb.h"
#include "build/com/chettoy/apexsky/apexlegends/spectator.pb.h"
#include "build/com/chettoy/apexsky/esp/esp_service.grpc.pb.h"
#include "build/com/chettoy/apexsky/esp/esp_service.pb.h"

using com::chettoy::apexsky::apexlegends::AimbotState;
using com::chettoy::apexsky::apexlegends::AimEntityData;
using com::chettoy::apexsky::apexlegends::AimTargetInfo;
using com::chettoy::apexsky::apexlegends::AimTargetItem;
using com::chettoy::apexsky::apexlegends::AimTargetList;
using com::chettoy::apexsky::apexlegends::Badge;
using com::chettoy::apexsky::apexlegends::EspData;
using com::chettoy::apexsky::apexlegends::EspDataOption;
using com::chettoy::apexsky::apexlegends::EspSettings;
using com::chettoy::apexsky::apexlegends::EspVisualsFlag;
using com::chettoy::apexsky::apexlegends::GradeFlag;
using com::chettoy::apexsky::apexlegends::Loots;
using com::chettoy::apexsky::apexlegends::LoveStatusCode;
using com::chettoy::apexsky::apexlegends::Matrix4x4;
using com::chettoy::apexsky::apexlegends::Players;
using com::chettoy::apexsky::apexlegends::PlayerState;
using com::chettoy::apexsky::apexlegends::SpectatorInfo;
using com::chettoy::apexsky::apexlegends::SpectatorList;
using com::chettoy::apexsky::apexlegends::TreasureClue;
using com::chettoy::apexsky::apexlegends::Vec3;
using com::chettoy::apexsky::esp::EspService;
using com::chettoy::apexsky::esp::GetPlayersRequest;

using grpc::Channel;
using grpc::ChannelArguments;
using grpc::ClientContext;
using grpc::Status;

class EspServiceClient {
public:
  EspServiceClient(std::shared_ptr<Channel> channel)
      : stub_(EspService::NewStub(channel)) {}

  // Assembles the client's payload, sends it and presents the response back
  // from the server.
  Players GetPlayers() {
    // Data we are sending to the server.
    GetPlayersRequest request;
    request.set_version(0);

    // Container for the data we expect from the server.
    Players reply;

    // Context for the client. It could be used to convey extra information to
    // the server and/or tweak certain RPC behaviors.
    ClientContext context;

    // // Overwrite the call's compression algorithm to DEFLATE.
    // context.set_compression_algorithm(GRPC_COMPRESS_DEFLATE);

    // The actual RPC.
    Status status = stub_->GetPlayers(&context, request, &reply);

    // Act upon its status.
    if (!status.ok()) {
      std::cout << status.error_code() << ": " << status.error_message()
                << std::endl;
    }
    return reply;
  }

  EspData GetEspData(bool full_aimbot_state, bool full_targets_list) {
    EspDataOption request;
    request.set_version(0);
    request.set_full_aimbot_state(full_aimbot_state);
    request.set_full_targets_list(full_targets_list);
    EspData reply;
    ClientContext context;
    Status status = stub_->GetEspData(&context, request, &reply);
    if (!status.ok()) {
      std::cout << status.error_code() << ": " << status.error_message()
                << std::endl;
    }
    return reply;
  }

  EspSettings GetEspSettings() {
    google::protobuf::Empty request;
    EspSettings reply;
    ClientContext context;
    Status status = stub_->GetEspSettings(&context, request, &reply);
    if (!status.ok()) {
      std::cout << status.error_code() << ": " << status.error_message()
                << std::endl;
    }
    return reply;
  }

private:
  std::unique_ptr<EspService::Stub> stub_;
};