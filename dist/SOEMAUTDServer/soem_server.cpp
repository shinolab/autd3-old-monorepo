// File: soem_server.cpp
// Project: SOEMAUTDServer
// Created Date: 26/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <argparse/argparse.hpp>
#include <soem_handler.hpp>

#include "autd3/link/soem.hpp"
#include "local_interface.hpp"
#include "tcp_interface.hpp"

int main(const int argc, char* argv[]) try {
  argparse::ArgumentParser program("SOEMAUTDServer", "2.5.0");

  argparse::ArgumentParser list_cmd("list");
  list_cmd.add_description("List EtherCAT adapter names");

  program.add_subparser(list_cmd);

  program.add_argument("-i", "--ifname").help("Interface name").default_value(std::string(""));

  program.add_argument("-c", "--client").help("Client IP address").default_value(std::string(""));

  program.add_argument("-s", "--sync0").help("Sync0 cycle time in units of 500us").scan<'i', int>().default_value(2);

  program.add_argument("-t", "--send").help("Send cycle time in units of 500us").scan<'i', int>().default_value(2);

  program.add_argument("-f", "--freerun").help("Set free run mode").implicit_value(true).default_value(false);

  program.add_argument("-l", "--disable_high_precision").help("Disable high precision mode").implicit_value(true).default_value(false);

  program.add_argument("-e", "--state_check_interval").help("State check interval in ms").scan<'i', int>().default_value(500);

  program.add_argument("-d", "--debug").help("Set debug mode").implicit_value(true).default_value(false);

  try {
    program.parse_args(argc, argv);
  } catch (const std::runtime_error& err) {
    spdlog::error(err.what());
    std::stringstream ss;
    ss << program;
    spdlog::error(ss.str());
    return -1;
  }

  if (program.is_subcommand_used("list")) {
    const auto adapters = autd3::link::SOEMHandler::enumerate_adapters();
    std::cout << "Available adapters are ..." << std::endl;
    for (const auto& adapter : adapters) std::cout << "\t" << adapter.desc << ": " << adapter.name << std::endl;
    return 0;
  }

  const auto ifname = program.get("--ifname");
  const auto client = program.get("--client");
  const auto sync0_cycle = std::max(1, program.get<int>("--sync0"));
  const auto send_cycle = std::max(1, program.get<int>("--send"));
  const auto state_check_interval = std::max(1, program.get<int>("--state_check_interval"));
  const auto freerun = program.get<bool>("--freerun");
  const auto disable_high_precision = program.get<bool>("--disable_high_precision");

  if (program.get<bool>("--debug")) spdlog::set_level(spdlog::level::debug);

  const auto local_connection = client.empty() || client == "127.0.0.1" || client == "localhost";

  auto soem_handler = autd3::link::SOEMHandler(
      !disable_high_precision, ifname, static_cast<uint16_t>(sync0_cycle), static_cast<uint16_t>(send_cycle),
      [](const std::string& msg) {
        spdlog::error("Link is lost");
        spdlog::error(msg);
#ifdef __APPLE__
        exit(-1);
#else
        std::quick_exit(-1);
#endif
      },
      freerun ? autd3::link::SYNC_MODE::FREE_RUN : autd3::link::SYNC_MODE::DC, std::chrono::milliseconds(state_check_interval));

  spdlog::info("Connecting SOEM server...");
  const auto dev = soem_handler.open(1);
  spdlog::info("{} AUTDs found", dev);

  std::unique_ptr<autd3::publish::Interface> interf;

  if (local_connection)
    interf = std::make_unique<autd3::publish::LocalInterface>(dev);
  else
    interf = std::make_unique<autd3::publish::TcpInterface>();

  bool run = true;
  auto th = std::thread([&soem_handler, &run, dev, &interf] {
    autd3::driver::TxDatagram tx(dev);
    autd3::driver::RxDatagram rx(dev);
    interf->connect();
    while (run) {
      if (interf->tx(tx)) soem_handler.send(tx);
      soem_handler.receive(rx);
      interf->rx(rx);
    }
    interf->close();
  });

  spdlog::info("enter any key to quit...");
  std::cin.ignore();

  run = false;
  if (th.joinable()) th.join();

  soem_handler.close();

  return 0;
} catch (const std::runtime_error& err) {
  spdlog::error(err.what());
  return -1;
}
