from datetime import timedelta

from pyautd3.link.soem import SOEM

SOEM.builder().with_timeout(timedelta(milliseconds=20))
