const auto g =
    autd3::gain::Group(
        [](const autd3::Device& dev,
           const autd3::Transducer& tr) -> std::optional<const char*> {
          if (tr.idx() <= 100) return "null";
          return "focus";
        })
        .set("null", autd3::gain::Null())
        .set("focus", autd3::gain::Focus(autd3::Vector3(x, y, z)));
