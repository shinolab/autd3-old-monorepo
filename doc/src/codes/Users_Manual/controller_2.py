from pyautd3.gain import Focus, Null


def grouping(dev):
    if dev.idx == 0:
        return "null"
    elif dev.idx == 1:
        return "focus"
    else:
        return None


await (
    autd.group(grouping)
    .set_data("null", Null())
    .set_data("focus", Focus([x, y, z]))
    .send_async()
)
