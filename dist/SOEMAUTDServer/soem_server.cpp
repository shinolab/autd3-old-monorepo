// File: soem_server.cpp
// Project: SOEMAUTDServer
// Created Date: 26/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#ifdef WIN32
#include <sdkddkver.h>  // for boost/asio
#endif

#include <argparse/argparse.hpp>
#include <autd3/autd3_device.hpp>
#include <boost/asio.hpp>
#include <soem_handler.hpp>

#include "autd3/link/soem.hpp"

constexpr size_t BUF_SIZE = 65536;

[[noreturn]] static void exit_() {
#ifdef __APPLE__
  exit(0);
#else
  std::quick_exit(0);
#endif
}

class App {
 public:
  explicit App(boost::asio::io_context& io_context, const uint16_t port, const autd3::TimerStrategy timer_strategy, std::string ifname,
               const size_t buf_size, const uint16_t sync0_cycle, const uint16_t send_cycle, const autd3::link::SyncMode sync_mode,
               const int state_check_interval, std::shared_ptr<spdlog::logger> logger)
      : _acceptor(io_context, boost::asio::ip::tcp::endpoint(boost::asio::ip::tcp::v4(), port)),
        _soem_handler(
            timer_strategy, std::move(ifname), buf_size, sync0_cycle, send_cycle,
            [](const std::string& msg) {
              spdlog::error("Link is lost");
              spdlog::error(msg);
              exit_();
            },
            sync_mode, std::chrono::milliseconds(state_check_interval), std::move(logger)) {
    boost::asio::signal_set signals(io_context, SIGINT);

    signals.async_wait([this](const boost::system::error_code& error, int) {
      if (error) spdlog::error("{}", error.message());
      spdlog::info("Closing...");
      close();
      spdlog::info("Done");
      exit_();
    });
    spdlog::info("Press Ctrl+C to exit...");

    spdlog::info("Connecting SOEM server...");
    const auto dev = _soem_handler.open({});
    spdlog::info("{} AUTDs found", dev);

    std::vector<size_t> dev_map;
    dev_map.resize(dev, autd3::AUTD3::NUM_TRANS_IN_UNIT);
    _tx = autd3::driver::TxDatagram(dev_map);
    _rx = autd3::driver::RxDatagram(dev);

    spdlog::info("Waiting for client connection...");
    do_accept();

    io_context.run();
  }

 private:
  void do_accept() {
    _acceptor.async_accept([this](const boost::system::error_code ec, boost::asio::ip::tcp::socket socket) {
      if (ec)
        spdlog::error("Accept error: {}", ec.message());
      else {
        spdlog::info("Connected to client: {}", socket.remote_endpoint().address().to_string());
        _socket = std::make_shared<boost::asio::ip::tcp::socket>(std::move(socket));
        do_read();
        do_write();
        do_accept();
      }
    });
  }

  void do_read() {
    _socket->async_read_some(boost::asio::buffer(_recv_buf), [this](const boost::system::error_code ec, std::size_t) {
      if (ec == boost::asio::error::eof || ec == boost::asio::error::connection_reset || ec == boost::asio::error::connection_aborted) return;
      if (ec)
        spdlog::error("Receive error: {}", ec.message());
      else {
        std::memcpy(_tx.data().data(), _recv_buf, _tx.transmitting_size_in_bytes());

        if (_tx.header().msg_id == autd3::driver::MSG_SERVER_CLOSE) {
          spdlog::info("Disconnected from client");
          spdlog::info("Waiting for client connection...");
          _tx.clear();
          _rx.clear();
          return;
        }
        _soem_handler.send(_tx);
      }
      do_read();
    });
  }

  void do_write() {
    _soem_handler.receive(_rx);
    async_write(*_socket, boost::asio::buffer(_rx.messages().data(), _rx.messages().size() * sizeof(autd3::driver::RxMessage)),
                [this](const boost::system::error_code ec, std::size_t) {
                  if (ec == boost::asio::error::eof || ec == boost::asio::error::connection_reset || ec == boost::asio::error::connection_aborted ||
                      ec == boost::asio::error::broken_pipe)
                    return;
                  if (ec) spdlog::error("Send error: {}", ec.message());
                  do_write();
                });
  }

  void close() { _soem_handler.close(); }

  boost::asio::ip::tcp::acceptor _acceptor;
  std::shared_ptr<boost::asio::ip::tcp::socket> _socket{nullptr};

  autd3::link::SOEMHandler _soem_handler;

  uint8_t _recv_buf[BUF_SIZE]{};
  autd3::driver::TxDatagram _tx{{}};
  autd3::driver::RxDatagram _rx{0};
};

int main(const int argc, char* argv[]) try {
  argparse::ArgumentParser program("SOEMAUTDServer", "9.0.0");

  argparse::ArgumentParser list_cmd("list");
  list_cmd.add_description("List EtherCAT adapter names");

  program.add_subparser(list_cmd);

  program.add_argument("-i", "--ifname").help("Interface name").default_value(std::string(""));

  program.add_argument("-p", "--port").help("Client port").scan<'i', int>().required();

  program.add_argument("-s", "--sync0").help("Sync0 cycle time in units of 500us").scan<'i', int>().default_value(2);

  program.add_argument("-t", "--send").help("Send cycle time in units of 500us").scan<'i', int>().default_value(2);

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

  std::string ifname = program.get("--ifname");
  const auto port = static_cast<uint16_t>(program.get<int>("--port"));
  const auto sync0_cycle = std::max(1, program.get<int>("--sync0"));
  const auto send_cycle = std::max(1, program.get<int>("--send"));
  const auto state_check_interval = std::max(1, program.get<int>("--state_check_interval"));
  const std::string sync_mode_str = program.get<std::string>("--sync_mode");
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

  if (program.get<bool>("--debug")) spdlog::set_level(spdlog::level::debug);

  if (spdlog::thread_pool() == nullptr) spdlog::init_thread_pool(8192, 1);
  auto logger = std::make_shared<spdlog::async_logger>("SOEMAUTDServer Log", autd3::get_default_sink(), spdlog::thread_pool());

  boost::asio::io_context io_context;
  App app(io_context, port, timer_strategy, std::move(ifname), buf_size, static_cast<uint16_t>(sync0_cycle), static_cast<uint16_t>(send_cycle),
          sync_mode, state_check_interval, std::move(logger));

  return 0;
} catch (const std::exception& err) {
  spdlog::error(err.what());
  return -1;
}
