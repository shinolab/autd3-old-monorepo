#include "autd3/link/soem.hpp"

[[noreturn]] void on_lost(const char* msg) {
  std::cerr << "Link is lost\n" << msg << std::endl;
  exit(-1);
}

void on_err(const char* msg) { std::cerr << "Err: " << msg << std::endl; }

autd3::link::SOEM::builder()
    .with_ifname("")
    .with_buf_size(32)
    .with_on_err(&on_err)
    .with_state_check_interval(std::chrono::milliseconds(100))
    .with_on_lost(&on_lost)
    .with_sync0_cycle(2)
    .with_send_cycle(2)
    .with_timer_strategy(autd3::TimerStrategy::BusyWait)
    .with_sync_mode(autd3::link::SyncMode::DC)