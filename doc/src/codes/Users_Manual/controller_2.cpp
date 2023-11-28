autd.group([](const autd3::Device& dev) -> std::optional<const char*> {
      if (dev.idx() == 0) {
        return "null";
      } else if (dev.idx() == 1) {
        return "focus";
      } else {
        return std::nullopt;
      }
    })
    .set("null", autd3::gain::Null())
    .set("focus", autd3::gain::Focus(autd3::Vector3(x, y, z)))
    .send_async()
    .get();