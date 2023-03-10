#include <cstdint>
#include <memory>
#include <qi/session.hpp>
#include "rust/cxx.h"

using Session = qi::Session;

std::unique_ptr<Session> new_session();
void session_listen(const std::unique_ptr<Session> &s, rust::String endpoint);
void session_connect(const std::unique_ptr<Session> &s, rust::String endpoint);