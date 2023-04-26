// File: soem_server.cpp
// Project: SOEMAUTDServer
// Created Date: 26/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#ifdef WIN32
#include <sdkddkver.h>
#endif

#include <argparse/argparse.hpp>
#include <boost/asio.hpp>
#include <soem_handler.hpp>

#include "autd3/link/soem.hpp"
#include "local_interface.hpp"
#include "tcp_interface.hpp"

static void exit_() {
#ifdef __APPLE__
  exit(0);
#else
  std::quick_exit(0);
#endif
}

class App {
 public:
  explicit App(std::string client, const uint16_t port, const autd3::TimerStrategy timer_strategy, std::string ifname, const size_t buf_size,
               const uint16_t sync0_cycle, const uint16_t send_cycle, const autd3::link::SyncMode sync_mode, const int state_check_interval,
               std::shared_ptr<spdlog::logger> logger)
      : _client(std::move(client)),
        _port(port),
        _soem_handler(
            timer_strategy, std::move(ifname), buf_size, sync0_cycle, send_cycle,
            [](const std::string& msg) {
              spdlog::error("Link is lost");
              spdlog::error(msg);
              exit_();
            },
            sync_mode, std::chrono::milliseconds(state_check_interval), std::move(logger)) {}

  void run() {
    boost::asio::signal_set signals(_io_service, SIGINT);

    signals.async_wait([this](const boost::system::error_code& error, int) {
      if (error) spdlog::error("{}", error.message());
      spdlog::info("Closing...");
      close();
      spdlog::info("Done");
      exit_();
    });
    spdlog::info("Press Ctrl+C to exit...");

    std::thread([this] {
      try {
        run_();
      } catch (const std::exception& ex) {
        spdlog::error("{}", ex.what());
        exit_();
      }
    }).detach();

    _io_service.run();
  }

 private:
  void close() {
    _is_running.store(false);
    if (_interf != nullptr) _interf->close();
    _soem_handler.close();
  }

  void run_() {
    spdlog::info("Connecting SOEM server...");
    const auto dev = _soem_handler.open({});
    spdlog::info("{} AUTDs found", dev);

    if (_client.empty() || _client == "127.0.0.1" || _client == "localhost")
      _interf = std::make_unique<autd3::publish::LocalInterface>(dev);
    else
      _interf = std::make_unique<autd3::publish::TcpInterface>(_client, _port, dev);

    _is_running.store(true);
    try {
      std::vector<size_t> dev_map;
      dev_map.resize(dev, autd3::AUTD3::NUM_TRANS_IN_UNIT);
      autd3::driver::TxDatagram tx(dev_map);
      autd3::driver::RxDatagram rx(dev);
      _interf->connect();
      while (_is_running.load()) {
        if (_interf->tx(tx)) _soem_handler.send(tx);
        if (_soem_handler.receive(rx)) _interf->rx(rx);
        if (tx.header().msg_id == autd3::driver::MSG_SERVER_CLOSE) {
          tx.clear();
          rx.clear();
        }
      }
    } catch (std::exception& ex) {
      spdlog::error("{}", ex.what());
    }
  }

  std::string _client;
  uint16_t _port;

  autd3::link::SOEMHandler _soem_handler;
  std::unique_ptr<autd3::publish::Interface> _interf{nullptr};

  std::atomic<bool> _is_running;

  boost::asio::io_service _io_service;
};

