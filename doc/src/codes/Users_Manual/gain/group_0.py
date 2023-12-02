from pyautd3.gain import Focus, Group, Null

g = (
    Group(lambda _, tr: "null" if tr.idx <= 100 else "focus")
    .set_gain("null", Null())
    .set_gain("focus", Focus([x, y, z]))
)
