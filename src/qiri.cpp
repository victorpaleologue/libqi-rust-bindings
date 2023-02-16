#include <qi/session.hpp>
#include "qiri/include/qiri.hpp"

std::unique_ptr<Session> new_session() {
  return std::make_unique<Session>();
}

void session_listen(const std::unique_ptr<Session> &s, rust::String endpoint) {
  s->listenStandalone(qi::Url(static_cast<std::string>(std::move(endpoint))));
}

void session_connect(const std::unique_ptr<Session> &s, rust::String endpoint) {
  s->connect(qi::Url(static_cast<std::string>(std::move(endpoint))));
}