int main(const int argc, char* argv[]) try {
  argparse::ArgumentParser program("SOEMAUTDServer", "8.5.0");

  argparse::ArgumentParser list_cmd("list");
  list_cmd.add_description("List EtherCAT adapter names");

  program.add_subparser(list_cmd);

  program.add_argument("-i", "--ifname").help("Interface name").default_value(std::string(""));

  program.add_argument("-c", "--client").help("Client IP address").default_value(std::string(""));

  program.add_argument("-p", "--port").help("Client port").scan<'i', int>().default_value(50632);

  program.add_argument("-s", "--sync0").help("Sync0 cycle time in units of 500us").scan<'i', int>().default_value(2);

  program.add_argument("-t", "--send").help("Send cycle time in units of 500us").scan<'i', int>().default_value(2);

  program.add_argument("-f", "--freerun")
      .help("Set free run mode (Deprecated, use --sync_mode option instead)")
      .implicit_value(true)
      .default_value(false);

  program.add_argument("-m", "--sync_mode")
      .help(R"(Sync mode ("dc", "freerun"))")
      .default_value(std::string{"freerun"})
      .action([](const std::string& value) {
        if (static const std::vector<std::string> CHOICES = {"dc", "freerun"}; std::find(CHOICES.begin(), CHOICES.end(), value) != CHOICES.end()) {
          return value;
        }
        spdlog::warn("{} is invalid. Using \"freerun\" instead.", value);
        return std::string{"freerun"};
      });

  program.add_argument("-l", "--disable_high_precision")
      .help("Disable high precision mode (Deprecated, use --timer_strategy option instead)")
      .implicit_value(true)
      .default_value(false);

  program.add_argument("-b", "--buffer_size").help("Buffer size (unlimited if 0)").scan<'i', int>().default_value(0);

  program.add_argument("-w", "--timer_strategy")
      .help(R"(Timer Strategy ("sleep", "busywait", "timer"))")
      .default_value(std::string{"sleep"})
      .action([](const std::string& value) {
        if (static const std::vector<std::string> CHOICES = {"sleep", "busywait", "timer"};
            std::find(CHOICES.begin(), CHOICES.end(), value) != CHOICES.end()) {
          return value;
        }
        spdlog::warn("{} is invalid. Using \"sleep\" instead.", value);
        return std::string{"sleep"};
      });

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
    std::transform(adapters.begin(), adapters.end(), std::ostream_iterator<std::string>(std::cout, "\n"),
                   [](const auto& adapter) { return "\t" + adapter.desc + ": " + adapter.name; });
    return 0;
  }

  auto ifname = program.get("--ifname");
  auto client = program.get("--client");
  const auto port = static_cast<uint16_t>(program.get<int>("--port"));
  const auto sync0_cycle = std::max(1, program.get<int>("--sync0"));
  const auto send_cycle = std::max(1, program.get<int>("--send"));
  const auto state_check_interval = std::max(1, program.get<int>("--state_check_interval"));
  const auto freerun = program.get<bool>("--freerun");
  const std::string sync_mode_str = program.get<std::string>("--sync_mode");
  const auto disable_high_precision = program.get<bool>("--disable_high_precision");
  const auto buf_size = program.get<int>("--buffer_size");
  const std::string timer_strategy_str = program.get<std::string>("--timer_strategy");
  autd3::core::TimerStrategy timer_strategy;
  if (timer_strategy_str == "busywait")
    timer_strategy = autd3::core::TimerStrategy::BusyWait;
  else if (timer_strategy_str == "timer")
    timer_strategy = autd3::core::TimerStrategy::NativeTimer;
  else
    timer_strategy = autd3::core::TimerStrategy::Sleep;

  autd3::link::SyncMode sync_mode;
  if (sync_mode_str == "dc")
    sync_mode = autd3::SyncMode::DC;
  else
    sync_mode = autd3::SyncMode::FreeRun;

  if (disable_high_precision) spdlog::warn("Please use timer_strategy option instead.");
  if (freerun) spdlog::warn("Please use --sync_mode option instead.");

  if (program.get<bool>("--debug")) spdlog::set_level(spdlog::level::debug);

  if (spdlog::thread_pool() == nullptr) spdlog::init_thread_pool(8192, 1);
  auto logger = std::make_shared<spdlog::async_logger>("SOEMAUTDServer Log", autd3::get_default_sink(), spdlog::thread_pool());

  App app(std::move(client), port, timer_strategy, std::move(ifname), buf_size, static_cast<uint16_t>(sync0_cycle), static_cast<uint16_t>(send_cycle),
          sync_mode, state_check_interval, std::move(logger));

  app.run();

  return 0;
} catch (const std::exception& err) {
  spdlog::error(err.what());
  return -1;
}
