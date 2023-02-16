#include <qi/session.hpp>
#include "qiri/include/qiri.hpp"

std::unique_ptr<Session> new_session() {
  return std::make_unique<Session>();
}

void session_listen(const std::unique_ptr<Session> &s, rust::String endpoint) {
  s->listenStandalone(qi::Url(static_cast<std::string>(std::move(endpoint))));
}

std::unique_ptr<Session> connect(rust::String endpoint) {
  auto s = std::make_unique<Session>();
  s->connect(static_cast<std::string>(std::move(endpoint)));
  return s;
}