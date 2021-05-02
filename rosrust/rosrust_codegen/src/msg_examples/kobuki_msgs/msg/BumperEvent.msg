# Provides a bumper event.
# This message is generated whenever a particular bumper is pressed or released.
# Note that, despite bumper field on SensorState messages, state field is not a
# bitmask, but the new state of a single sensor.

# bumper
u8 LEFT   = 0
u8 CENTER = 1
u8 RIGHT  = 2

# state
u8 RELEASED = 0
u8 PRESSED  = 1

u8 bumper
u8 state

